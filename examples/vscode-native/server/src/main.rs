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

use std::error::Error;
use std::panic::RefUnwindSafe;

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
use auto_lsp::lsp_types::CompletionOptions;
use auto_lsp::server::capabilities::{
    changed_watched_files, get_code_actions, get_code_lenses, get_completion_items,
    get_diagnostics, get_document_symbols, get_hover, get_inlay_hints, get_selection_ranges,
    get_semantic_tokens_full, get_workspace_diagnostics, get_workspace_symbols, open_text_document,
};
use auto_lsp::server::{InitOptions, LspOptions, NotificationRegistry, RequestRegistry, Session};

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let (connection, io_threads) = Connection::stdio();
    let db = BaseDb::default();

    stderrlog::new()
        .modules([module_path!(), "auto_lsp"])
        .verbosity(4)
        .init()
        .unwrap();

    let mut session = Session::create(
        InitOptions {
            parsers: &PYTHON_PARSERS,
            lsp_options: LspOptions {
                workspace_symbols: true,
                document_symbols: true,
                diagnostics: true,
                inlay_hints: true,
                hover_info: true,
                code_lens: true,
                completions: Some(CompletionOptions {
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
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
        .on::<DocumentSymbolRequest, _>(|s, p| get_document_symbols(s, p))
        .on::<HoverRequest, _>(|s, p| get_hover(s, p))
        .on::<SemanticTokensFullRequest, _>(|s, p| get_semantic_tokens_full(s, p))
        .on::<SelectionRangeRequest, _>(|s, p| get_selection_ranges(s, p))
        .on::<WorkspaceSymbolRequest, _>(|s, p| get_workspace_symbols(s, p))
        .on::<WorkspaceDiagnosticRequest, _>(|s, p| get_workspace_diagnostics(s, p))
        .on::<InlayHintRequest, _>(|s, p| get_inlay_hints(s, p))
        .on::<CodeActionRequest, _>(|s, p| get_code_actions(s, p))
        .on::<CodeLensRequest, _>(|s, p| get_code_lenses(s, p))
        .on::<Completion, _>(|s, p| get_completion_items(s, p))
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
