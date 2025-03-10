use crossbeam_channel::select;
use lsp_server::Message;
use lsp_server::{ExtractError, Notification};
use lsp_types::notification::{DidChangeTextDocument, DidChangeWatchedFiles, DidOpenTextDocument};

use crate::server::session::REQUEST_REGISTRY;

use super::Session;

impl Session {
    /// Main loop of the LSP server, backed by [`lsp-server`] and [`crossbeam-channel`] crates.
    pub fn main_loop(&mut self) -> anyhow::Result<()> {
        loop {
            select! {
                recv(self.connection.receiver) -> msg => {
                    match msg? {
                        Message::Request(req) => {
                            if self.connection.handle_shutdown(&req)? {
                                return Ok(());
                            };

                            if let Some(response) = REQUEST_REGISTRY.lock().handle(self, req)? {
                                self.connection.sender.send(Message::Response(response))?;
                            }
                        }
                        Message::Notification(not) => {
                            NotificationDispatcher::new(self, not)
                                .on::<DidOpenTextDocument>(Self::open_text_document)?
                                .on::<DidChangeTextDocument>(Self::edit_text_document)?
                                .on::<DidChangeWatchedFiles>(Self::changed_watched_files)?;
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
        let params = serde_json::to_value(&params).unwrap();
        let n = lsp_server::Notification {
            method: N::METHOD.into(),
            params,
        };
        self.connection.sender.send(Message::Notification(n))?;
        Ok(())
    }
}

pub struct NotificationDispatcher<'a> {
    session: &'a mut Session,
    not: Option<Notification>,
}

impl<'a> NotificationDispatcher<'a> {
    pub fn new(session: &'a mut Session, not: Notification) -> Self {
        NotificationDispatcher {
            session,
            not: Some(not),
        }
    }

    pub fn on<N>(
        &'a mut self,
        hook: impl Fn(&mut Session, N::Params) -> anyhow::Result<()>,
    ) -> anyhow::Result<&'a mut Self>
    where
        N: lsp_types::notification::Notification,
        N::Params: serde::de::DeserializeOwned,
    {
        let not = match self.not.take() {
            Some(r) => r,
            None => return Ok(self),
        };

        match not.extract::<N::Params>(N::METHOD) {
            Ok(params) => {
                hook(self.session, params)?;
                Ok(self)
            }
            Err(err @ ExtractError::JsonError { .. }) => Err(anyhow::Error::from(err)),
            Err(ExtractError::MethodMismatch(not)) => {
                self.not = Some(not);
                Ok(self)
            }
        }
    }
}
