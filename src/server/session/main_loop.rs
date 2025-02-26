use crossbeam_channel::select;
use lsp_server::Message;
use lsp_server::{ExtractError, Notification, Request, Response};
use lsp_types::{
    notification::{DidChangeTextDocument, DidChangeWatchedFiles, DidOpenTextDocument},
    request::{
        CodeActionRequest, CodeLensRequest, Completion, DocumentDiagnosticRequest,
        DocumentLinkRequest, DocumentSymbolRequest, FoldingRangeRequest, GotoDeclaration,
        GotoDefinition, HoverRequest, InlayHintRequest, References, SelectionRangeRequest,
        SemanticTokensFullRequest, SemanticTokensRangeRequest, WorkspaceDiagnosticRequest,
        WorkspaceSymbolRequest,
    },
};
use serde::Serialize;

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
                            RequestDispatcher::new(self, req)
                                .on::<DocumentDiagnosticRequest, _>(Self::get_diagnostics)?
                                .on::<DocumentLinkRequest, _>(Self::get_document_links)?
                                .on::<DocumentSymbolRequest, _>(Self::get_document_symbols)?
                                .on::<FoldingRangeRequest, _>(Self::get_folding_ranges)?
                                .on::<HoverRequest, _>(Self::get_hover)?
                                .on::<SemanticTokensFullRequest, _>(Self::get_semantic_tokens_full)?
                                .on::<SemanticTokensRangeRequest, _>(Self::get_semantic_tokens_range)?
                                .on::<SelectionRangeRequest, _>(Self::get_selection_ranges)?
                                .on::<WorkspaceSymbolRequest, _>(Self::get_workspace_symbols)?
                                .on::<WorkspaceDiagnosticRequest, _>(Self::get_workspace_diagnostics)?
                                .on::<InlayHintRequest, _>(Self::get_inlay_hints)?
                                .on::<CodeActionRequest, _>(Self::get_code_actions)?
                                .on::<CodeLensRequest, _>(Self::get_code_lenses)?
                                .on::<Completion, _>(Self::get_completion_items)?
                                .on::<GotoDefinition, _>(Self::go_to_definition)?
                                .on::<GotoDeclaration, _>(Self::go_to_declaration)?
                                .on::<References, _>(Self::get_references)?;
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

/// Code taken from <https://github.com/oxlip-lang/oal/blob/b6741ff99f7c9338551e2067c0de7acd492fad00/oal-client/src/lsp/dispatcher.rs>
pub struct RequestDispatcher<'a> {
    session: &'a mut Session,
    req: Option<Request>,
}

impl<'a> RequestDispatcher<'a> {
    pub fn new(session: &'a mut Session, req: Request) -> Self {
        RequestDispatcher {
            session,
            req: Some(req),
        }
    }

    pub fn on<R, T>(
        &'a mut self,
        hook: impl Fn(&mut Session, R::Params) -> anyhow::Result<T>,
    ) -> anyhow::Result<&'a mut Self>
    where
        R: lsp_types::request::Request,
        R::Params: serde::de::DeserializeOwned,
        T: Serialize,
    {
        let req = match self.req.take() {
            Some(r) => r,
            None => return Ok(self),
        };

        match req.extract::<R::Params>(R::METHOD) {
            Ok((id, params)) => {
                let resp = Response {
                    id,
                    result: Some(serde_json::to_value(hook(self.session, params)?).unwrap()),
                    error: None,
                };
                self.session
                    .connection
                    .sender
                    .send(Message::Response(resp))?;
                Ok(self)
            }
            Err(err @ ExtractError::JsonError { .. }) => Err(anyhow::Error::from(err)),
            Err(ExtractError::MethodMismatch(req)) => {
                self.req = Some(req);
                Ok(self)
            }
        }
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
