use std::sync::{RwLock, Weak};

use lsp_types::Url;

use super::ast_item::AstItem;

pub trait WorkspaceContext {
    fn find(&self, position: &tree_sitter::Range, url: &Url) -> Option<Weak<RwLock<dyn AstItem>>>;
}
