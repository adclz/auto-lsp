use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, RwLock},
};

use lsp_types::{Diagnostic, Url};
use tree_sitter::{Query, QueryCapture};

use crate::traits::{ast_item::AstItem, ast_item_builder::AstItemBuilder};

pub type BinderFn = fn(
    url: Arc<Url>,
    capture: &QueryCapture,
    query: &Query,
) -> Option<Rc<RefCell<dyn AstItemBuilder>>>;

pub type ItemBinderFn = fn(
    roots: Vec<Rc<RefCell<dyn AstItemBuilder>>>,
) -> Vec<Result<Arc<RwLock<dyn AstItem>>, Diagnostic>>;
