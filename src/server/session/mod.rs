use std::{collections::HashMap, sync::LazyLock};

use crate::server::session::init::TextFn;
use auto_lsp_core::workspace::Workspace;
use lsp_server::Connection;
use options::InitOptions;
use parking_lot::Mutex;

pub mod documents;
pub mod fs;
pub mod init;
pub mod main_loop;
pub mod options;

/// Workspace
pub(crate) static WORKSPACE: LazyLock<Mutex<Workspace>> = LazyLock::new(Mutex::default);

/// Main session object that holds both lsp server connection and initialization options.
///
/// Documents are stored in [`WORKSPACE`].
pub struct Session {
    /// Initialization options provided by the library user.
    pub init_options: InitOptions,
    pub connection: Connection,
    /// Text `fn` used to parse text files with the correct encoding.
    ///
    /// The client is responsible for providing the encoding at initialization (UTF-8, 16 or 32).
    pub text_fn: TextFn,
    /// Language extensions to parser mappings.
    pub extensions: HashMap<String, String>,
}
