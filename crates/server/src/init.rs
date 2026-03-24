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
use std::num::NonZeroUsize;

use crate::vendored::pool::Pool;

use super::InitOptions;
use super::Session;
use lsp_server::{Connection, ReqQueue};
use lsp_types::{InitializeParams, InitializeResult, PositionEncodingKind};

impl<Db: salsa::Database> Session<Db> {
    pub(crate) fn new(init_options: InitOptions, connection: Connection, db: Db) -> Self {
        let (task_sender, task_receiver) = crossbeam_channel::unbounded();

        let max_threads = std::thread::available_parallelism()
            .unwrap_or_else(|_| NonZeroUsize::new(1).unwrap())
            .get();

        log::info!("Max threads: {max_threads}");

        let encoding = init_options
            .capabilities
            .position_encoding
            .clone()
            .unwrap_or(PositionEncodingKind::UTF16);

        log::info!("Position encoding: {encoding:?}");

        Self {
            init_options,
            encoding,
            connection,
            extensions: HashMap::new(),
            req_queue: ReqQueue::default(),
            db,
            task_receiver,
            task_sender,
            task_pool: Pool::new(max_threads),
            on_error: None,
        }
    }

    /// Create a new session with the given initialization options.
    ///
    /// This will establish the connection with the client and send the server capabilities.
    pub fn create(
        init_options: InitOptions,
        connection: Connection,
        db: Db,
    ) -> anyhow::Result<(Session<Db>, InitializeParams)> {
        // This is a workaround for a deadlock issue in WASI libc.
        // See https://github.com/WebAssembly/wasi-libc/pull/491
        #[cfg(target_arch = "wasm32")]
        std::fs::metadata("/workspace").unwrap();

        log::info!("Starting LSP server");
        log::info!("");

        // Create the transport. Includes the stdio (stdin and stdout) versions but this could
        // also be implemented to use sockets or HTTP.
        let (id, resp) = connection.initialize_start()?;
        let params: InitializeParams = serde_json::from_value(resp)?;

        let server_capabilities = serde_json::to_value(&InitializeResult {
            capabilities: init_options.capabilities.clone(),
            server_info: init_options.server_info.clone(),
        })
        .unwrap();

        connection.initialize_finish(id, server_capabilities)?;

        Ok((Session::new(init_options, connection, db), params))
    }
}
