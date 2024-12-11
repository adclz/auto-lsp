use crate::builder_error;
use crate::pending_symbol::{AstBuilder, PendingSymbol};
use crate::symbol::{DynSymbol, SymbolData};
use crate::workspace::WorkspaceContext;
use lsp_textdocument::FullTextDocument;
use lsp_types::{Diagnostic, Url};
use std::sync::Arc;
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

pub type BuilderFn = fn(
    ctx: &dyn WorkspaceContext,
    query: &tree_sitter::Query,
    root_node: tree_sitter::Node,
    doc: &FullTextDocument,
    url: Arc<Url>,
) -> BuilderResult;

pub struct BuilderResult {
    pub item: Result<DynSymbol, Diagnostic>,
    pub errors: Vec<Diagnostic>,
}

pub trait Builder {
    fn builder(
        ctx: &dyn WorkspaceContext,
        query: &tree_sitter::Query,
        root_node: tree_sitter::Node,
        doc: &FullTextDocument,
        url: Arc<Url>,
    ) -> BuilderResult;
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

fn create_root_node<T: AstBuilder>(
    url: Arc<Url>,
    query: &Query,
    capture: &QueryCapture,
    capture_index: usize,
) -> Result<PendingSymbol, Diagnostic> {
    let mut node = T::new(
        url.clone(),
        query,
        capture_index,
        capture.node.range(),
        capture.node.start_position(),
        capture.node.end_position(),
    );

    match node.take() {
        Some(builder) => Ok(PendingSymbol::new(builder)),
        None => Err(builder_error!(
            tree_sitter_range_to_lsp_range(&capture.node.range()),
            format!(
                "Failed to create builder for query: {:?}, is the symbol declared in the AST ?",
                query.capture_names()[capture_index as usize]
            )
        )),
    }
}

fn create_child_node<'a>(
    parent: &PendingSymbol,
    url: Arc<Url>,
    query: &Query,
    capture: &QueryCapture,
    capture_index: usize,
) -> Result<PendingSymbol, Diagnostic> {
    let node = parent
        .get_rc()
        .borrow()
        .query_binder(url.clone(), &capture, &query);

    match node.as_ref() {
        Some(builder) => Ok(builder.clone()),
        None => Err(builder_error!(
            tree_sitter_range_to_lsp_range(&capture.node.range()),
            format!(
                "Failed to create builder for query: {:?}, is the symbol declared in the AST ?",
                query.capture_names()[capture_index as usize],
            )
        )),
    }
}

fn finalize_builder(
    mut roots: Vec<PendingSymbol>,
    errors: &mut Vec<Diagnostic>,
    doc: &FullTextDocument,
    ctx: &dyn WorkspaceContext,
) -> BuilderResult {
    let result_node = match roots.pop() {
        Some(node) => node,
        None => {
            return BuilderResult {
                item: Err(builder_error!(
                    lsp_types::Range {
                        start: lsp_types::Position {
                            line: 0,
                            character: 0,
                        },
                        end: lsp_types::Position {
                            line: doc.line_count() as u32,
                            character: 0,
                        },
                    },
                    "No root node found".to_string()
                )),
                errors: errors.clone(),
            }
        }
    };

    let result: std::cell::Ref<'_, dyn AstBuilder> = result_node.get_rc().borrow();

    let mut deferred = vec![];
    let item = result.try_to_dyn_symbol(&mut deferred);

    match item {
        Err(err) => {
            return BuilderResult {
                item: Err(err),
                errors: errors.clone(),
            };
        }
        Ok(ref item) => {
            item.write().inject_parent(item.to_weak());
            if item.read().is_accessor() {
                match item.read().find(doc, ctx) {
                    Ok(a) => {
                        if let Some(a) = a {
                            // todo!
                            a.write().get_mut_referrers().add_reference(item.to_weak());

                            item.write().set_target(a.to_weak());
                        };
                    }
                    Err(err) => errors.push(err),
                }
            }

            if let Err(err) = item.write().find(doc, ctx) {
                errors.push(err);
            }

            let item = item.read();

            if item.must_check() {
                item.check(doc, errors);
            }
        }
    }

    for item in deferred {
        let acc = if item.read().is_accessor() {
            match item.read().find(doc, ctx) {
                Ok(a) => a,
                Err(err) => {
                    errors.push(err);
                    None
                }
            }
        } else {
            None
        };

        if let Some(a) = acc {
            // todo!
            a.write().get_mut_referrers().add_reference(item.to_weak());

            item.write().set_target(a.to_weak());
        }

        if item.read().must_check() {
            item.read().check(doc, errors);
        }
    }

    BuilderResult {
        item,
        errors: errors.clone(),
    }
}

impl<T: AstBuilder> Builder for T {
    fn builder(
        ctx: &dyn WorkspaceContext,
        query: &tree_sitter::Query,
        root_node: tree_sitter::Node,
        doc: &FullTextDocument,
        url: Arc<Url>,
    ) -> BuilderResult {
        let source_code = doc.get_content(None).as_bytes();
        let mut errors = vec![];

        let mut cursor = tree_sitter::QueryCursor::new();
        let mut captures = cursor.captures(&query, root_node, source_code);

        let mut roots = vec![];
        let mut stack: Vec<_> = vec![];

        while let Some((m, capture_index)) = captures.next() {
            let capture = m.captures[*capture_index];
            let capture_index = capture.index as usize;

            let mut parent = stack.pop();

            loop {
                match &parent {
                    None => {
                        let node = match create_root_node::<T>(
                            url.clone(),
                            query,
                            &capture,
                            capture_index,
                        ) {
                            Ok(node) => node,
                            Err(err) => {
                                errors.push(err);
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
                            let node = match create_child_node(
                                parent,
                                url.clone(),
                                &query,
                                &capture,
                                capture_index,
                            ) {
                                Ok(node) => node,
                                Err(err) => {
                                    errors.push(err);
                                    break;
                                }
                            };

                            match parent.get_rc().borrow_mut().add(
                                &query,
                                node.clone(),
                                &source_code,
                            ) {
                                Ok(_) => (),
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

        finalize_builder(roots, &mut errors, doc, ctx)
    }
}
