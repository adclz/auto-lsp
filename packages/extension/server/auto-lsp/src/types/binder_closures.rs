use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, RwLock},
};

use lsp_types::Diagnostic;
use tree_sitter::{Query, QueryCapture};

use crate::traits::{ast_item::AstItem, ast_item_builder::AstItemBuilder};

pub type BinderFn =
    fn(capture: &QueryCapture, query: &Query) -> Option<Rc<RefCell<dyn AstItemBuilder>>>;

pub type ItemBinderFn = fn(
    roots: Vec<Rc<RefCell<dyn AstItemBuilder>>>,
    workspace: &dyn crate::traits::workspace::WorkspaceContext,
) -> Vec<Result<Arc<RwLock<dyn AstItem>>, Diagnostic>>;
