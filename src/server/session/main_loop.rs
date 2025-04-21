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

use super::Session;
use crate::server::session::notification_registry::NotificationRegistry;
use crate::server::session::request_registry::RequestRegistry;
use auto_lsp_core::salsa::db::BaseDatabase;
use crossbeam_channel::select;
use lsp_server::{Message, Notification};

impl<Db: BaseDatabase> Session<Db> {
    /// Main loop of the LSP server, backed by [`lsp-server`] and [`crossbeam-channel`] crates.
    pub fn main_loop(
        &mut self,
        req_registry: &RequestRegistry<Db>,
        not_registry: &NotificationRegistry<Db>,
    ) -> anyhow::Result<()> {
        loop {
            select! {
                recv(self.connection.receiver) -> msg => {
                    match msg? {
                        Message::Request(req) => {
                            if self.connection.handle_shutdown(&req)? {
                                return Ok(());
                            };

                            self.req_queue.incoming.on(req.id.clone(), req.method.clone());

                            let id = req.id.clone();
                            if let Some(response) = req_registry.handle(self, req.clone())? {
                                 if !self.req_queue.incoming.is_completed(&id) {
                                    self.req_queue.incoming.complete(&id);
                                    self.connection.sender.send(Message::Response(response))?;
                                }
                            }
                        }
                        Message::Notification(not) => {
                            if let Err(err) = not_registry.handle(self, not) {
                                self.connection.sender.send(Message::Notification(Notification {
                                    method: "window/showMessage".to_string(),
                                    params: serde_json::json!({
                                        "type": lsp_types::MessageType::ERROR,
                                        "message": err.to_string(),
                                    })}
                                ))?;
                                log::error!("Error handling notification: {}", err.to_string());
                            }
                        }
                        Message::Response(_) => {}
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
