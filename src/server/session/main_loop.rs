/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/

use std::panic::RefUnwindSafe;

use super::Session;
use crate::server::session::notification_registry::NotificationRegistry;
use crate::server::session::request_registry::RequestRegistry;
use anyhow::Error;
use auto_lsp_core::salsa::db::BaseDatabase;
use crossbeam_channel::select;
use lsp_server::Message;

#[derive(Debug)]
pub(crate) enum Task {
    Response(lsp_server::Response),
    NotificationError(Error),
}

impl<Db: BaseDatabase + Clone + Send + RefUnwindSafe> Session<Db> {
    /// Main loop of the LSP server, backed by [`lsp-server`] and [`crossbeam-channel`] crates.
    pub fn main_loop<'a>(
        &'a mut self,
        req_registry: &'a RequestRegistry<Db>,
        not_registry: &'a NotificationRegistry<Db>,
    ) -> anyhow::Result<()> {
        loop {
            select! {
                recv(self.connection.receiver) -> msg => {
                    match msg? {
                        Message::Request(req) => {
                            if self.connection.handle_shutdown(&req)? {
                                return Ok(());
                            };

                            self.req_queue.incoming.register(req.id.clone(), req.method.clone());

                            let id = req.id.clone();
                            if let Some(response) = req_registry.handle(self, req.clone())? {
                                 if !self.req_queue.incoming.is_completed(&id) {
                                    self.req_queue.incoming.complete(&id);
                                    self.connection.sender.send(Message::Response(response))?;
                                }
                            }
                        }
                        Message::Notification(not) => {
                            if let Some(method) = not_registry.get(&not) {
                                NotificationRegistry::exec(self, method, not);
                            } else if let Some(method) = not_registry.get_sync_mut(&not) {
                                NotificationRegistry::exec_sync_mut(self, method, not)?;
                            } else {
                                NotificationRegistry::handle_error(self, anyhow::format_err!("Unknown notification: {}", not.method))?
                            }
                        }
                        Message::Response(_) => {}
                    }
                },
                recv(self.task_rx) -> task => {
                    match task? {
                        Task::Response(resp) => {
                            if !self.req_queue.incoming.is_completed(&resp.id) {
                                self.req_queue.incoming.complete(&resp.id);
                                self.connection.sender.send(Message::Response(resp))?;
                            }
                        },
                        Task::NotificationError(err) => NotificationRegistry::handle_error(self, err)?,
                    }
                }
            }
        }
    }

    /// Send a notification to the client.
    pub fn send_notification<N: lsp_types::notification::Notification>(
        &self,
        params: N::Params,
    ) -> anyhow::Result<()> {
        let params = serde_json::to_value(&params)?;
        let n = lsp_server::Notification {
            method: N::METHOD.into(),
            params,
        };
        self.connection.sender.send(Message::Notification(n))?;
        Ok(())
    }
}
