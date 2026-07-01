use std::panic::RefUnwindSafe;

use crate::{notification_registry::NotificationRegistry, request_registry::RequestRegistry};

use super::Session;
use anyhow::Error;
use crossbeam_channel::select;
use lsp_server::Message;

#[derive(Debug)]
pub enum Task {
    Response(lsp_server::Response),
    NotificationError(Error),
}

impl<Db: salsa::Database + Clone + Send + RefUnwindSafe> Session<Db> {
    /// Main loop of the LSP server, backed by [`lsp-server`] and [`crossbeam-channel`] crates.
    pub fn main_loop(
        mut self,
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

                            self.req_queue.incoming.register(req.id.clone(), req.method.clone());

                            if let Some(method) = req_registry.get(&req) {
                                RequestRegistry::exec(&self, method, req);
                            } else if let Some(method) = req_registry.get_sync_mut(&req) {
                                RequestRegistry::exec_sync_mut(&mut self, method, req)?;
                            } else {
                                RequestRegistry::complete(&mut self,
                                    RequestRegistry::<Db>::request_mismatch(req.id.clone(), anyhow::format_err!("Unknown request: {}", req.method))
                                )?
                            }
                        }
                        Message::Notification(not) => {
                            if let Some(method) = not_registry.get(&not) {
                                NotificationRegistry::exec(&self, method, not);
                            } else if let Some(method) = not_registry.get_sync_mut(&not) {
                                NotificationRegistry::exec_sync_mut(&mut self, method, not)?;
                            } else {
                                NotificationRegistry::handle_error(&self, anyhow::format_err!("Unknown notification: {}", not.method))?
                            }
                        }
                        Message::Response(_) => {}
                    }
                },
                recv(self.task_receiver) -> task => {
                    match task? {
                        Task::Response(resp) => RequestRegistry::complete(&mut self, resp)?,
                        Task::NotificationError(err) => NotificationRegistry::handle_error(&self, err)?,
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
