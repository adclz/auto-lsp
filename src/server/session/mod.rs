use std::{collections::HashMap, sync::LazyLock};

use crate::server::session::init::TextFn;
use auto_lsp_core::document::Document;
use auto_lsp_core::workspace::Workspace;
use init::InitOptions;
use lsp_server::{Connection, IoThreads};
use lsp_types::Url;
use parking_lot::Mutex;

pub mod comment;
pub mod documents;
pub mod init;
pub mod lexer;
pub mod main_loop;
pub mod senders;
pub mod workspace;

/// List of workspaces and documents in the current session.
pub(crate) static WORKSPACES: LazyLock<Mutex<HashMap<Url, (Workspace, Document)>>> =
    LazyLock::new(Mutex::default);

/// Main session object that holds both lsp server connection and initialization options.
///
/// Documents are stored in [`WORKSPACES`].
pub struct Session {
    /// Initialization options provided by the library user.
    pub init_options: InitOptions,
    pub connection: Connection,
    pub io_threads: IoThreads,
    /// Text `fn` used to parse text files with the correct encoding.
    ///
    /// The client is responsible for providing the encoding at initialization (UTF-8, 16 or 32).
    pub text_fn: TextFn,
    /// Language extensions to parser mappings.
    pub extensions: HashMap<String, String>,
}
