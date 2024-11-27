use auto_lsp::builder_error;
use auto_lsp::traits::ast_item::{AstItem, IsAccessor};
use auto_lsp::traits::ast_item_builder::{AstItemBuilder, PendingSymbol};
use lsp_textdocument::FullTextDocument;
use lsp_types::{Diagnostic, Url};
use std::rc::Rc;
use std::sync::{RwLock, Weak};
use std::{cell::RefCell, sync::Arc};
use streaming_iterator::StreamingIterator;
struct Deferred {
    parent: PendingSymbol,
    child: PendingSymbol,
    binder: Box<dyn Fn(PendingSymbol, PendingSymbol, &[u8]) -> Result<(), Diagnostic>>,
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
        let mut stack: Vec<_> = vec![];
        let mut deferred_maps: Vec<Deferred> = vec![];

        while let Some((m, capture_index)) = captures.next() {
            let capture = m.captures[*capture_index];
            let capture_index = capture.index as usize;

            let mut parent = stack.pop();

            loop {
                match &parent {
                    None => {
                        let node = T::static_query_binder(url.clone(), &capture, &query);
                        let node = match node.as_ref() {
                            Some(builder) => builder,
                            None => {
                                errors.push(builder_error!(
                                    tree_sitter_range_to_lsp_range(&capture.node.range()),
                                    format!(
                                        "Failed to create builder for query: {:?}, is the symbol declared in the AST ?",
                                        query.capture_names()[capture_index as usize]
                                    )
                                ));
                                break;
                            }
                        };
                        roots.push(node.clone());
                        stack.push(node.clone());
                        break;
                    }
                    Some(ref parent) => {
                        if intersecting_ranges(
                            &parent.get_rc().borrow().get_range(),
                            &capture.node.range(),
                        ) {
                            let node =
                                parent
                                    .get_rc()
                                    .borrow()
                                    .query_binder(url.clone(), &capture, query);

                            let node = match node.as_ref() {
                                Some(builder) => builder,
                                None => {
                                    errors.push(builder_error!(
                                            tree_sitter_range_to_lsp_range(&capture.node.range()),
                                            format!(
                                            "Failed to create builder for query: {:?}, is the symbol declared in the AST ?",
                                            query.capture_names()[capture_index as usize],
                                        )
                                        ));
                                    break;
                                }
                            };

                            match parent.get_rc().borrow_mut().add(
                                &query,
                                node.clone(),
                                &source_code,
                            ) {
                                Ok(def) => {
                                    if let Some(def) = def {
                                        deferred_maps.push(Deferred {
                                            parent: parent.clone(),
                                            child: node.clone(),
                                            binder: def,
                                        })
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

        deferred_maps.into_iter().for_each(|def| {
            if let Err(err) = (def.binder)(def.parent, def.child, source_code) {
                errors.push(err);
            }
        });

        let result = roots.pop().unwrap();
        let result: std::cell::Ref<'_, dyn AstItemBuilder> = result.get_rc().borrow();

        let mut todo = vec![];
        let result = result.try_into_item(&mut todo);

        todo.iter().for_each(|item| {
            if *item.is_accessor() {
                todo!()
            }
        });

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
                    let ranges = scope.get_scope_range();

                    for range in ranges {
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
                }
                None => continue,
            }
        }

        None
    }
}
