/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/

use crate::errors::PositionError;
use downcast_rs::{impl_downcast, DowncastSync};
use std::cmp::Ordering;
use std::sync::Arc;
use tree_sitter::Node;

/// Trait representing an AST node.
pub trait AstNode: std::fmt::Debug + Send + Sync + DowncastSync {
    /// Returns `true` if a given [`tree_sitter::Node`] matches this node type.
    fn contains(node: &Node) -> bool
    where
        Self: Sized;

    /// Returns the inner node as a trait object.
    ///
    /// If the node is a struct, returns self.    
    fn lower(&self) -> &dyn AstNode;

    /// Returns the unique ID of this node.
    ///
    /// IDs are assigned when [`TryFrom`] is called and are unique within the tree.
    fn get_id(&self) -> usize;

    /// Returns the ID of the parent node, if any.
    fn get_parent_id(&self) -> Option<usize>;

    /// Returns the [`tree_sitter::Range`] of this node.
    fn get_range(&self) -> &tree_sitter::Range;

    /// Returns the LSP-compatible range of this node.
    fn get_lsp_range(&self) -> lsp_types::Range {
        let range = self.get_range();
        lsp_types::Range {
            start: lsp_types::Position {
                line: range.start_point.row as u32,
                character: range.start_point.column as u32,
            },
            end: lsp_types::Position {
                line: range.end_point.row as u32,
                character: range.end_point.column as u32,
            },
        }
    }

    /// Returns the start position in LSP format.
    fn get_start_position(&self) -> lsp_types::Position {
        let range = self.get_range();
        lsp_types::Position {
            line: range.start_point.row as u32,
            character: range.start_point.column as u32,
        }
    }

    /// Returns the end position in LSP format.
    fn get_end_position(&self) -> lsp_types::Position {
        let range = self.get_range();
        lsp_types::Position {
            line: range.end_point.row as u32,
            character: range.end_point.column as u32,
        }
    }

    /// Returns the UTF-8 text slice corresponding to this node.
    ///
    /// Returns:
    /// - `Ok(&str)` with the node's source text
    /// - `Err(PositionError::WrongTextRange)` if the range is invalid
    /// - `Err(PositionError::UTF8Error)` if the byte slice is not valid UTF-8
    fn get_text<'a>(&self, source_code: &'a [u8]) -> Result<&'a str, PositionError> {
        let range = self.get_range();
        let range = range.start_byte..range.end_byte;
        match source_code.get(range.start..range.end) {
            Some(text) => match std::str::from_utf8(text) {
                Ok(text) => Ok(text),
                Err(utf8_error) => Err(PositionError::UTF8Error { range, utf8_error }),
            },
            None => Err(PositionError::WrongTextRange { range }),
        }
    }

    /// Retrieves the parent node, if present, from the node list.
    ///
    /// The node list must be sorted by ID.
    fn get_parent<'a>(&'a self, nodes: &'a [Arc<dyn AstNode>]) -> Option<&'a Arc<dyn AstNode>> {
        match nodes.first() {
            Some(first) => {
                assert_eq!(
                    first.get_id(),
                    0,
                    "get_parent called on an unsorted node list"
                );
                nodes.get(self.get_parent_id()?)
            }
            None => None,
        }
    }
}

impl_downcast!(AstNode);

impl PartialEq for dyn AstNode {
    fn eq(&self, other: &Self) -> bool {
        self.get_range().eq(other.get_range()) && self.get_id().eq(&other.get_id())
    }
}

impl Eq for dyn AstNode {}

impl PartialOrd for dyn AstNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.get_id().cmp(&other.get_id()))
    }
}

impl Ord for dyn AstNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_id().cmp(&other.get_id())
    }
}
