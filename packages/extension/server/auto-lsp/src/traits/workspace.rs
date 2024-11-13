use std::sync::{RwLock, Weak};

use lsp_types::Url;

use super::ast_item::AstItem;

pub trait WorkspaceContext {
    fn find(&self, node: &dyn AstItem, url: &Url) -> Option<Weak<RwLock<dyn AstItem>>>;
}
