use crate::traits::ast_item::AstItem;
use crate::traits::ast_item_builder::AstItemBuilder;
use std::rc::Rc;
use std::sync::RwLock;
use std::{cell::RefCell, sync::Arc};
use streaming_iterator::StreamingIterator;
use tree_sitter::{Query, QueryCapture};

fn intersecting_ranges(range1: &tree_sitter::Range, range2: &tree_sitter::Range) -> bool {
    range1.start_byte <= range2.start_byte && range1.end_byte >= range2.end_byte
}

pub fn localized_builder(
    query: &tree_sitter::Query,
    binder_fn: fn(capture: &QueryCapture, query: &Query) -> Option<Rc<RefCell<dyn AstItemBuilder>>>,
    root_node: tree_sitter::Node,
    source_code: &[u8],
    range: std::ops::Range<usize>,
) -> Option<std::rc::Rc<std::cell::RefCell<dyn AstItemBuilder>>> {
    use std::cell::RefCell;
    use std::rc::Rc;

    let mut cursor = tree_sitter::QueryCursor::new();
    cursor.set_byte_range(range);

    let mut captures = cursor.captures(&query, root_node, source_code);

    let mut root = None;
    let mut stack: Vec<Rc<RefCell<dyn AstItemBuilder>>> = vec![];

    while let Some((m, capture_index)) = captures.next() {
        let capture = m.captures[*capture_index];
        let capture_index = capture.index as usize;

        let node = match binder_fn(&capture, &query) {
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
                        return root;
                    } else {
                        root = Some(node.clone());
                        stack.push(node.clone());
                    }
                }
                Some(parent) => {
                    if intersecting_ranges(&parent.borrow().get_range(), &node.borrow().get_range())
                    {
                        if let false = parent.borrow_mut().add(&query, node.clone()) {
                            eprintln!(
                                "Warning: Failed to add {:?} to parent {:?}",
                                query.capture_names()[node.borrow().get_query_index()],
                                query.capture_names()[parent.borrow().get_query_index()]
                            );
                        }
                        stack.push(parent.clone());
                        stack.push(node.clone());
                        break;
                    };
                }
            }
            parent = stack.pop();
        }
    }
    root
}

pub fn builder(
    query: &tree_sitter::Query,
    query_binder_fn: fn(
        capture: &QueryCapture,
        query: &Query,
    ) -> Option<Rc<RefCell<dyn AstItemBuilder>>>,
    item_binder_fn: fn(
        roots: Vec<std::rc::Rc<std::cell::RefCell<dyn AstItemBuilder>>>,
    ) -> Vec<std::sync::Arc<std::sync::RwLock<dyn AstItem>>>,
    root_node: tree_sitter::Node,
    source_code: &[u8],
) -> Vec<Arc<RwLock<dyn AstItem>>> {
    use std::cell::RefCell;
    use std::rc::Rc;

    let mut cursor = tree_sitter::QueryCursor::new();
    let mut captures = cursor.captures(&query, root_node, source_code);

    let mut roots = vec![];
    let mut stack: Vec<Rc<RefCell<dyn AstItemBuilder>>> = vec![];

    //eprintln!("count captures: {:?}", captures.len());
    while let Some((m, capture_index)) = captures.next() {
        let capture = m.captures[*capture_index];
        let capture_index = capture.index as usize;

        eprintln!(
            "Create builder for query: {:?}",
            query.capture_names()[capture_index],
        );
        let node = match query_binder_fn(&capture, &query) {
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
                    eprintln!(
                        "Add parent {:?} to roots",
                        query.capture_names()[node.borrow().get_query_index()]
                    );
                    roots.push(node.clone());
                    stack.push(node.clone());
                    break;
                }
                Some(parent) => {
                    if intersecting_ranges(&parent.borrow().get_range(), &node.borrow().get_range())
                    {
                        eprintln!(
                            "Add {:?} to parent {:?}",
                            query.capture_names()[node.borrow().get_query_index()],
                            query.capture_names()[parent.borrow().get_query_index()]
                        );

                        if let false = parent.borrow_mut().add(&query, node.clone()) {
                            eprintln!(
                                "Warning: Failed to add {:?} to parent {:?}",
                                query.capture_names()[node.borrow().get_query_index()],
                                query.capture_names()[parent.borrow().get_query_index()]
                            );
                        }
                        stack.push(parent.clone());
                        stack.push(node.clone());
                        break;
                    };
                }
            }
            parent = stack.pop();
        }
    }

    item_binder_fn(roots)
}
