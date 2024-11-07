use crate::traits::ast_builder::AstBuilder;
use crate::traits::ast_item::AstItem;
use crate::traits::ast_item_builder::AstItemBuilder;
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

fn intersecting_ranges(range1: &tree_sitter::Range, range2: &tree_sitter::Range) -> bool {
    range1.start_byte <= range2.start_byte && range1.end_byte >= range2.end_byte
}

pub fn localized_builder(
    query: &tree_sitter::Query,
    ast_builder: &AstBuilder,
    root_node: tree_sitter::Node,
    source_code: &[u8],
    range: std::ops::Range<usize>,
) -> Result<Option<Rc<RefCell<dyn AstItemBuilder>>>, Vec<Diagnostic>> {
    let mut errors = vec![];
    let mut cursor = tree_sitter::QueryCursor::new();
    cursor.set_byte_range(range);

    let mut captures = cursor.captures(&query, root_node, source_code);

    let mut root = None;
    let mut stack: Vec<Rc<RefCell<dyn AstItemBuilder>>> = vec![];

    while let Some((m, capture_index)) = captures.next() {
        let capture = m.captures[*capture_index];
        let capture_index = capture.index as usize;

        let node = match (ast_builder.query_to_builder)(&capture, &query) {
            Some(builder) => builder,
            None => {
                panic!(
                    "Warning: Failed to create builder for query: {:?}",
                    query.capture_names()[capture_index as usize]
                );
            }
        };
        let mut parent = stack.pop();
        loop {
            match parent {
                None => {
                    if root.is_some() {
                        return Ok(root);
                    } else {
                        root = Some(node.clone());
                        stack.push(node.clone());
                    }
                }
                Some(parent) => {
                    if intersecting_ranges(&parent.borrow().get_range(), &node.borrow().get_range())
                    {
                        if let Err(err) =
                            parent.borrow_mut().add(&query, node.clone(), &source_code)
                        {
                            errors.push(err);
                        };
                        stack.push(parent.clone());
                        stack.push(node.clone());
                        break;
                    };
                }
            }
            parent = stack.pop();
        }
    }
    Ok(root)
}

fn tree_sitter_range_to_lsp_range(range: &tree_sitter::Range) -> lsp_types::Range {
    let start = range.start_point;
    let end = range.end_point;
    lsp_types::Range {
        start: lsp_types::Position {
            line: start.row as u32,
            character: start.column as u32,
        },
        end: lsp_types::Position {
            line: end.row as u32,
            character: end.column as u32,
        },
    }
}

pub fn builder(
    query: &tree_sitter::Query,
    ast_builder: &AstBuilder,
    root_node: tree_sitter::Node,
    source_code: &[u8],
) -> Vec<Result<Arc<RwLock<dyn AstItem>>, Diagnostic>> {
    let mut errors = vec![];

    let mut cursor = tree_sitter::QueryCursor::new();
    let mut captures = cursor.captures(&query, root_node, source_code);

    let mut roots = vec![];
    let mut stack: Vec<Rc<RefCell<dyn AstItemBuilder>>> = vec![];
    let mut deferred: Vec<Deferred> = vec![];

    while let Some((m, capture_index)) = captures.next() {
        let capture = m.captures[*capture_index];
        let capture_index = capture.index as usize;

        eprintln!(
            "Create builder for query: {:?}",
            query.capture_names()[capture_index],
        );

        let mut parent = stack.pop();

        loop {
            match &parent {
                None => {
                    let node = match (ast_builder.query_to_builder)(&capture, &query) {
                        Some(builder) => builder,
                        None => {
                            errors.push(builder_error!(
                                tree_sitter_range_to_lsp_range(&capture.node.range()),
                                format!(
                                    "Failed to create builder for query: {:?}, is the symbol declared in root ?",
                                    query.capture_names()[capture_index as usize]
                                )
                            ));
                            break;
                        }
                    };

                    eprintln!(
                        "Add parent {:?} to roots",
                        query.capture_names()[node.borrow().get_query_index()]
                    );
                    roots.push(node.clone());
                    stack.push(node.clone());
                    break;
                }
                Some(ref parent) => {
                    if intersecting_ranges(&parent.borrow().get_range(), &capture.node.range()) {
                        let node = match parent.borrow().query_binder(&capture, query) {
                            Some(builder) => builder,
                            None => {
                                errors.push(builder_error!(
                                    tree_sitter_range_to_lsp_range(&capture.node.range()),
                                    format!(
                                        "Failed to create builder for query: {:?} using {:?}",
                                        query.capture_names()[capture_index as usize],
                                        query.capture_names()[parent.borrow().get_query_index()]
                                    )
                                ));
                                break;
                            }
                        };

                        eprintln!(
                            "Add {:?} to parent {:?}",
                            query.capture_names()[node.borrow().get_query_index()],
                            query.capture_names()[parent.borrow().get_query_index()]
                        );

                        match parent.borrow_mut().add(&query, node.clone(), &source_code) {
                            Ok(def) => {
                                if let Some(def) = def {
                                    eprintln!("Is deferred!");
                                    deferred.push(Deferred {
                                        parent: parent.clone(),
                                        child: node.clone(),
                                        binder: def,
                                    });
                                }
                            }
                            Err(err) => errors.push(err),
                        };
                        stack.push(parent.clone());
                        stack.push(node.clone());
                        break;
                    }
                }
            }
            parent = stack.pop();
        }
    }

    deferred.into_iter().for_each(|def| {
        if let Err(err) = (def.binder)(def.parent, def.child, source_code) {
            errors.push(err);
        }
    });

    let mut result = (ast_builder.builder_to_item)(roots);
    result.extend(errors.into_iter().map(Err));
    result
}
