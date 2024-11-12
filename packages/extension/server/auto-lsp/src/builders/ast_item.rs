use crate::traits::ast_builder::AstBuilder;
use crate::traits::ast_item::AstItem;
use crate::traits::ast_item_builder::{AstItemBuilder, DeferredAstItemBuilder};
use lsp_types::Diagnostic;
use std::rc::Rc;
use std::sync::RwLock;
use std::{cell::RefCell, sync::Arc};
use streaming_iterator::StreamingIterator;
use tree_sitter::{Query, QueryCapture};

#[macro_export]
macro_rules! builder_error {
    ($range: expr, $text: expr) => {
        lsp_types::Diagnostic::new(
            $range,
            Some(lsp_types::DiagnosticSeverity::ERROR),
            None,
            None,
            $text.into(),
            None,
            None,
        )
    };
}

pub type BinderFn =
    fn(capture: &QueryCapture, query: &Query) -> Option<Rc<RefCell<dyn AstItemBuilder>>>;

pub type ItemBinderFn = fn(
    roots: Vec<Rc<RefCell<dyn AstItemBuilder>>>,
    workspace: &dyn crate::traits::workspace::WorkspaceContext,
) -> Vec<Result<Arc<RwLock<dyn AstItem>>, Diagnostic>>;

struct Deferred {
    parent: Rc<RefCell<dyn AstItemBuilder>>,
    child: Rc<RefCell<dyn AstItemBuilder>>,
    binder: Box<
        dyn Fn(
            Rc<RefCell<dyn AstItemBuilder>>,
            Rc<RefCell<dyn AstItemBuilder>>,
            &[u8],
        ) -> Result<(), Diagnostic>,
    >,
}
