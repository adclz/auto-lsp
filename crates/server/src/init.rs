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

        log::info!(target: "auto_lsp::server::init", "Max threads: {max_threads}");

        let encoding = init_options
            .capabilities
            .position_encoding
            .clone()
            .unwrap_or(PositionEncodingKind::UTF16);

        log::info!(target: "auto_lsp::server::init", "Position encoding: {encoding:?}");

        Self {
            init_options,
            encoding,
            connection,
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

        log::info!(target: "auto_lsp::server::init", "Starting LSP server");

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
