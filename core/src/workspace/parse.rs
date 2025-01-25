use std::ops::ControlFlow;

use tree_sitter::InputEdit;

use crate::{ast::UpdateRange, document::Document};

use super::Workspace;

impl Workspace {
    /// Parses a document and updates the AST.
    ///
    /// This method assumes the document has already been updated and parsed by the tree-sitter parser.
    /// It performs the following steps:
    ///
    /// 1. Clears existing diagnostics.
    /// 2. Extracts syntax diagnostics from tree-sitter.
    /// 3. If no AST exists, it creates one from scratch.
    /// 4. If edit ranges are provided, updates the AST incrementally.
    pub fn parse(
        &mut self,
        edit_ranges: Option<&Vec<(InputEdit, bool)>>,
        document: &Document,
    ) -> &mut Self {
        // Clean diagnostics
        self.diagnostics.clear();

        // Get new diagnostics from tree sitter
        Workspace::get_tree_sitter_errors(
            &document.tree.root_node(),
            document.texter.text.as_bytes(),
            &mut self.diagnostics,
        );

        let ast_parser = self.parsers.ast_parser;

        // Create a new AST if none exists and return
        let root = match self.ast.clone() {
            Some(root) => root,
            None => {
                self.ast = match ast_parser(self, &document, None) {
                    Ok(ast) => Some(ast),
                    Err(e) => {
                        self.diagnostics.push(e);
                        None
                    }
                };
                self.set_comments(document)
                    .resolve_checks(document)
                    .resolve_references(document);
                #[cfg(feature = "log")]
                self.log_unsolved();

                return self;
            }
        };

        // If no edit ranges, return
        let edit_ranges = match edit_ranges {
            Some(ranges) => ranges,
            None => return self,
        };

        // Apply edit ranges to update AST nodes
        for (edit, _) in edit_ranges {
            let start_byte = edit.start_byte;
            let old_end_byte = edit.old_end_byte;
            let new_end_byte = edit.new_end_byte;

            let is_noop = old_end_byte == start_byte && new_end_byte == start_byte;
            if is_noop {
                continue;
            }

            root.edit_range(
                start_byte,
                (new_end_byte.wrapping_sub(old_end_byte)) as isize,
            );
        }

        // Filter intersecting edits and update AST incrementally
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

            // Skip invalid nodes
            if let Some(node) = node {
                if let Some(node) = node.parent() {
                    if node.is_error() {
                        #[cfg(feature = "log")]
                        {
                            log::warn!("");
                            log::warn!("Node has an invalid syntax, aborting incremental update");
                        }
                        continue;
                    }
                }
                if node.is_extra() {
                    #[cfg(feature = "log")]
                    {
                        log::info!("");
                        log::info!("Node is extra, only update ranges");
                    }
                    continue;
                }
            }

            // Skip whitespace-only edits
            if *is_ws {
                #[cfg(feature = "log")]
                {
                    log::info!("");
                    log::info!("Whitespace edit, only update ranges");
                }
                continue;
            }

            let parent_check = match root.read().must_check() && !root.read().has_check_pending() {
                true => Some(root.to_weak()),
                false => None,
            };

            // Update AST incrementally
            let result = root.write().dyn_update(
                start_byte,
                (new_end_byte.wrapping_sub(old_end_byte)) as isize,
                parent_check,
                self,
                document,
            );
            match result {
                ControlFlow::Break(Err(e)) => {
                    self.diagnostics.push(e);
                }
                ControlFlow::Continue(()) => {
                    #[cfg(feature = "log")]
                    {
                        log::info!("");
                        log::info!("No incremental update available, root node will be reparsed");
                        log::info!("");
                    }
                    // clean checks, since we are going to reparse the root node
                    self.unsolved_checks.clear();
                    self.unsolved_references.clear();

                    let ast_builder = ast_parser(self, document, None);
                    match ast_builder {
                        Ok(new_root) => {
                            self.ast = Some(new_root);
                        }
                        Err(e) => {
                            self.diagnostics.push(e);
                            self.ast = None;
                        }
                    }
                }
                ControlFlow::Break(Ok(_)) => {}
            };
        }
        self.set_comments(document)
            .resolve_checks(document)
            .resolve_references(document);

        #[cfg(feature = "log")]
        self.log_unsolved();

        return self;
    }

    #[cfg(feature = "log")]
    fn log_unsolved(&self) -> &Self {
        {
            if !self.unsolved_checks.is_empty() {
                log::info!("");
                log::warn!("Unsolved checks: {:?}", self.unsolved_checks.len());
            }

            if !self.unsolved_references.is_empty() {
                log::info!("");
                log::warn!("Unsolved references: {:?}", self.unsolved_references.len());
            }
            self
        }
    }
}

/// Filters intersecting edits and keeps only the largest non-overlapping ones.
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
