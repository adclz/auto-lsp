#![allow(unused)]
use crate::document::{texter_impl::updateable::Change, Document};

use super::Workspace;

impl Workspace {
    fn set_ast(&mut self, document: &Document) -> &mut Self {
        self.unsolved_checks.clear();
        self.unsolved_references.clear();

        let ast_parser = self.parsers.ast_parser;

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
    /// Parses a document and updates the AST.
    ///
    /// This method assumes the document has already been updated and parsed by the tree-sitter parser.
    /// It performs the following steps:
    ///
    /// 1. Clears existing diagnostics.
    /// 2. Extracts syntax diagnostics from tree-sitter.
    /// 3. If no AST exists, it creates one from scratch.
    /// 4. If edit ranges are provided, updates the AST incrementally.
    pub fn parse(&mut self, edit_ranges: Option<&Vec<Change>>, document: &Document) -> &mut Self {
        // Clear diagnostics
        self.diagnostics.clear();

        #[cfg(feature = "incremental")]
        // Clear changes
        self.changes.clear();

        // Get new diagnostics from tree sitter
        Workspace::get_tree_sitter_errors(
            &document.tree.root_node(),
            document.texter.text.as_bytes(),
            &mut self.diagnostics,
        );

        // Clear AST if document is empty
        if document.texter.text.is_empty() {
            self.ast = None;
            self.unsolved_checks.clear();
            self.unsolved_references.clear();
            return self;
        }

        // Create a new AST if none exists and returns
        let root = match self.ast.clone() {
            Some(root) => root,
            None => return self.set_ast(document),
        };

        #[cfg(not(feature = "incremental"))]
        {
            self.set_ast(document);
        }

        #[cfg(feature = "incremental")]
        {
            // If no edit ranges, return
            let edit_ranges = match edit_ranges {
                Some(ranges) => ranges,
                None => return self,
            };

            let mut collect = vec![];
            for edit in edit_ranges {
                root.write().adjust(*edit, &mut collect, self, document);
            }

            // Filter intersecting edits and update AST incrementally
            for edit in filter_intersecting_edits(edit_ranges) {
                let Change {
                    kind: _,
                    is_whitespace,
                    ..
                } = edit;

                // Skip whitespace-only edits
                if is_whitespace {
                    #[cfg(feature = "log")]
                    {
                        log::info!("");
                        log::info!("Whitespace edit");
                    }
                    continue;
                }

                let mut collect = vec![];

                // Attempt to update AST incrementally
                let result = root.write().update(edit, &mut collect, self, document);
                match result {
                    std::ops::ControlFlow::Break(crate::ast::UpdateState::Result(Ok(()))) => {
                        // Update succeeded
                        #[cfg(feature = "log")]
                        {
                            log::info!("");
                            log::info!("Incremental update succeeded");
                        }
                    }
                    std::ops::ControlFlow::Break(crate::ast::UpdateState::Result(Err(e))) => {
                        // Update failed, add error to diagnostics
                        self.diagnostics.push(e);
                    }
                    _ => {
                        // Could not locate the node to update
                        // therefore we need to reparse the root node
                        #[cfg(feature = "log")]
                        {
                            log::info!("");
                            log::info!("No incremental update available, root node will be parsed");
                            log::info!("");
                        }
                        self.set_ast(document);
                    }
                };
            }
            self.set_comments(document)
                .resolve_checks(document)
                .resolve_references(document);

            #[cfg(feature = "log")]
            {
                self.changes.iter().for_each(|change| {
                    log::info!("");
                    change.log();
                });
                self.log_unsolved();
            }
        }
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

#[cfg(feature = "incremental")]
/// Filters intersecting edits and keeps only the largest non-overlapping ones.
fn filter_intersecting_edits(params: &Vec<Change>) -> Vec<Change> {
    if params.is_empty() {
        return vec![];
    }

    if params.len() == 1 {
        return params.clone();
    }

    // Sort by range
    let mut sorted_edits = params.clone();
    sorted_edits
        .sort_by_key(|change| change.input_edit.start_byte + change.input_edit.new_end_byte);

    // Filter out overlapping edits
    let mut filtered = Vec::new();
    let mut last_end = sorted_edits[0].input_edit.new_end_byte;

    for edit in sorted_edits {
        // Check if current edit starts after previous edit ends
        if edit.input_edit.start_byte >= last_end {
            filtered.push(edit);
            last_end = edit.input_edit.new_end_byte;
        }
    }

    filtered
}

#[cfg(feature = "incremental")]
#[cfg(test)]
mod tests {
    use crate::document::texter_impl::updateable::ChangeKind;

    use super::*;
    use tree_sitter::{InputEdit, Point};

    #[test]
    fn test_intersecting_edits() {
        let edits = vec![
            Change {
                kind: ChangeKind::Replace,
                input_edit: InputEdit {
                    start_byte: 0,
                    new_end_byte: 20,
                    old_end_byte: 0,
                    start_position: Point::default(),
                    old_end_position: Point::default(),
                    new_end_position: Point::default(),
                },
                is_whitespace: false,
                trim_start: 0,
            },
            Change {
                kind: ChangeKind::Replace,
                input_edit: InputEdit {
                    start_byte: 10,
                    new_end_byte: 30,
                    old_end_byte: 0,
                    start_position: Point::default(),
                    old_end_position: Point::default(),
                    new_end_position: Point::default(),
                },
                is_whitespace: false,
                trim_start: 0,
            },
            Change {
                kind: ChangeKind::Replace,
                input_edit: InputEdit {
                    start_byte: 20,
                    new_end_byte: 40,
                    old_end_byte: 0,
                    start_position: Point::default(),
                    old_end_position: Point::default(),
                    new_end_position: Point::default(),
                },
                is_whitespace: false,
                trim_start: 0,
            },
        ];

        let filtered = filter_intersecting_edits(&edits);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].input_edit.start_byte, 20);
        assert_eq!(filtered[0].input_edit.new_end_byte, 40);
    }
}
