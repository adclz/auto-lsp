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
    fn find(&self, node: &dyn AstItem, url: &Url) -> Option<Weak<RwLock<dyn AstItem>>> {
        let mut result = vec![];
        let workspace = self.workspaces.get(url).unwrap();
        let source_code = workspace.document.get_content(None).as_bytes();
        let word = node.get_text(source_code);

        let highest_scope = node.get_parent_scope();
        match highest_scope {
            Some(scope) => {
                let query = Query::new(
                    &workspace.cst_parser.language,
                    &format!("((identifier) @id (#match? @id \"^{}+\"))", word),
                )
                .unwrap();

                let mut query_cursor = QueryCursor::new();
                let range = scope.get_scope_range();
                query_cursor.set_byte_range(Range {
                    start: range[0],
                    end: range[1],
                });
                let mut captures =
                    query_cursor.captures(&query, workspace.cst.root_node(), source_code);

                while let Some((m, capture_index)) = captures.next() {
                    let capture = m.captures[*capture_index];

                    workspace
                        .ast
                        .iter()
                        .filter_map(|x| x.find_at_offset(&capture.node.start_byte()))
                        .for_each(|x| {
                            result.push(Arc::downgrade(&x));
                        });
                }
            }
            None => {}
        }
        result.first().cloned()
    }
}
