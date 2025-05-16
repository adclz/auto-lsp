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

use std::collections::HashMap;

use super::task_pool::TaskPool;
use super::InitOptions;
use super::Session;
use auto_lsp_core::salsa::db::BaseDatabase;
use lsp_server::{Connection, ReqQueue};
use lsp_types::WorkspaceServerCapabilities;
use lsp_types::{
    CodeLensOptions, DiagnosticOptions, DiagnosticServerCapabilities, DocumentLinkOptions,
    InitializeParams, InitializeResult, OneOf, PositionEncodingKind,
    SelectionRangeProviderCapability, SemanticTokensFullOptions, SemanticTokensLegend,
    SemanticTokensOptions, ServerCapabilities, WorkspaceFoldersServerCapabilities,
};
#[cfg(target_arch = "wasm32")]
use std::fs;
use texter::core::text::Text;

/// Function to create a new [`Text`] from a [`String`]
pub(crate) type TextFn = fn(String) -> Text;

fn decide_encoding(encs: Option<&[PositionEncodingKind]>) -> (TextFn, PositionEncodingKind) {
    const DEFAULT: (TextFn, PositionEncodingKind) = (Text::new_utf16, PositionEncodingKind::UTF16);
    let Some(encs) = encs else {
        return DEFAULT;
    };

    for enc in encs {
        if *enc == PositionEncodingKind::UTF16 {
            return (Text::new_utf16, enc.clone());
        } else if *enc == PositionEncodingKind::UTF8 {
            return (Text::new, enc.clone());
        }
    }

    DEFAULT
}

impl<Db: BaseDatabase + Default> Session<Db> {
    pub(crate) fn new(
        mut init_options: InitOptions,
        connection: Connection,
        text_fn: TextFn,
        db: Db,
    ) -> Self {
        let (sender, task_rx) = crossbeam_channel::unbounded();

        let max_threads = std::thread::available_parallelism().unwrap().get();

        log::info!("Max threads: {}", max_threads);

        Self {
            init_options,
            connection,
            text_fn,
            extensions: HashMap::new(),
            req_queue: ReqQueue::default(),
            db,
            task_rx,
            task_pool: TaskPool::new_with_threads(sender, max_threads),
        }
    }

    /// Create a new session with the given initialization options.
    ///
    /// This will establish the connection with the client and send the server capabilities.
    pub fn create(
        mut init_options: InitOptions,
        connection: Connection,
        db: Db,
    ) -> anyhow::Result<Session<Db>> {
        // This is a workaround for a deadlock issue in WASI libc.
        // See https://github.com/WebAssembly/wasi-libc/pull/491
        #[cfg(target_arch = "wasm32")]
        fs::metadata("/workspace").unwrap();

        log::info!("Starting LSP server");
        log::info!("");

        // Create the transport. Includes the stdio (stdin and stdout) versions but this could
        // also be implemented to use sockets or HTTP.
        let (id, resp) = connection.initialize_start()?;
        let params: InitializeParams = serde_json::from_value(resp)?;

        let pos_encoding = params
            .capabilities
            .general
            .as_ref()
            .and_then(|g| g.position_encodings.as_deref());

        let (t_fn, enc) = decide_encoding(pos_encoding);
        init_options.capabilities.position_encoding = Some(enc);

        let server_capabilities = serde_json::to_value(&InitializeResult {
            capabilities: init_options.capabilities.clone(),
            server_info: init_options.server_info.clone(),
        })
        .unwrap();

        connection.initialize_finish(id, server_capabilities)?;

        let mut session = Session::new(init_options, connection, t_fn, db);

        // Initialize the session with the client's initialization options.
        // This will also add all documents, parse and send diagnostics.
        let init_results = session.init_workspace(params)?;
        if !init_results.is_empty() {
            //todo: What to do with workspace initilization errors ?
            //There might be a lot of errors and sending them back to the client may not be the best decision.
        };

        Ok(session)
    }
}
