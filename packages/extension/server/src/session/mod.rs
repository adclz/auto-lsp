use std::{
    collections::HashMap,
    ops::Range,
    sync::{Arc, RwLock, Weak},
};

use auto_lsp::traits::{ast_item::AstItem, workspace::WorkspaceContext};
use lsp_server::Connection;
use lsp_types::Url;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Query, QueryCursor};
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
    fn find(&self, node: &dyn AstItem) -> Option<Weak<RwLock<dyn AstItem>>> {
        todo!()
    }
}
