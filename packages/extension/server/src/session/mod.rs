use std::collections::HashMap;

use lsp_server::Connection;
use lsp_types::Url;
use workspace::Workspace;

pub mod dispatchers;
pub mod init;
pub mod parser_provider;
pub mod senders;
pub mod workspace;

pub struct Session<'a> {
    pub connection: Connection,
    pub extensions: HashMap<String, String>,
    pub workspaces: HashMap<Url, Workspace<'a>>,
}

impl<'a> Session<'a> {
    pub fn new(connection: Connection) -> Self {
        Self {
            connection,
            extensions: HashMap::new(),
            workspaces: HashMap::new(),
        }
    }
}
