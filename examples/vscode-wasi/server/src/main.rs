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

use ast_python::capabilities::code_actions::code_actions;
use ast_python::capabilities::code_lenses::code_lenses;
use ast_python::capabilities::completion_items::completion_items;
use ast_python::capabilities::diagnostics::diagnostics;
use ast_python::capabilities::document_symbols::document_symbols;
use ast_python::capabilities::folding_ranges::folding_ranges;
use ast_python::capabilities::hover::hover;
use ast_python::capabilities::inlay_hints::inlay_hints;
use ast_python::capabilities::selection_ranges::selection_ranges;
use ast_python::capabilities::semantic_tokens::{
    semantic_tokens_full, SUPPORTED_MODIFIERS, SUPPORTED_TYPES,
};
use ast_python::capabilities::workspace_diagnostics::workspace_diagnostics;
use ast_python::capabilities::workspace_symbols::workspace_symbols;
use ast_python::db::PYTHON_PARSERS;
use auto_lsp::anyhow;
use auto_lsp::default::db::{BaseDatabase, BaseDb};
use auto_lsp::default::server::capabilities::{
    semantic_tokens_provider, TEXT_DOCUMENT_SYNC, WORKSPACE_PROVIDER,
};
use auto_lsp::default::server::file_events::{changed_watched_files, open_text_document};
use auto_lsp::default::server::workspace_init::WorkspaceInit;
use auto_lsp::lsp_server::{self, Connection};
use auto_lsp::lsp_types::notification::{
    Cancel, DidChangeTextDocument, DidChangeWatchedFiles, DidCloseTextDocument,
    DidOpenTextDocument, DidSaveTextDocument, LogTrace, SetTrace,
};
use auto_lsp::lsp_types::request::{
    CodeActionRequest, CodeLensRequest, Completion, DocumentDiagnosticRequest,
    DocumentSymbolRequest, FoldingRangeRequest, HoverRequest, InlayHintRequest,
    SelectionRangeRequest, SemanticTokensFullRequest, WorkspaceDiagnosticRequest,
    WorkspaceSymbolRequest,
};
use auto_lsp::lsp_types::{
    self, CodeActionProviderCapability, CodeLensOptions, CompletionOptions, DiagnosticOptions,
    DiagnosticServerCapabilities, HoverProviderCapability, OneOf, ServerCapabilities,
};
use auto_lsp::server::notification_registry::NotificationRegistry;
use auto_lsp::server::options::InitOptions;
use auto_lsp::server::request_registry::RequestRegistry;
use auto_lsp::server::Session;
use std::error::Error;
use std::panic::RefUnwindSafe;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let (connection, io_threads) = Connection::stdio();
    let db = BaseDb::default();

    let (mut session, params) = Session::create(
        InitOptions {
            parsers: &PYTHON_PARSERS,
            capabilities: ServerCapabilities {
                text_document_sync: TEXT_DOCUMENT_SYNC.clone(),
                workspace: WORKSPACE_PROVIDER.clone(),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        workspace_diagnostics: true,
                        ..Default::default()
                    },
                )),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                code_lens_provider: Some(CodeLensOptions {
                    resolve_provider: Some(false),
                }),
                inlay_hint_provider: Some(OneOf::Left(true)),
                semantic_tokens_provider: semantic_tokens_provider(
                    false,
                    Some(SUPPORTED_TYPES),
                    Some(SUPPORTED_MODIFIERS),
                ),
                document_symbol_provider: Some(OneOf::Left(true)),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            server_info: None,
        },
        connection,
        db,
    )?;

    // Initialize the session with the client's initialization options.
    // This will also add all documents, parse and send diagnostics.
    let init_results = session.init_workspace(params)?;
    if !init_results.is_empty() {
        init_results.into_iter().for_each(|result| {
            if let Err(err) = result {
                eprintln!("{}", err);
            }
        });
    };

    let mut request_registry = RequestRegistry::<BaseDb>::default();
    let mut notification_registry = NotificationRegistry::<BaseDb>::default();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    session.main_loop(
        on_requests(&mut request_registry),
        on_notifications(&mut notification_registry),
    )?;
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("Shutting down server");
    Ok(())
}

fn on_requests<Db: BaseDatabase + Clone + RefUnwindSafe>(
    registry: &mut RequestRegistry<Db>,
) -> &mut RequestRegistry<Db> {
    registry
        .on::<DocumentDiagnosticRequest, _>(diagnostics)
        .on::<DocumentSymbolRequest, _>(document_symbols)
        .on::<FoldingRangeRequest, _>(folding_ranges)
        .on::<HoverRequest, _>(hover)
        .on::<SemanticTokensFullRequest, _>(semantic_tokens_full)
        .on::<SelectionRangeRequest, _>(selection_ranges)
        .on::<WorkspaceSymbolRequest, _>(workspace_symbols)
        .on::<WorkspaceDiagnosticRequest, _>(workspace_diagnostics)
        .on::<InlayHintRequest, _>(inlay_hints)
        .on::<CodeActionRequest, _>(code_actions)
        .on::<CodeLensRequest, _>(code_lenses)
        .on::<Completion, _>(completion_items)
}

fn on_notifications<Db: BaseDatabase + Clone + RefUnwindSafe>(
    registry: &mut NotificationRegistry<Db>,
) -> &mut NotificationRegistry<Db> {
    registry
        .on_mut::<DidOpenTextDocument, _>(|s, p| Ok(open_text_document(s, p)?))
        .on_mut::<DidChangeTextDocument, _>(|s, p| {
            let file =
                s.db.get_file(&p.text_document.uri)
                    .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;
            file.update_edit(&mut s.db, &p)?;
            Ok(())
        })
        .on_mut::<DidChangeWatchedFiles, _>(|s, p| Ok(changed_watched_files(s, p)?))
        .on_mut::<Cancel, _>(|s, p| {
            let id: lsp_server::RequestId = match p.id {
                lsp_types::NumberOrString::Number(id) => id.into(),
                lsp_types::NumberOrString::String(id) => id.into(),
            };
            if let Some(response) = s.req_queue.incoming.cancel(id) {
                s.connection.sender.send(response.into())?;
            }
            Ok(())
        })
        .on::<DidSaveTextDocument, _>(|_s, _p| Ok(()))
        .on::<DidCloseTextDocument, _>(|_s, _p| Ok(()))
        .on::<SetTrace, _>(|_s, _p| Ok(()))
        .on::<LogTrace, _>(|_s, _p| Ok(()))
}
