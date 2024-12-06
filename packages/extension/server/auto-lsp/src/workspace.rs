use std::sync::{RwLock, Weak};

use lsp_types::Url;

use super::symbol::AstSymbol;

pub trait WorkspaceContext {
    fn find(&self, node: &dyn AstSymbol) -> Option<Weak<RwLock<dyn AstSymbol>>>;
}
