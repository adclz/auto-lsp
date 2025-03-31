use crate::server::session::init::TextFn;
use auto_lsp_core::salsa::db::WorkspaceDatabase;
use lsp_server::Connection;
use options::InitOptions;
use parking_lot::Mutex;
use std::collections::HashMap;

pub mod documents;
pub mod fs;
pub mod init;
pub mod main_loop;
pub mod notification_registry;
pub mod options;
pub mod request_registry;

pub(crate) type ReqHandler<Db> = fn(&mut Session<Db>, lsp_server::Response);
type ReqQueue<Db> = lsp_server::ReqQueue<String, ReqHandler<Db>>;

/// Main session object that holds both lsp server connection and initialization options.
pub struct Session<Db: WorkspaceDatabase> {
    /// Initialization options provided by the library user.
    pub init_options: InitOptions,
    pub connection: Connection,
    /// Text `fn` used to parse text files with the correct encoding.
    ///
    /// The client is responsible for providing the encoding at initialization (UTF-8, 16 or 32).
    pub text_fn: TextFn,
    /// Language extensions to parser mappings.
    pub extensions: HashMap<String, String>,
    /// Request queue for incoming requests
    pub req_queue: ReqQueue<Db>,
    pub db: Db,
}
