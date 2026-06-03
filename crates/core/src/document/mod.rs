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

use lsp_types::PositionEncodingKind;
use texter::{change::GridIndex, core::text::Text};
use texter_impl::{change::WrapChange, updateable::WrapTree};
use tree_sitter::{Point, Tree};

use crate::errors::{DocumentError, TreeSitterError};

pub(crate) mod texter_impl;

/// Represents a text document that combines plain text [`texter`] with its parsed syntax tree
/// [`tree_sitter::Tree`].
///
/// Encoding-aware position conversions are delegated to texter via
/// [`GridIndex::normalize`]/[`GridIndex::denormalize`]; the underlying [`Text`] knows the
/// negotiated encoding internally, so it is no longer duplicated on `Document`.
#[derive(Debug, Clone)]
pub struct Document {
    pub texter: Text,
    pub tree: Tree,
}

impl Document {
    /// Creates a new `Document` instance with the provided source, syntax tree, and encoding.
    ///
    /// Defaults to UTF-16 if the encoding is not specified or unrecognized.
    pub fn new(source: String, tree: Tree, encoding: Option<&PositionEncodingKind>) -> Self {
        let texter = match encoding.map(|e| e.as_str()) {
            Some("utf-8") => Text::new(source),
            Some("utf-32") => Text::new_utf32(source),
            _ => Text::new_utf16(source),
        };
        Self { texter, tree }
    }

    pub fn as_str(&self) -> &str {
        &self.texter.text
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.texter.text.as_bytes()
    }

    pub fn is_empty(&self) -> bool {
        self.texter.text.is_empty()
    }

    /// Updates the document based on the provided list of text changes.
    ///
    /// Applies the changes to both the text [`texter`] and the syntax tree [`Tree`], using
    /// incremental parsing to minimize the cost of updating the syntax tree.
    ///
    /// # Errors
    /// Returns an error if Tree-sitter fails to reparse the updated text.
    pub fn update(
        &mut self,
        parser: &mut tree_sitter::Parser,
        changes: &[lsp_types::TextDocumentContentChangeEvent],
    ) -> Result<(), DocumentError> {
        let mut new_tree = WrapTree::from(&mut self.tree);

        for change in changes {
            self.texter
                .update(WrapChange::from(change).change, &mut new_tree)?;
        }

        self.tree = parser
            .parse(self.texter.text.as_bytes(), Some(&self.tree))
            .ok_or_else(|| DocumentError::from(TreeSitterError::TreeSitterParser))?;

        Ok(())
    }

    /// Converts an LSP [`lsp_types::Range`] from the client encoding to UTF-8, returning a new range.
    /// Mirrors texter's own [`GridIndex::normalize`].
    pub fn normalize_range(
        &self,
        position: &lsp_types::Range,
    ) -> Result<lsp_types::Range, DocumentError> {
        let start = self.normalize_position(&position.start)?;
        let end = self.normalize_position(&position.end)?;

        Ok(lsp_types::Range { start, end })
    }

    /// Converts an LSP [`lsp_types::Position`] from the client encoding to UTF-8, returning a new position.
    /// Mirrors texter's own [`GridIndex::normalize`].
    pub fn normalize_position(
        &self,
        position: &lsp_types::Position,
    ) -> Result<lsp_types::Position, DocumentError> {
        let mut grid = GridIndex {
            row: position.line as usize,
            col: position.character as usize,
        };
        grid.normalize(&self.texter)?;

        Ok(lsp_types::Position {
            line: grid.row as u32,
            character: grid.col as u32,
        })
    }

    /// Converts a tree-sitter [`tree_sitter::Range`] to an LSP [`lsp_types::Range`],
    /// adjusting columns to the LSP client encoding.
    /// Mirrors texter's own [`GridIndex::denormalize`].
    pub fn denormalize_range(
        &self,
        range: &tree_sitter::Range,
    ) -> Result<lsp_types::Range, DocumentError> {
        let start = self.denormalize_point(range.start_point)?;
        let end = self.denormalize_point(range.end_point)?;
        Ok(lsp_types::Range {
            start: lsp_types::Position {
                line: start.row as u32,
                character: start.column as u32,
            },
            end: lsp_types::Position {
                line: end.row as u32,
                character: end.column as u32,
            },
        })
    }

    /// Converts a tree-sitter [`tree_sitter::Point`] to an LSP [`lsp_types::Position`],
    /// adjusting columns to the LSP client encoding.
    /// Mirrors texter's own [`GridIndex::denormalize`].
    pub fn denormalize_point(&self, point: Point) -> Result<Point, DocumentError> {
        let mut grid = GridIndex::from(point);
        grid.denormalize(&self.texter)?;
        Ok(Point::from(grid))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::errors::TexterError;
    use lsp_types::{Position, PositionEncodingKind};
    use rstest::{fixture, rstest};
    use tree_sitter::Parser;

    /// Local test parameter so each `#[case(...)]` can drive both the encoding
    /// kind passed to `Document::new` and the encoding-specific expected values.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Encoding {
        UTF8,
        UTF16,
        UTF32,
    }

    impl Encoding {
        fn kind(self) -> PositionEncodingKind {
            match self {
                Encoding::UTF8 => PositionEncodingKind::UTF8,
                Encoding::UTF16 => PositionEncodingKind::UTF16,
                Encoding::UTF32 => PositionEncodingKind::UTF32,
            }
        }
    }

    #[fixture]
    fn parser() -> Parser {
        let mut p = Parser::new();
        p.set_language(&tree_sitter_html::LANGUAGE.into()).unwrap();
        p
    }

    #[rstest]
    fn normalize(mut parser: Parser) {
        // All-ASCII source, so encoded columns equal UTF-8 columns.
        let source = "Apples\nBashdjad\nashdkasdh\nasdsad";
        let document = Document::new(
            source.into(),
            parser.parse(source, None).unwrap(),
            Some(&PositionEncodingKind::UTF16),
        );

        assert_eq!(&document.texter.br_indexes.0, &[0, 6, 15, 25]);

        let normalized = |line, character| {
            let pos = Position { line, character };
            document.normalize_position(&pos).unwrap()
        };

        assert_eq!(
            normalized(0, 0),
            Position {
                line: 0,
                character: 0
            }
        );
        assert_eq!(
            normalized(0, 5),
            Position {
                line: 0,
                character: 5
            }
        );
        assert_eq!(
            normalized(1, 3),
            Position {
                line: 1,
                character: 3
            }
        );
        assert_eq!(
            normalized(3, 5),
            Position {
                line: 3,
                character: 5
            }
        );

        // Line out of bounds surfaces as a texter error wrapped in DocumentError.
        let oob = Position {
            line: 10,
            character: 0,
        };
        assert!(matches!(
            document.normalize_position(&oob),
            Err(DocumentError::Texter(TexterError::TexterError(
                texter::error::Error::OutOfBoundsRow { .. }
            )))
        ));

        // Column past the line length is clamped by texter to the end of the line ("Bashdjad").
        assert_eq!(
            normalized(1, 100),
            Position {
                line: 1,
                character: 8
            }
        );
    }

    #[rstest]
    #[case(Encoding::UTF8, 20)]
    #[case(Encoding::UTF16, 10)]
    #[case(Encoding::UTF32, 10)]
    fn ts_range_to_range_from_cst(
        mut parser: Parser,
        #[case] encoding: Encoding,
        #[case] end_character: u32,
    ) {
        let source = "<div>こんにちは</div>";
        let tree = parser.parse(source, None).unwrap();
        let document = Document::new(source.into(), tree.clone(), Some(&encoding.kind()));

        let element = tree.root_node().named_child(0).expect("element");
        let text_node = element.named_child(1).expect("text node");

        assert_eq!(
            document.denormalize_range(&text_node.range()).unwrap(),
            lsp_types::Range {
                start: Position {
                    line: 0,
                    character: 5,
                },
                end: Position {
                    line: 0,
                    character: end_character,
                },
            }
        );
    }
}
