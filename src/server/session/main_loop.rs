use super::Session;
use crate::server::session::notification_registry::NotificationRegistry;
use crate::server::session::request_registry::RequestRegistry;
use auto_lsp_core::salsa::db::BaseDatabase;
use crossbeam_channel::select;
use lsp_server::{Message, Notification};
use parking_lot::Mutex;

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
