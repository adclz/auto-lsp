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

// `texter` 0.3.0 made `GridIndex::normalize`/`denormalize` take `&Text` (instead of `&mut Text`),
// which finally lets us route all encoding-aware position conversions through texter without
// invalidating salsa queries. This module is now a thin wrapper around `Text` and `Tree`:
// what used to be a handful of manual UTF-8/16/32 loops and a duplicated `Encoding` enum is
// gone, and the public surface is reduced to what consumers actually call. Callers receive
// LSP positions in the negotiated encoding and never see `GridIndex` or normalization itself.

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

    /// Converts an LSP [`lsp_types::Position`] (in the negotiated encoding) to a byte offset.
    ///
    /// Uses texter's [`GridIndex::normalize`] to convert the encoded column to a UTF-8 column,
    /// then adds the row's start byte. An out-of-bounds row surfaces as a
    /// [`texter::error::Error::OutOfBoundsRow`] wrapped in [`DocumentError::Texter`].
    pub fn offset_at(&self, position: lsp_types::Position) -> Result<usize, DocumentError> {
        let mut grid = GridIndex {
            row: position.line as usize,
            col: position.character as usize,
        };
        grid.normalize(&self.texter)?;
        let row_start = self
            .texter
            .br_indexes
            .row_start(grid.row)
            .expect("row validated by normalize");
        Ok(row_start + grid.col)
    }

    /// Converts a tree-sitter [`tree_sitter::Range`] to an LSP [`lsp_types::Range`],
    /// adjusting columns to the document's negotiated encoding.
    pub fn ts_range_to_range(
        &self,
        range: &tree_sitter::Range,
    ) -> Result<lsp_types::Range, DocumentError> {
        let start = self.encoded_point(range.start_point)?;
        let end = self.encoded_point(range.end_point)?;
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

    /// Converts a tree-sitter [`tree_sitter::Range`] to another [`tree_sitter::Range`] with
    /// columns in the document's negotiated encoding. Byte offsets are preserved.
    pub fn ts_range_to_enc_range(
        &self,
        range: &tree_sitter::Range,
    ) -> Result<tree_sitter::Range, DocumentError> {
        let start_point = self.encoded_point(range.start_point)?;
        let end_point = self.encoded_point(range.end_point)?;
        Ok(tree_sitter::Range {
            start_byte: range.start_byte,
            end_byte: range.end_byte,
            start_point,
            end_point,
        })
    }

    fn encoded_point(&self, point: Point) -> Result<Point, DocumentError> {
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
    fn offset_at(mut parser: Parser) {
        let source = "Apples\nBashdjad\nashdkasdh\nasdsad";
        let document = Document::new(
            source.into(),
            parser.parse(source, None).unwrap(),
            Some(&PositionEncodingKind::UTF16),
        );

        assert_eq!(&document.texter.br_indexes.0, &[0, 6, 15, 25]);

        assert_eq!(
            document
                .offset_at(Position {
                    line: 0,
                    character: 0
                })
                .unwrap(),
            0
        );
        assert_eq!(
            document
                .offset_at(Position {
                    line: 0,
                    character: 5
                })
                .unwrap(),
            5
        );
        assert_eq!(
            document
                .offset_at(Position {
                    line: 1,
                    character: 3
                })
                .unwrap(),
            10
        );
        assert_eq!(
            document
                .offset_at(Position {
                    line: 3,
                    character: 5
                })
                .unwrap(),
            31
        );

        // Line out of bounds surfaces as a texter error wrapped in DocumentError.
        assert!(matches!(
            document.offset_at(Position {
                line: 10,
                character: 0
            }),
            Err(DocumentError::Texter(TexterError::TexterError(
                texter::error::Error::OutOfBoundsRow { .. }
            )))
        ));

        // Column past encoded line length is clamped by texter to the end of the line.
        assert_eq!(
            document
                .offset_at(Position {
                    line: 1,
                    character: 100
                })
                .unwrap(),
            15
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
            document.ts_range_to_range(&text_node.range()).unwrap(),
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

    #[rstest]
    #[case(Encoding::UTF8, 20)]
    #[case(Encoding::UTF16, 10)]
    #[case(Encoding::UTF32, 10)]
    fn ts_range_to_enc_range_from_cst(
        mut parser: Parser,
        #[case] encoding: Encoding,
        #[case] end_column: usize,
    ) {
        let source = "<div>こんにちは</div>\n<p>hello</p>";
        let tree = parser.parse(source, None).unwrap();
        let document = Document::new(source.into(), tree.clone(), Some(&encoding.kind()));

        let first_element = tree.root_node().named_child(0).expect("first element");
        let text_node = first_element.named_child(1).expect("text node");
        let original = text_node.range();

        let enc_range = document.ts_range_to_enc_range(&original).unwrap();

        // Byte offsets are preserved
        assert_eq!(enc_range.start_byte, original.start_byte);
        assert_eq!(enc_range.end_byte, original.end_byte);
        assert_eq!(enc_range.start_point, Point { row: 0, column: 5 });
        assert_eq!(
            enc_range.end_point,
            Point {
                row: 0,
                column: end_column,
            }
        );
    }
}
