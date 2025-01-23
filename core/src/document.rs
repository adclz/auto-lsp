use texter::core::text::Text;
use tree_sitter::{Point, Tree};

pub struct Document {
    pub texter: Text,
    pub tree: Tree,
}

impl Document {
    pub fn new(texter: Text, tree: Tree) -> Self {
        Self { texter, tree }
    }

    /// Get the smallest node within the root node that spans the given range.
    pub fn descendant_at_position(
        &self,
        position: lsp_types::Position,
    ) -> Option<tree_sitter::Node<'_>> {
        let position = Point {
            row: position.line as usize,
            column: position.character as usize,
        };

        self.tree
            .root_node()
            .descendant_for_point_range(position, position)
    }

    /// Get the smallest node within root node that spans the given range.
    pub fn position_at(&self, offset: usize) -> Option<lsp_types::Position> {
        self.tree
            .root_node()
            .descendant_for_byte_range(offset, offset)
            .map(|pos| lsp_types::Position {
                line: pos.start_position().row as u32,
                character: pos.start_position().column as u32,
            })
    }

    /// Get the smallest node's range within root node that spans the given range.
    pub fn range_at(&self, offset: usize) -> Option<lsp_types::Range> {
        self.tree
            .root_node()
            .descendant_for_byte_range(offset, offset)
            .map(|pos| lsp_types::Range {
                start: lsp_types::Position {
                    line: pos.start_position().row as u32,
                    character: pos.start_position().column as u32,
                },
                end: lsp_types::Position {
                    line: pos.end_position().row as u32,
                    character: pos.end_position().column as u32,
                },
            })
    }

    /// Get the byte offset of the given position.
    pub fn offset_at(&self, position: lsp_types::Position) -> Option<usize> {
        match self.tree.root_node().descendant_for_point_range(
            Point {
                row: position.line as usize,
                column: position.character as usize,
            },
            Point {
                row: position.line as usize,
                column: position.character as usize,
            },
        ) {
            Some(node) => Some(node.start_byte()),
            None => None,
        }
    }
}
