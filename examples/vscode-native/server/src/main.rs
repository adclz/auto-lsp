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

use ast_python::capabilities::code_actions::dispatch_code_actions;
use ast_python::capabilities::code_lenses::dispatch_code_lenses;
use ast_python::capabilities::completion_items::dispatch_completion_items;
use ast_python::capabilities::document_symbols::dispatch_document_symbols;
use ast_python::capabilities::hover::dispatch_hover;
use ast_python::capabilities::inlay_hints::dispatch_inlay_hints;
use ast_python::capabilities::semantic_tokens::{dispatch_semantic_tokens, SUPPORTED_MODIFIERS, SUPPORTED_TYPES};
use ast_python::db::PYTHON_PARSERS;
use auto_lsp::core::salsa::db::{BaseDatabase, BaseDb};
use auto_lsp::lsp_server::{self, Connection};
use auto_lsp::lsp_types::notification::{
    Cancel, DidChangeTextDocument, DidChangeWatchedFiles, DidCloseTextDocument,
    DidOpenTextDocument, DidSaveTextDocument, LogTrace, SetTrace,
};
use auto_lsp::lsp_types::request::{
    CodeActionRequest, CodeLensRequest, Completion, DocumentDiagnosticRequest,
    DocumentSymbolRequest, HoverRequest, InlayHintRequest, SelectionRangeRequest,
    SemanticTokensFullRequest, WorkspaceDiagnosticRequest, WorkspaceSymbolRequest,
};
use auto_lsp::lsp_types::{CodeActionProviderCapability, CodeLensOptions, CompletionOptions, DiagnosticOptions, DiagnosticServerCapabilities, HoverProviderCapability, OneOf, ServerCapabilities};
use auto_lsp::server::capabilities::{
    changed_watched_files, get_code_actions, get_code_lenses, get_completion_items,
    get_diagnostics, get_document_symbols, get_hover, get_inlay_hints, get_selection_ranges,
    get_semantic_tokens_full, get_workspace_diagnostics, get_workspace_symbols, open_text_document,
    TraversalKind,
};
use auto_lsp::server::{semantic_tokens_provider, InitOptions, NotificationRegistry, RequestRegistry, Session, WORKSPACE_PROVIDER};
use fastrace::collector::Config;
use fastrace::collector::ConsoleReporter;
use std::error::Error;
use std::panic::RefUnwindSafe;
use auto_lsp::lsp_types::SemanticTokensServerCapabilities::SemanticTokensOptions;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let (connection, io_threads) = Connection::stdio();
    let db = BaseDb::default();

    stderrlog::new()
        .modules([module_path!(), "auto_lsp"])
        .verbosity(4)
        .init()
        .unwrap();

    fastrace::set_reporter(ConsoleReporter, Config::default());

    let mut session = Session::create(
        InitOptions {
            parsers: &PYTHON_PARSERS,
            capabilities: ServerCapabilities {
                workspace: WORKSPACE_PROVIDER.clone(),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(DiagnosticOptions {
                    workspace_diagnostics: true,
                    ..Default::default()
                })),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                code_lens_provider: Some(CodeLensOptions {
                    resolve_provider: Some(false),
                }),
                inlay_hint_provider: Some(OneOf::Left(true)),
                semantic_tokens_provider: semantic_tokens_provider(false, Some(SUPPORTED_TYPES), Some(SUPPORTED_MODIFIERS)),
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
            server_info: None
        },
        connection,
        db,
    )?;

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
        .on::<DocumentDiagnosticRequest, _>(|s, p| get_diagnostics(s, p))
        .on::<DocumentSymbolRequest, _>(|s, p| {
            get_document_symbols(s, p, TraversalKind::Single, dispatch_document_symbols)
        })
        .on::<HoverRequest, _>(|s, p| get_hover(s, p, dispatch_hover))
        .on::<SemanticTokensFullRequest, _>(|s, p| {
            get_semantic_tokens_full(s, p, TraversalKind::Iter, dispatch_semantic_tokens)
        })
        .on::<SelectionRangeRequest, _>(|s, p| get_selection_ranges(s, p))
        .on::<WorkspaceSymbolRequest, _>(|s, p| {
            get_workspace_symbols(s, p, TraversalKind::Single, dispatch_document_symbols)
        })
        .on::<WorkspaceDiagnosticRequest, _>(|s, p| get_workspace_diagnostics(s, p))
        .on::<InlayHintRequest, _>(|s, p| {
            get_inlay_hints(s, p, TraversalKind::Iter, dispatch_inlay_hints)
        })
        .on::<CodeActionRequest, _>(|s, p| {
            get_code_actions(s, p, TraversalKind::Iter, dispatch_code_actions)
        })
        .on::<CodeLensRequest, _>(|s, p| {
            get_code_lenses(s, p, TraversalKind::Iter, dispatch_code_lenses)
        })
        .on::<Completion, _>(|s, p| {
            get_completion_items(s, p, TraversalKind::Iter, dispatch_completion_items)
        })
}

fn on_notifications<Db: BaseDatabase + Clone + RefUnwindSafe>(
    registry: &mut NotificationRegistry<Db>,
) -> &mut NotificationRegistry<Db> {
    registry
        .on_mut::<DidOpenTextDocument, _>(|s, p| Ok(open_text_document(s, p)?))
        .on_mut::<DidChangeTextDocument, _>(|s, p| {
            Ok(s.db.update(&p.text_document.uri, &p.content_changes)?)
        })
        .on_mut::<DidChangeWatchedFiles, _>(|s, p| Ok(changed_watched_files(s, p)?))
        .on_mut::<Cancel, _>(|s, p| {
            let id: lsp_server::RequestId = match p.id {
                auto_lsp::lsp_types::NumberOrString::Number(id) => id.into(),
                auto_lsp::lsp_types::NumberOrString::String(id) => id.into(),
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
