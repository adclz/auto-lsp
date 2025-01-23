use std::{ops::ControlFlow, sync::Arc};

use crate::{
    core_ast::{
        data::ReferrersTrait,
        symbol::{DynSymbol, WeakSymbol},
        update::UpdateRange,
    },
    core_build::stack_builder::InvokeStackBuilderFn,
    document::Document,
};
use lsp_types::{Diagnostic, Url};
use parking_lot::RwLock;
use streaming_iterator::StreamingIterator;
use tree_sitter::{InputEdit, Language, Parser, Query};

pub struct Queries {
    pub core: Query,
    pub comments: Option<Query>,
    pub fold: Option<Query>,
    pub highlights: Option<Query>,
}

pub struct TreeSitter {
    pub parser: RwLock<Parser>,
    pub node_types: &'static str,
    pub language: Language,
    pub queries: Queries,
}

pub struct Parsers {
    pub tree_sitter: TreeSitter,
    pub ast_parser: InvokeStackBuilderFn,
}

pub struct Workspace {
    pub url: Arc<Url>,
    pub parsers: &'static Parsers,
    pub diagnostics: Vec<Diagnostic>,
    pub ast: Option<DynSymbol>,
    pub unsolved_checks: Vec<WeakSymbol>,
    pub unsolved_references: Vec<WeakSymbol>,
}

impl Workspace {
    pub fn set_comments(&self, document: &Document) -> anyhow::Result<()> {
        let comments_query = match self.parsers.tree_sitter.queries.comments {
            Some(ref query) => query,
            None => return Ok(()),
        };

        let source_code = document.texter.text.as_bytes();
        let cst = &document.tree;
        let ast = match self.ast.as_ref() {
            Some(ast) => ast,
            None => return Ok(()),
        };

        let mut cursor = tree_sitter::QueryCursor::new();
        let mut captures = cursor.captures(comments_query, cst.root_node(), source_code);

        while let Some((m, capture_index)) = captures.next() {
            let capture = m.captures[*capture_index];
            // Since a comment is not within a query, we look for the next named sibling

            let next_named_sibling = match capture.node.next_named_sibling() {
                Some(node) => node,
                None => continue,
            };

            // We then look if this next sibling exists in the ast

            let node = ast.read().find_at_offset(next_named_sibling.start_byte());

            if let Some(node) = node {
                let range = capture.node.range();
                if node.read().is_comment() {
                    node.write().set_comment(Some(std::ops::Range {
                        start: range.start_byte,
                        end: range.end_byte,
                    }));
                } else {
                    match node.read().get_parent() {
                        Some(parent) => {
                            let parent = parent.to_dyn().unwrap();
                            if parent.read().get_range().start == node.read().get_range().start {
                                if parent.read().is_comment() {
                                    parent.write().set_comment(Some(std::ops::Range {
                                        start: range.start_byte,
                                        end: range.end_byte,
                                    }));
                                }
                            }
                        }
                        None => {}
                    }
                }
            };
        }
        Ok(())
    }

    pub(crate) fn add_unsolved_check(&mut self, symbol: &DynSymbol) -> &mut Self {
        self.unsolved_checks.push(symbol.to_weak());
        self
    }

    pub fn get_unsolved_checks(&self) -> &Vec<WeakSymbol> {
        &self.unsolved_checks
    }

    pub(crate) fn add_unsolved_reference(&mut self, symbol: &DynSymbol) -> &mut Self {
        self.unsolved_references.push(symbol.to_weak());
        self
    }

    pub fn get_unsolved_references(&self) -> &Vec<WeakSymbol> {
        &self.unsolved_references
    }

    #[cfg(not(feature = "rayon"))]
    pub fn resolve_references(&mut self, document: &Document) -> &mut Self {
        self.unsolved_references.retain(|item| {
            let item = match item.to_dyn() {
                Some(read) => read,
                None => return false,
            };
            let read = item.read();
            match read.find(&document) {
                Ok(Some(target)) => {
                    target.write().add_referrer(item.to_weak());
                    drop(read);
                    item.write().set_target_reference(target.to_weak());
                    false
                }
                Ok(None) => true,
                Err(err) => {
                    self.diagnostics.push(err);
                    true
                }
            }
        });
        self
    }

    #[cfg(feature = "rayon")]
    pub fn resolve_references(&mut self, document: &Document) -> &mut Self {
        use parking_lot::RwLock;
        use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

        let diagnostics = RwLock::new(vec![]);
        self.unsolved_references = self
            .unsolved_references
            .par_iter()
            .cloned()
            .filter(|item| {
                let item = match item.to_dyn() {
                    Some(read) => read,
                    None => return false,
                };
                let read = item.read();
                match read.find(&document) {
                    Ok(Some(target)) => {
                        target.write().add_referrer(item.to_weak());
                        drop(read);
                        item.write().set_target_reference(target.to_weak());
                        false
                    }
                    Ok(None) => true,
                    Err(err) => {
                        diagnostics.write().push(err);
                        true
                    }
                }
            })
            .collect::<Vec<WeakSymbol>>();
        self.diagnostics.extend(diagnostics.into_inner());
        self
    }

    #[cfg(not(feature = "rayon"))]
    pub fn resolve_checks(&mut self, document: &Document) -> &mut Self {
        self.unsolved_checks.retain(|item| {
            let item = match item.to_dyn() {
                Some(read) => read,
                None => return false,
            };
            let read = item.read();
            match read.check(&document, &mut self.diagnostics) {
                Ok(()) => false,
                Err(()) => true,
            }
        });
        self
    }

    #[cfg(feature = "rayon")]
    pub fn resolve_checks(&mut self, document: &Document) -> &mut Self {
        use parking_lot::RwLock;
        use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

        let diagnostics = RwLock::new(vec![]);
        self.unsolved_checks = self
            .unsolved_checks
            .par_iter()
            .cloned()
            .filter(|item| {
                let item = match item.to_dyn() {
                    Some(read) => read,
                    None => return false,
                };
                let read = item.read();
                match read.check(&document, &mut diagnostics.write()) {
                    Ok(()) => false,
                    Err(()) => true,
                }
            })
            .collect::<Vec<WeakSymbol>>();
        self.diagnostics.extend(diagnostics.into_inner());
        self
    }

    pub fn swap_ast(
        &mut self,
        edit_ranges: &Vec<(InputEdit, bool)>,
        document: &Document,
    ) -> &mut Self {
        let mut root = self.ast.as_mut().unwrap().clone();
        let ast_parser = self.parsers.ast_parser;

        // All ranges have to be updated
        for (edit, _) in edit_ranges {
            let start_byte = edit.start_byte;
            let old_end_byte = edit.old_end_byte;
            let new_end_byte = edit.new_end_byte;

            let is_noop = old_end_byte == start_byte && new_end_byte == start_byte;
            if is_noop {
                continue;
            }

            root.edit_range(start_byte, (new_end_byte - old_end_byte) as isize);
        }

        // Filter out overlapping edits for ast edit
        // Since the containing node is already updated, child nodes do not need to be built twice.
        for (edit, is_ws) in filter_intersecting_edits(edit_ranges).iter() {
            let start_byte = edit.start_byte;
            let old_end_byte = edit.old_end_byte;
            let new_end_byte = edit.new_end_byte;

            let is_noop = old_end_byte == start_byte && new_end_byte == start_byte;
            if is_noop {
                continue;
            }

            let node = document
                .tree
                .root_node()
                .descendant_for_byte_range(edit.start_byte, edit.new_end_byte);

            if let Some(node) = node {
                if let Some(node) = node.parent() {
                    if node.is_error() {
                        log::warn!("");
                        log::warn!("Node has an invalid syntax, aborting incremental update");
                        continue;
                    }
                }
                if node.is_extra() {
                    log::info!("");
                    log::info!("Node is extra, only update ranges");
                    continue;
                }
            }

            if *is_ws {
                log::info!("");
                log::info!("Whitespace edit, only update ranges");
                continue;
            }

            let parent_check = match root.read().must_check() {
                true => Some(root.to_weak()),
                false => None,
            };

            let result = root.write().dyn_update(
                start_byte,
                (new_end_byte - old_end_byte) as isize,
                parent_check,
                self,
                document,
            );
            match result {
                ControlFlow::Break(Err(e)) => {
                    self.diagnostics.push(e);
                }
                ControlFlow::Continue(()) => {
                    log::info!("");
                    log::info!("No incremental update available, root node will be reparsed");
                    log::info!("");
                    let mut ast_builder = ast_parser(self, document, None);
                    match ast_builder {
                        Ok(ref mut new_root) => {
                            root.swap(new_root);
                        }
                        Err(e) => {
                            self.diagnostics.push(e);
                        }
                    }
                }
                ControlFlow::Break(Ok(_)) => {}
            };
        }
        self
    }
}

/// Filter out intersecting edits and keep the biggest one
fn filter_intersecting_edits(params: &Vec<(InputEdit, bool)>) -> Vec<(InputEdit, bool)> {
    if params.is_empty() {
        return vec![];
    }

    if params.len() == 1 {
        return params.clone();
    }

    // Sort by range
    let mut sorted_edits = params.clone();
    sorted_edits.sort_by_key(|(edit, _)| edit.start_byte + edit.new_end_byte);

    // Filter out overlapping edits
    let mut filtered = Vec::new();
    let mut last_end = sorted_edits[0].0.new_end_byte;

    for edit in sorted_edits {
        // Check if current edit starts after previous edit ends
        if edit.0.start_byte >= last_end {
            filtered.push(edit);
            last_end = edit.0.new_end_byte;
        }
    }

    filtered
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::{InputEdit, Point};

    #[test]
    fn test_intersecting_edits() {
        let edits = vec![
            (
                InputEdit {
                    start_byte: 0,
                    new_end_byte: 20,
                    old_end_byte: 0,
                    start_position: Point::default(),
                    old_end_position: Point::default(),
                    new_end_position: Point::default(),
                },
                false,
            ),
            (
                InputEdit {
                    start_byte: 10,
                    new_end_byte: 30,
                    old_end_byte: 0,
                    start_position: Point::default(),
                    old_end_position: Point::default(),
                    new_end_position: Point::default(),
                },
                false,
            ),
            (
                InputEdit {
                    start_byte: 20,
                    new_end_byte: 40,
                    old_end_byte: 0,
                    start_position: Point::default(),
                    old_end_position: Point::default(),
                    new_end_position: Point::default(),
                },
                false,
            ),
        ];

        let filtered = filter_intersecting_edits(&edits);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].0.start_byte, 20);
        assert_eq!(filtered[0].0.new_end_byte, 40);
    }
}
