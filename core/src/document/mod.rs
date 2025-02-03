use texter::core::text::Text;
use texter_impl::{change::WrapChange, updateable::WrapTree};
use tree_sitter::{Point, Tree};

pub(crate) mod texter_impl;
pub use texter_impl::updateable::{Change, ChangeKind};

/// Represents a text document that combines plain text [`texter`] with its parsed syntax tree [`tree_sitter::Tree`].
///
/// This struct allows for incremental updates of both the text content and the syntax tree,
/// ensuring they stay synchronized after each change. It provides utility functions for querying
/// the syntax tree, such as finding nodes by position or range.
pub struct Document {
    pub texter: Text,
    pub tree: Tree,
}

impl Document {
    pub fn new(texter: Text, tree: Tree) -> Self {
        Self { texter, tree }
    }

    /// Updates the document based on the provided list of text changes.
    ///
    /// This method applies the changes to both the text (`texter`) and the syntax tree (`tree`).
    /// It uses incremental parsing to minimize the cost of updating the syntax tree.
    ///
    /// # Parameters
    /// - `parser`: A mutable reference to the Tree-sitter parser used to re-parse the document.
    /// - `changes`: A vector of `TextDocumentContentChangeEvent` objects representing text changes.
    ///
    /// # Returns
    /// A `Result` containing:
    /// - A vector of tuples where each tuple consists of:
    ///   - [`tree_sitter::InputEdit`]: Represents the edit applied to the syntax tree.
    ///   - `bool`: Indicates whether the edit involves only whitespace changes.
    ///
    /// # Errors
    /// Returns an error if Tree-sitter fails to re-parse the updated text
    pub fn update(
        &mut self,
        parser: &mut tree_sitter::Parser,
        changes: &Vec<lsp_types::TextDocumentContentChangeEvent>,
    ) -> anyhow::Result<Vec<Change>> {
        let mut new_tree = WrapTree::from(&mut self.tree);

        for change in changes {
            self.texter
                .update(WrapChange::from(change).change, &mut new_tree)?;
        }

        let edits = new_tree.get_edits();
        self.tree = parser
            .parse(self.texter.text.as_bytes(), Some(&self.tree))
            .ok_or(anyhow::format_err!("Tree sitter failed to edit tree",))?;

        Ok(edits)
    }

    /// Retrieves the smallest syntax node that spans the given position in the document.
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

    /// Converts a byte offset in the document to its corresponding position (line and character).
    pub fn position_at(&self, offset: usize) -> Option<lsp_types::Position> {
        self.tree
            .root_node()
            .descendant_for_byte_range(offset, offset)
            .map(|pos| lsp_types::Position {
                line: pos.start_position().row as u32,
                character: pos.start_position().column as u32,
            })
    }

    /// Retrieves the range (start and end positions) of the smallest syntax node that spans the given byte offset.
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

    /// Converts a position (line and character) in the document to its corresponding byte offset.
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
