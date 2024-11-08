use std::collections::HashMap;

use lsp_server::Connection;
use lsp_types::Url;
use workspace::Workspace;

pub mod cst_parser;
pub mod dispatchers;
pub mod init;
pub mod senders;
pub mod workspace;

pub struct Session {
    pub connection: Connection,
    pub extensions: HashMap<String, String>,
    pub workspaces: HashMap<Url, Workspace>,
}

impl Session {
    pub fn new(connection: Connection) -> Self {
        Self {
            connection,
            extensions: HashMap::new(),
            workspaces: HashMap::new(),
        }
    }
}
