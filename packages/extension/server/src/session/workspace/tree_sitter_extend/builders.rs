use auto_lsp::builder_error;
use auto_lsp::traits::ast_item::AstItem;
use auto_lsp::traits::ast_item_builder::{AstItemBuilder, DeferredAstItemBuilder};
use lsp_textdocument::FullTextDocument;
use lsp_types::{Diagnostic, Url};
use std::rc::Rc;
use std::sync::{RwLock, Weak};
use std::{cell::RefCell, sync::Arc};
use streaming_iterator::StreamingIterator;

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

pub type BuilderFn = fn(
    query: &tree_sitter::Query,
    root_node: tree_sitter::Node,
    source_code: &[u8],
    url: Arc<Url>,
) -> BuilderResult;

pub struct BuilderResult {
    pub item: Result<Arc<RwLock<dyn AstItem>>, Diagnostic>,
    pub errors: Vec<Diagnostic>,
}

pub trait Builder {
    fn builder(
        query: &tree_sitter::Query,
        root_node: tree_sitter::Node,
        source_code: &[u8],
        url: Arc<Url>,
    ) -> BuilderResult;
}

pub trait Finder {
    fn find_reference(&self, doc: &FullTextDocument) -> Option<Weak<RwLock<dyn AstItem>>>;
}

impl<T: AstItemBuilder> Builder for T {
    fn builder(
        query: &tree_sitter::Query,
        root_node: tree_sitter::Node,
        source_code: &[u8],
        url: Arc<Url>,
    ) -> BuilderResult {
        let mut errors = vec![];

        let mut cursor = tree_sitter::QueryCursor::new();
        let mut captures = cursor.captures(&query, root_node, source_code);

        let mut roots = vec![];
        let mut stack: Vec<Rc<RefCell<dyn AstItemBuilder>>> = vec![];
        let mut deferred_maps: Vec<Deferred> = vec![];

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
                        let node = match T::static_query_binder(url.clone(), &capture, &query) {
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
                        if intersecting_ranges(&parent.borrow().get_range(), &capture.node.range())
                        {
                            let node =
                                match parent.borrow().query_binder(url.clone(), &capture, query) {
                                    Some(builder) => builder,
                                    None => {
                                        errors.push(builder_error!(
                                            tree_sitter_range_to_lsp_range(&capture.node.range()),
                                            format!(
                                            "Failed to create builder for query: {:?} using {:?}",
                                            query.capture_names()[capture_index as usize],
                                            query.capture_names()
                                                [parent.borrow().get_query_index()]
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
                                Ok(def) => match def {
                                    DeferredAstItemBuilder::HashMap(def) => {
                                        deferred_maps.push(Deferred {
                                            parent: parent.clone(),
                                            child: node.clone(),
                                            binder: def,
                                        });
                                    }
                                    _ => {}
                                },
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

        deferred_maps.into_iter().for_each(|def| {
            if let Err(err) = (def.binder)(def.parent, def.child, source_code) {
                errors.push(err);
            }
        });

        let result = roots.pop().unwrap();
        let result: std::cell::Ref<'_, dyn AstItemBuilder> = result.borrow();
        let result = result.try_into_item();
        BuilderResult {
            item: result,
            errors,
        }
    }
}

impl<T: AstItem> Finder for T {
    fn find_reference(&self, doc: &FullTextDocument) -> Option<Weak<RwLock<dyn AstItem>>> {
        let pattern = self.get_text(doc.get_content(None).as_bytes());

        while let Some(scope) = self.get_parent_scope() {
            match scope.upgrade() {
                Some(scope) => {
                    let scope = scope.read().unwrap();
                    let range = scope.get_scope_range();
                    let area = doc
                        .get_content(None)
                        .get(range[0] as usize..range[1] as usize)
                        .unwrap();

                    for (index, _) in area.match_indices(pattern) {
                        if let Some(elem) = scope.find_at_offset(&index) {
                            return Some(Arc::downgrade(&elem));
                        }
                    }
                }
                None => continue,
            }
        }

        None
    }
}
