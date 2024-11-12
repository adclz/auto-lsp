use std::{
    collections::HashMap,
    sync::{RwLock, Weak},
};

use auto_lsp::traits::{ast_item::AstItem, workspace::WorkspaceContext};
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

impl WorkspaceContext for Session {
    fn find(&self, position: &tree_sitter::Range, url: &Url) -> Option<Weak<RwLock<dyn AstItem>>> {
        todo!()
    }
}
