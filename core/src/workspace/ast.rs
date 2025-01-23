use std::ops::ControlFlow;

use tree_sitter::InputEdit;

use crate::{ast::UpdateRange, document::Document};

use super::Workspace;

impl Workspace {
    pub fn create_ast(
        &mut self,
        edit_ranges: Option<&Vec<(InputEdit, bool)>>,
        document: &Document,
    ) -> &mut Self {
        let ast_parser = self.parsers.ast_parser;

        // If no root node, create a new one and return
        let mut root = match self.ast.clone() {
            Some(root) => root,
            None => {
                self.ast = match ast_parser(self, &document, None) {
                    Ok(ast) => Some(ast),
                    Err(e) => {
                        self.diagnostics.push(e);
                        None
                    }
                };
                return self;
            }
        };

        // If no edit ranges, return
        let edit_ranges = match edit_ranges {
            Some(ranges) => ranges,
            None => return self,
        };

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

        // Filter out intersecting edits for ast edit
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

    // Filter out intersecting edits
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
