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
use auto_lsp::lsp_types::{self, CompletionOptions};
use auto_lsp::python::PYTHON_PARSERS;
use auto_lsp::server::capabilities::{
    changed_watched_files, get_code_actions, get_code_lenses, get_completion_items,
    get_diagnostics, get_document_symbols, get_hover, get_inlay_hints, get_selection_ranges,
    get_semantic_tokens_full, get_workspace_diagnostics, get_workspace_symbols, open_text_document,
};
use auto_lsp::server::{InitOptions, LspOptions, NotificationRegistry, RequestRegistry, Session};

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let (connection, io_threads) = Connection::stdio();
    let db = BaseDb::default();

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
        register_requests(&mut request_registry),
        register_notifications(&mut notification_registry),
    )?;
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("Shutting down server");
    Ok(())
}

fn register_requests<Db: BaseDatabase>(
    registry: &mut RequestRegistry<Db>,
) -> &mut RequestRegistry<Db> {
    registry
        .register::<DocumentDiagnosticRequest, _>(|s, p| get_diagnostics(&s.db, p))
        .register::<DocumentSymbolRequest, _>(|s, p| get_document_symbols(&s.db, p))
        .register::<HoverRequest, _>(|s, p| get_hover(&s.db, p))
        .register::<SemanticTokensFullRequest, _>(|s, p| get_semantic_tokens_full(&s.db, p))
        .register::<SelectionRangeRequest, _>(|s, p| get_selection_ranges(&s.db, p))
        .register::<WorkspaceSymbolRequest, _>(|s, p| get_workspace_symbols(&s.db, p))
        .register::<WorkspaceDiagnosticRequest, _>(|s, p| get_workspace_diagnostics(&s.db, p))
        .register::<InlayHintRequest, _>(|s, p| get_inlay_hints(&s.db, p))
        .register::<CodeActionRequest, _>(|s, p| get_code_actions(&s.db, p))
        .register::<CodeLensRequest, _>(|s, p| get_code_lenses(&s.db, p))
        .register::<Completion, _>(|s, p| get_completion_items(&s.db, p))
}

fn register_notifications<Db: BaseDatabase>(
    registry: &mut NotificationRegistry<Db>,
) -> &mut NotificationRegistry<Db> {
    registry
        .register::<DidOpenTextDocument, _>(|s, p| Ok(open_text_document(s, p)?))
        .register::<DidChangeTextDocument, _>(|s, p| {
            Ok(s.db.update(&p.text_document.uri, &p.content_changes)?)
        })
        .register::<DidChangeWatchedFiles, _>(|s, p| Ok(changed_watched_files(s, p)?))
        .register::<Cancel, _>(|s, p| {
            let id: lsp_server::RequestId = match p.id {
                lsp_types::NumberOrString::Number(id) => id.into(),
                lsp_types::NumberOrString::String(id) => id.into(),
            };
            if let Some(response) = s.req_queue.incoming.cancel(id) {
                s.connection.sender.send(response.into())?;
            }
            Ok(())
        })
        .register::<DidSaveTextDocument, _>(|s, p| Ok(()))
        .register::<DidCloseTextDocument, _>(|s, p| Ok(()))
        .register::<SetTrace, _>(|s, p| Ok(()))
        .register::<LogTrace, _>(|s, p| Ok(()))
}
