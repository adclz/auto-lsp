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

use lsp_types::{Position, PositionEncodingKind};
use std::ops::Range;
use texter::{change::GridIndex, core::text::Text};
use texter_impl::{change::WrapChange, updateable::WrapTree};
use tree_sitter::{Point, Tree};

use crate::errors::{DocumentError, PositionError, TexterError, TreeSitterError};

pub(crate) mod texter_impl;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Encoding {
    UTF8,
    UTF16,
    UTF32,
}

/// Represents a text document that combines plain text [`texter`] with its parsed syntax tree [`tree_sitter::Tree`].
///
/// This struct allows for incremental updates of both the text content and the syntax tree,
/// ensuring they stay synchronized after each change. It provides utility functions for querying
/// the syntax tree, such as finding nodes by position or range.
#[derive(Debug, Clone)]
pub struct Document {
    pub texter: Text,
    pub tree: Tree,
    pub encoding: Encoding,
}

impl Document {
    /// Creates a new `Document` instance with the provided source, syntax tree, and encoding.
    ///
    /// Will default to UTF16 if the encoding is not specified or invalid.
    pub fn new(source: String, tree: Tree, encoding: Option<&PositionEncodingKind>) -> Self {
        let encoding = match encoding.as_ref() {
            Some(enc) => match enc.as_str() {
                "utf-8" => Encoding::UTF8,
                "utf-16" => Encoding::UTF16,
                "utf-32" => Encoding::UTF32,
                _ => Encoding::UTF16,
            },
            None => Encoding::UTF16,
        };

        Self {
            texter: match encoding {
                Encoding::UTF8 => Text::new(source),
                Encoding::UTF16 => Text::new_utf16(source),
                Encoding::UTF32 => Text::new_utf32(source),
            },
            tree,
            encoding,
        }
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
    /// This method applies the changes to both the text [`texter`] and the syntax tree [`Tree`].
    /// It uses incremental parsing to minimize the cost of updating the syntax tree.
    ///
    /// # Parameters
    /// - `parser`: A mutable reference to the Tree-sitter parser used to reparse the document.
    /// - `changes`: A vector of `TextDocumentContentChangeEvent` objects representing text changes.
    ///
    /// # Errors
    /// Returns an error if Tree-sitter fails to reparse the updated text
    pub fn update(
        &mut self,
        parser: &mut tree_sitter::Parser,
        changes: &[lsp_types::TextDocumentContentChangeEvent],
    ) -> Result<(), DocumentError> {
        let mut new_tree = WrapTree::from(&mut self.tree);

        for change in changes {
            self.texter
                .update(WrapChange::from(change).change, &mut new_tree)
                .map_err(|e| DocumentError::from(TexterError::from(e)))?;
        }

        self.tree = parser
            .parse(self.texter.text.as_bytes(), Some(&self.tree))
            .ok_or_else(|| DocumentError::from(TreeSitterError::TreeSitterParser))?;

        Ok(())
    }

    /// Converts a tree-sitter [`Point`] (byte-offset column) to an LSP [`lsp_types::Position`],
    /// adjusting the column to the document's encoding (UTF-8, UTF-16, or UTF-32).
    pub fn ts_point_to_position(&self, point: Point) -> Option<Position> {
        let enc = self.ts_point_to_enc_point(point)?;
        Some(lsp_types::Position {
            line: enc.row as u32,
            character: enc.column as u32,
        })
    }

    /// Converts a tree-sitter [`tree_sitter::Range`] to an LSP [`lsp_types::Range`],
    /// adjusting columns to the document's encoding.
    pub fn ts_range_to_range(&self, range: &tree_sitter::Range) -> Option<lsp_types::Range> {
        let start = self.ts_point_to_position(range.start_point)?;
        let end = self.ts_point_to_position(range.end_point)?;
        Some(lsp_types::Range { start, end })
    }

    /// Converts a tree-sitter [`Point`] (byte-offset column) to another [`Point`]
    /// with the column adjusted to the document's encoding.
    ///
    /// Uses texter's [`GridIndex::denormalize`] to convert from UTF-8 to the document's encoding.
    pub fn ts_point_to_enc_point(&self, point: Point) -> Option<Point> {
        // same as https://github.com/airblast-dev/texter/blob/2f4927d33682596e5d84d6497642d73ec2ed4eb3/src/change.rs#L70
        //
        let mut grid = GridIndex {
            row: point.row,
            col: point.column,
        };
        grid.denormalize(&self.texter).ok()?;
        Some(Point {
            row: grid.row,
            column: grid.col,
        })
    }

    /// Converts a tree-sitter [`tree_sitter::Range`] to another [`tree_sitter::Range`]
    /// with columns adjusted to the document's encoding. Byte offsets are preserved.
    pub fn ts_range_to_enc_range(&self, range: &tree_sitter::Range) -> Option<tree_sitter::Range> {
        let start_point = self.ts_point_to_enc_point(range.start_point)?;
        let end_point = self.ts_point_to_enc_point(range.end_point)?;
        Some(tree_sitter::Range {
            start_byte: range.start_byte,
            end_byte: range.end_byte,
            start_point,
            end_point,
        })
    }

    /// Converts an LSP [`lsp_types::Position`] (encoded character offset) to a tree-sitter
    /// [`Point`] (byte-offset column).
    pub fn lsp_position_to_ts_point(&self, position: lsp_types::Position) -> Option<Point> {
        let line_str = self.texter.get_row(position.line as usize)?;
        let target = position.character as usize;

        let mut u8_offset = 0;
        let mut encoded_offset = 0;

        for c in line_str.chars() {
            if encoded_offset >= target {
                break;
            }
            match self.encoding {
                Encoding::UTF8 => encoded_offset += c.len_utf8(),
                Encoding::UTF16 => encoded_offset += c.len_utf16(),
                Encoding::UTF32 => encoded_offset += 1,
            }
            u8_offset += c.len_utf8();
        }

        Some(Point {
            row: position.line as usize,
            column: u8_offset,
        })
    }

    /// Retrieves the smallest syntax node that spans the given position in the document.
    pub fn node_at_position(&self, position: lsp_types::Position) -> Option<tree_sitter::Node<'_>> {
        let point = self.lsp_position_to_ts_point(position)?;

        self.tree
            .root_node()
            .named_descendant_for_point_range(point, point)
    }

    /// Retrieves the range (start and end positions) of the smallest syntax node that spans the given byte offset.
    pub fn node_range_at(&self, offset: usize) -> Option<lsp_types::Range> {
        self.tree
            .root_node()
            .named_descendant_for_byte_range(offset, offset)
            .and_then(|node| self.ts_range_to_range(&node.range()))
    }

    /// Converts a byte offset in the document to its corresponding position (line and character).
    pub fn position_at(&self, offset: usize) -> Result<lsp_types::Position, PositionError> {
        let (line, start, end) = self
            .find_line(offset)
            .ok_or(PositionError::WrongPosition { offset })?;
        let line_str = self.texter.get_row(line).unwrap();

        if let Some(end) = end {
            if offset > end {
                return Err(PositionError::LineOutOfBound {
                    offset: offset,
                    length: end,
                });
            }
        }

        let target_u8_offset = offset.saturating_sub(start);

        let column = self.find_column_offset(line_str, target_u8_offset).ok_or(
            PositionError::LineOutOfBound {
                offset: target_u8_offset,
                length: line_str.len(),
            },
        )?;

        Ok(lsp_types::Position {
            line: line as u32,
            character: column as u32,
        })
    }

    /// Converts a byte offset in the document to its corresponding range (start and end positions).
    pub fn range_at(&self, range: Range<usize>) -> Result<lsp_types::Range, PositionError> {
        let start = self
            .position_at(range.start)
            .map_err(|err| PositionError::WrongRange {
                range: range.clone(),
                position_error: Box::new(err),
            })?;
        let end = self
            .position_at(range.end)
            .map_err(|err| PositionError::WrongRange {
                range: range.clone(),
                position_error: Box::new(err),
            })?;
        Ok(lsp_types::Range { start, end })
    }

    /// Converts a position (line and character) in the document to its corresponding byte offset.
    pub fn offset_at(&self, position: lsp_types::Position) -> Option<usize> {
        let line_index = self.texter.br_indexes.row_start(position.line as usize)?;
        let line_str = self.texter.get_row(position.line as usize)?;

        let target_u8_offset = position.character as usize;
        if target_u8_offset > line_str.len() {
            return None;
        }
        let byte_offset = self.find_column_offset(line_str, target_u8_offset)?;

        Some(line_index + byte_offset)
    }

    // 1 - line | 2 - start br index | 3 - end br index
    fn find_line(&self, offset: usize) -> Option<(usize, usize, Option<usize>)> {
        if self.texter.br_indexes.0.len() == 1 {
            return Some((0, 0, None));
        }

        for (line, &br_index) in self.texter.br_indexes.0.iter().enumerate() {
            if offset < br_index {
                let line = line.saturating_sub(1);
                // +1 to skip the breakline (EOL indexes only mark the end of the line, not the start of next line)
                return Some((line, self.texter.br_indexes.0[line] + 1, Some(br_index)));
            }
        }
        None
    }

    fn find_column_offset(&self, line_str: &str, target_u8_offset: usize) -> Option<usize> {
        let mut u8_offset: usize = 0;
        let mut u16_offset: usize = 0;
        let mut u32_offset: usize = 0;
        let mut found = false;

        for c in line_str.chars() {
            if u8_offset >= target_u8_offset {
                found = true;
                break;
            } else {
                u8_offset += c.len_utf8();
                u16_offset += c.len_utf16();
                u32_offset += 1;
            }
        }

        if !found && u8_offset == target_u8_offset {
            found = true;
        }

        match found {
            true => match self.encoding {
                Encoding::UTF8 => Some(u8_offset),
                Encoding::UTF16 => Some(u16_offset),
                Encoding::UTF32 => Some(u32_offset),
            },
            false => return None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lsp_types::{Position, PositionEncodingKind};
    use rstest::{fixture, rstest};
    use tree_sitter::Parser;

    #[fixture]
    fn parser() -> Parser {
        let mut p = Parser::new();
        p.set_language(&tree_sitter_html::LANGUAGE.into()).unwrap();
        p
    }

    #[rstest]
    #[case(Encoding::UTF8)]
    #[case(Encoding::UTF16)]
    #[case(Encoding::UTF32)]
    fn position_at(mut parser: Parser, #[case] encoding: Encoding) {
        let source = "<div>こんにちは\nGoodbye\r\nSee 👨‍👨‍👧 you!\n</div>";
        let document = Document::new(
            source.into(),
            parser.parse(source, None).unwrap(),
            Some(&match encoding {
                Encoding::UTF8 => PositionEncodingKind::UTF8,
                Encoding::UTF16 => PositionEncodingKind::UTF16,
                Encoding::UTF32 => PositionEncodingKind::UTF32,
            }),
        );

        assert_eq!(&document.texter.br_indexes.0, &[0, 20, 29, 57]);

        assert_eq!(
            document.position_at(0).unwrap(),
            Position {
                line: 0,
                character: 0
            }
        );

        // Offset 11 is inside the Japanese text "こんにちは"
        assert_eq!(
            document.position_at(11).unwrap(),
            Position {
                line: 0,
                character: match encoding {
                    Encoding::UTF8 => 11,
                    Encoding::UTF16 => 7,
                    Encoding::UTF32 => 7,
                }
            }
        );

        // Offset 21 is at the beginning of "Goodbye" (after '\n')
        assert_eq!(
            document.position_at(21).unwrap(),
            Position {
                line: 1,
                character: 0
            }
        );

        // Offset 28 is in "Goodbye" (before '\r')
        assert_eq!(
            document.position_at(28).unwrap(),
            Position {
                line: 1,
                character: 7
            }
        );

        // Offset 30 is the last byte of "\r\n", meaning we move to the next line
        assert_eq!(
            document.position_at(30).unwrap(),
            Position {
                line: 2,
                character: 0
            }
        );

        assert_eq!(
            document.position_at(52).unwrap(),
            Position {
                line: 2,
                character: match encoding {
                    Encoding::UTF8 => 22,
                    Encoding::UTF16 => 12,
                    Encoding::UTF32 => 9,
                }
            }
        );

        // Offset 59 is out of bounds
        assert_eq!(
            document.position_at(59),
            Err(PositionError::WrongPosition { offset: 59 })
        );
    }

    #[rstest]
    #[case(Encoding::UTF8)]
    #[case(Encoding::UTF16)]
    #[case(Encoding::UTF32)]
    fn position_at_single_line(mut parser: Parser, #[case] encoding: Encoding) {
        let source = "<div>✨✨✨AREALLYREALLYREALLYLONGTEXT<div>";
        let document = Document::new(
            source.into(),
            parser.parse(source, None).unwrap(),
            Some(&match encoding {
                Encoding::UTF8 => PositionEncodingKind::UTF8,
                Encoding::UTF16 => PositionEncodingKind::UTF16,
                Encoding::UTF32 => PositionEncodingKind::UTF32,
            }),
        );

        assert_eq!(&document.texter.br_indexes.0, &[0]);

        assert_eq!(
            document.position_at(0).unwrap(),
            Position {
                line: 0,
                character: 0
            }
        );

        assert_eq!(
            document.position_at(5).unwrap(),
            Position {
                line: 0,
                character: 5
            }
        );

        assert_eq!(
            document.position_at(30).unwrap(),
            Position {
                line: 0,
                character: match encoding {
                    Encoding::UTF8 => 30,
                    Encoding::UTF16 => 24,
                    Encoding::UTF32 => 24,
                }
            }
        );
    }

    #[rstest]
    #[case(Encoding::UTF8)]
    #[case(Encoding::UTF16)]
    #[case(Encoding::UTF32)]
    fn range_at(mut parser: Parser, #[case] encoding: Encoding) {
        let source = "<div>こんにちは\n❤️Goodbye\r\nSee you!\n</div>";
        let document = Document::new(
            source.into(),
            parser.parse(source, None).unwrap(),
            Some(&match encoding {
                Encoding::UTF8 => PositionEncodingKind::UTF8,
                Encoding::UTF16 => PositionEncodingKind::UTF16,
                Encoding::UTF32 => PositionEncodingKind::UTF32,
            }),
        );

        assert_eq!(&document.texter.br_indexes.0, &[0, 20, 35, 44]);

        // Test range covering part of first line
        assert_eq!(
            document.range_at(0..11).unwrap(),
            lsp_types::Range {
                start: Position {
                    line: 0,
                    character: 0
                },
                end: Position {
                    line: 0,
                    character: match encoding {
                        Encoding::UTF8 => 11,
                        Encoding::UTF16 => 7,
                        Encoding::UTF32 => 7,
                    }
                },
            }
        );

        // Test range spanning multiple lines
        assert_eq!(
            document.range_at(14..28).unwrap(),
            lsp_types::Range {
                start: Position {
                    line: 0,
                    character: match encoding {
                        Encoding::UTF8 => 14,
                        Encoding::UTF16 => 8,
                        Encoding::UTF32 => 8,
                    }
                },
                end: Position {
                    line: 1,
                    character: match encoding {
                        Encoding::UTF8 => 7,
                        Encoding::UTF16 => 3,
                        Encoding::UTF32 => 3,
                    }
                },
            }
        );

        // Test range from start of a line to another
        assert_eq!(
            document.range_at(20..30).unwrap(),
            lsp_types::Range {
                start: Position {
                    line: 1,
                    character: 0
                },
                end: Position {
                    line: 1,
                    character: match encoding {
                        Encoding::UTF8 => 9,
                        Encoding::UTF16 => 5,
                        Encoding::UTF32 => 5,
                    }
                },
            }
        );

        // Test range entirely in one line
        assert_eq!(
            document.range_at(20..35).unwrap(),
            lsp_types::Range {
                start: Position {
                    line: 1,
                    character: 0
                },
                end: Position {
                    line: 2,
                    character: 0
                },
            }
        );

        // Test out-of-bounds range
        assert_eq!(
            document.range_at(35..50),
            Err(PositionError::WrongRange {
                range: 35..50,
                position_error: Box::new(PositionError::WrongPosition { offset: 50 })
            })
        );
    }

    #[rstest]
    fn range_at_single_line(mut parser: Parser) {
        let source = "<div>AREALLYREALLYREALLYLONGTEXT<div>";
        let document = Document::new(
            source.into(),
            parser.parse(source, None).unwrap(),
            Some(&PositionEncodingKind::UTF8),
        );

        // Ensure the line break indexes are correct
        assert_eq!(&document.texter.br_indexes.0, &[0]);

        // Check range from start to some offset
        assert_eq!(
            document.range_at(0..5).unwrap(),
            lsp_types::Range {
                start: Position {
                    line: 0,
                    character: 0
                },
                end: Position {
                    line: 0,
                    character: 5
                }
            }
        );

        // Check range covering the entire line
        let length = source.len();
        assert_eq!(
            document.range_at(0..length).unwrap(),
            lsp_types::Range {
                start: Position {
                    line: 0,
                    character: 0
                },
                end: Position {
                    line: 0,
                    character: length as u32
                }
            }
        );

        // Out-of-bounds check
        assert_eq!(
            document.range_at(0..(length + 5)),
            Err(PositionError::WrongRange {
                range: 0..(length + 5),
                position_error: Box::new(PositionError::LineOutOfBound {
                    offset: 42,
                    length: 37
                })
            })
        );
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

        // Test for start of first line
        assert_eq!(
            document.offset_at(Position {
                line: 0,
                character: 0
            }),
            Some(0)
        );

        // Test for char at first line
        assert_eq!(
            document.offset_at(Position {
                line: 0,
                character: 5
            }),
            Some(5)
        );

        // Test for middle of second line (after "Bash")
        assert_eq!(
            document.offset_at(Position {
                line: 1,
                character: 3
            }),
            Some(10)
        );

        // Test for end of last line
        assert_eq!(
            document.offset_at(Position {
                line: 3,
                character: 5
            }),
            Some(31)
        );

        // Test for out of bounds position (line too high)
        assert_eq!(
            document.offset_at(Position {
                line: 10,
                character: 0
            }),
            None
        );

        // Test for out of bounds position (column too high)
        assert_eq!(
            document.offset_at(Position {
                line: 1,
                character: 100
            }),
            None
        );
    }

    #[rstest]
    #[case(Encoding::UTF8)]
    #[case(Encoding::UTF16)]
    #[case(Encoding::UTF32)]
    fn ts_point_to_position_from_cst(mut parser: Parser, #[case] encoding: Encoding) {
        // HTML with multi-byte content: tree-sitter reports byte-offset columns
        // "<div>" = 5 bytes, "こんにちは" = 15 bytes (5 chars × 3), "</div>" = 6 bytes
        let source = "<div>こんにちは</div>\n<p>hello</p>";
        let tree = parser.parse(source, None).unwrap();
        let document = Document::new(
            source.into(),
            tree.clone(),
            Some(&match encoding {
                Encoding::UTF8 => PositionEncodingKind::UTF8,
                Encoding::UTF16 => PositionEncodingKind::UTF16,
                Encoding::UTF32 => PositionEncodingKind::UTF32,
            }),
        );

        let root = tree.root_node();
        // fragment > element > text (the "こんにちは" node)
        let first_element = root.named_child(0).expect("first element");
        let text_node = first_element.named_child(1).expect("text node");

        // tree-sitter reports: start (0, 5) end (0, 20) — byte offsets
        assert_eq!(text_node.start_position(), Point { row: 0, column: 5 });
        assert_eq!(text_node.end_position(), Point { row: 0, column: 20 });

        // After encoding adjustment, the LSP positions differ
        let start = document
            .ts_point_to_position(text_node.start_position())
            .unwrap();
        let end = document
            .ts_point_to_position(text_node.end_position())
            .unwrap();

        // Start is after "<div>" — pure ASCII, same for all encodings
        assert_eq!(
            start,
            Position {
                line: 0,
                character: 5
            }
        );

        // End is after "こんにちは" — encoding matters
        assert_eq!(
            end,
            Position {
                line: 0,
                character: match encoding {
                    Encoding::UTF8 => 20,  // 5 + 15 bytes
                    Encoding::UTF16 => 10, // 5 + 5 code units
                    Encoding::UTF32 => 10, // 5 + 5 code points
                }
            }
        );

        // The </div> end_tag starts right after the text
        let end_tag = first_element.named_child(2).expect("end tag");
        assert_eq!(end_tag.start_position(), Point { row: 0, column: 20 });
        assert_eq!(end_tag.end_position(), Point { row: 0, column: 26 });

        let end_tag_start = document
            .ts_point_to_position(end_tag.start_position())
            .unwrap();
        let end_tag_end = document
            .ts_point_to_position(end_tag.end_position())
            .unwrap();

        assert_eq!(
            end_tag_start,
            Position {
                line: 0,
                character: match encoding {
                    Encoding::UTF8 => 20,
                    Encoding::UTF16 => 10,
                    Encoding::UTF32 => 10,
                }
            }
        );
        assert_eq!(
            end_tag_end,
            Position {
                line: 0,
                character: match encoding {
                    Encoding::UTF8 => 26,
                    Encoding::UTF16 => 16,
                    Encoding::UTF32 => 16,
                }
            }
        );

        // Second line: pure ASCII "<p>hello</p>" — all encodings agree
        let second_element = root.named_child(1).expect("second element");
        let hello_text = second_element.named_child(1).expect("hello text");
        let hello_start = document
            .ts_point_to_position(hello_text.start_position())
            .unwrap();
        let hello_end = document
            .ts_point_to_position(hello_text.end_position())
            .unwrap();

        assert_eq!(
            hello_start,
            Position {
                line: 1,
                character: 3
            }
        );
        assert_eq!(
            hello_end,
            Position {
                line: 1,
                character: 8
            }
        );
    }

    #[rstest]
    #[case(Encoding::UTF8)]
    #[case(Encoding::UTF16)]
    #[case(Encoding::UTF32)]
    fn ts_range_to_range_from_cst(mut parser: Parser, #[case] encoding: Encoding) {
        let source = "<div>こんにちは</div>";
        let tree = parser.parse(source, None).unwrap();
        let document = Document::new(
            source.into(),
            tree.clone(),
            Some(&match encoding {
                Encoding::UTF8 => PositionEncodingKind::UTF8,
                Encoding::UTF16 => PositionEncodingKind::UTF16,
                Encoding::UTF32 => PositionEncodingKind::UTF32,
            }),
        );

        let root = tree.root_node();
        let element = root.named_child(0).expect("element");
        let text_node = element.named_child(1).expect("text node");

        let lsp_range = document
            .ts_range_to_range(&text_node.range())
            .expect("range conversion");

        assert_eq!(
            lsp_range,
            lsp_types::Range {
                start: Position {
                    line: 0,
                    character: 5
                },
                end: Position {
                    line: 0,
                    character: match encoding {
                        Encoding::UTF8 => 20,
                        Encoding::UTF16 => 10,
                        Encoding::UTF32 => 10,
                    }
                },
            }
        );
    }

    #[rstest]
    #[case(Encoding::UTF8)]
    #[case(Encoding::UTF16)]
    #[case(Encoding::UTF32)]
    fn cst_node_roundtrip(mut parser: Parser, #[case] encoding: Encoding) {
        let source = "<div>こんにちは</div>\n<p>hello</p>";
        let tree = parser.parse(source, None).unwrap();
        let document = Document::new(
            source.into(),
            tree.clone(),
            Some(&match encoding {
                Encoding::UTF8 => PositionEncodingKind::UTF8,
                Encoding::UTF16 => PositionEncodingKind::UTF16,
                Encoding::UTF32 => PositionEncodingKind::UTF32,
            }),
        );

        // Collect all named nodes from the CST
        let root = tree.root_node();
        let mut cursor = root.walk();
        let nodes: Vec<_> = root.named_children(&mut cursor).collect();

        for node in &nodes {
            let start = node.start_position();
            let end = node.end_position();

            // ts_point → lsp_position → ts_point should be identity for all CST nodes
            let lsp_start = document.ts_point_to_position(start).unwrap();
            let back_start = document.lsp_position_to_ts_point(lsp_start).unwrap();
            assert_eq!(
                back_start,
                start,
                "start roundtrip failed for {:?} (encoding: {encoding:?})",
                node.kind()
            );

            let lsp_end = document.ts_point_to_position(end).unwrap();
            let back_end = document.lsp_position_to_ts_point(lsp_end).unwrap();
            assert_eq!(
                back_end,
                end,
                "end roundtrip failed for {:?} (encoding: {encoding:?})",
                node.kind()
            );
        }
    }

    #[rstest]
    #[case(Encoding::UTF8)]
    #[case(Encoding::UTF16)]
    #[case(Encoding::UTF32)]
    fn node_at_position_with_encoding(mut parser: Parser, #[case] encoding: Encoding) {
        let source = "<div>こんにちは</div>";
        let document = Document::new(
            source.into(),
            parser.parse(source, None).unwrap(),
            Some(&match encoding {
                Encoding::UTF8 => PositionEncodingKind::UTF8,
                Encoding::UTF16 => PositionEncodingKind::UTF16,
                Encoding::UTF32 => PositionEncodingKind::UTF32,
            }),
        );

        // Query the text node "こんにちは" using an LSP position in the middle of it
        let mid_character = match encoding {
            Encoding::UTF8 => 8,  // byte offset of "に"
            Encoding::UTF16 => 7, // "こん" = 2 code units + 5 ("<div>") = 7
            Encoding::UTF32 => 7, // same as UTF-16 for BMP chars
        };

        let node = document
            .node_at_position(Position {
                line: 0,
                character: mid_character,
            })
            .expect("should find a node");

        // The node encompassing that position is the text content
        assert_eq!(node.kind(), "text");
    }

    #[rstest]
    #[case(Encoding::UTF8)]
    #[case(Encoding::UTF16)]
    #[case(Encoding::UTF32)]
    fn ts_range_to_enc_range_from_cst(mut parser: Parser, #[case] encoding: Encoding) {
        // "<div>" = 5 bytes, "こんにちは" = 15 bytes (5 chars × 3), "</div>" = 6 bytes
        let source = "<div>こんにちは</div>\n<p>hello</p>";
        let tree = parser.parse(source, None).unwrap();
        let document = Document::new(
            source.into(),
            tree.clone(),
            Some(&match encoding {
                Encoding::UTF8 => PositionEncodingKind::UTF8,
                Encoding::UTF16 => PositionEncodingKind::UTF16,
                Encoding::UTF32 => PositionEncodingKind::UTF32,
            }),
        );

        let root = tree.root_node();
        let first_element = root.named_child(0).expect("first element");
        let text_node = first_element.named_child(1).expect("text node");

        // Original ts range: start (0, 5) end (0, 20) — byte offsets
        let original = text_node.range();
        assert_eq!(original.start_point, Point { row: 0, column: 5 });
        assert_eq!(original.end_point, Point { row: 0, column: 20 });

        let enc_range = document
            .ts_range_to_enc_range(&original)
            .expect("enc range conversion");

        // Byte offsets are preserved
        assert_eq!(enc_range.start_byte, original.start_byte);
        assert_eq!(enc_range.end_byte, original.end_byte);

        // Start column: after "<div>" — pure ASCII, same for all encodings
        assert_eq!(enc_range.start_point, Point { row: 0, column: 5 });

        // End column: after "こんにちは" — encoding matters
        assert_eq!(
            enc_range.end_point,
            Point {
                row: 0,
                column: match encoding {
                    Encoding::UTF8 => 20,  // 5 + 15 bytes
                    Encoding::UTF16 => 10, // 5 + 5 code units
                    Encoding::UTF32 => 10, // 5 + 5 code points
                }
            }
        );

        // Second line: pure ASCII — all encodings agree
        let second_element = root.named_child(1).expect("second element");
        let hello_text = second_element.named_child(1).expect("hello text");
        let hello_enc = document
            .ts_range_to_enc_range(&hello_text.range())
            .expect("hello enc range");

        assert_eq!(hello_enc.start_point, Point { row: 1, column: 3 });
        assert_eq!(hello_enc.end_point, Point { row: 1, column: 8 });
        assert_eq!(hello_enc.start_byte, hello_text.range().start_byte);
        assert_eq!(hello_enc.end_byte, hello_text.range().end_byte);
    }

    #[rstest]
    #[case(Encoding::UTF8)]
    #[case(Encoding::UTF16)]
    #[case(Encoding::UTF32)]
    fn ts_point_to_enc_point_from_cst(mut parser: Parser, #[case] encoding: Encoding) {
        let source = "<div>こんにちは</div>";
        let tree = parser.parse(source, None).unwrap();
        let document = Document::new(
            source.into(),
            tree.clone(),
            Some(&match encoding {
                Encoding::UTF8 => PositionEncodingKind::UTF8,
                Encoding::UTF16 => PositionEncodingKind::UTF16,
                Encoding::UTF32 => PositionEncodingKind::UTF32,
            }),
        );

        let root = tree.root_node();
        let element = root.named_child(0).expect("element");
        let end_tag = element.named_child(2).expect("end tag");

        // end_tag starts at byte offset 20 (after "こんにちは")
        let enc_point = document
            .ts_point_to_enc_point(end_tag.start_position())
            .expect("enc point conversion");

        assert_eq!(enc_point.row, 0);
        assert_eq!(
            enc_point.column,
            match encoding {
                Encoding::UTF8 => 20,
                Encoding::UTF16 => 10,
                Encoding::UTF32 => 10,
            }
        );
    }
}
