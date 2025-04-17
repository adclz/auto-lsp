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

use std::{
    ops::Range,
    sync::atomic::{AtomicUsize, Ordering},
};
use texter::core::text::Text;
use texter_impl::{change::WrapChange, updateable::WrapTree};
use tree_sitter::{Point, Tree};

use crate::errors::DocumentError;

pub(crate) mod texter_impl;

/// Represents a text document that combines plain text [`texter`] with its parsed syntax tree [`tree_sitter::Tree`].
///
/// This struct allows for incremental updates of both the text content and the syntax tree,
/// ensuring they stay synchronized after each change. It provides utility functions for querying
/// the syntax tree, such as finding nodes by position or range.
#[derive(Debug, Clone)]
pub struct Document {
    pub texter: Text,
    pub tree: Tree,
}

thread_local! {
    /// Thread-local storage for the last line index accessed.
    ///
    /// It is initialized to 0, indicating that no lines have been accessed yet.
    /// This is a performance optimization to avoid searching from the beginning of the document
    /// every time we need to find a position.
    /// The value is updated whenever a position is found, so that subsequent calls can start from
    /// the last accessed line.
    /// If the offset is greater than value, we reset the counter to 0.

    pub static LAST_LINE: AtomicUsize = const { AtomicUsize::new(0) };
}

impl Document {
    pub fn new(texter: Text, tree: Tree) -> Self {
        Self { texter, tree }
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
    ) -> anyhow::Result<()> {
        let mut new_tree = WrapTree::from(&mut self.tree);

        for change in changes {
            self.texter
                .update(WrapChange::from(change).change, &mut new_tree)?;
        }

        self.tree = parser
            .parse(self.texter.text.as_bytes(), Some(&self.tree))
            .ok_or(anyhow::format_err!("Tree sitter failed to edit tree",))?;

        Ok(())
    }

    /// Retrieves the smallest syntax node that spans the given position in the document.
    pub fn node_at_position(&self, position: lsp_types::Position) -> Option<tree_sitter::Node<'_>> {
        let position = Point {
            row: position.line as usize,
            column: position.character as usize,
        };

        self.tree
            .root_node()
            .named_descendant_for_point_range(position, position)
    }

    /// Retrieves the range (start and end positions) of the smallest syntax node that spans the given byte offset.
    pub fn node_range_at(&self, offset: usize) -> Option<lsp_types::Range> {
        self.tree
            .root_node()
            .named_descendant_for_byte_range(offset, offset)
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

    /// Converts a byte offset in the document to its corresponding position (line and character).
    pub fn position_at(&self, offset: usize) -> Result<lsp_types::Position, DocumentError> {
        let mut last_br_index = 0;
        let last_line = LAST_LINE.with(|a| a.load(Ordering::SeqCst));

        // If the document is a single line, we can avoid the loop
        if self.texter.br_indexes.0.len() == 1 {
            return if offset > self.texter.text.len() {
                Err(DocumentError::DocumentLineOutOfBound {
                    offset,
                    length: self.texter.text.len(),
                })
            } else {
                Ok(lsp_types::Position {
                    line: 0,
                    character: offset as u32,
                })
            };
        }

        // Determine the starting line for the search
        let start = match self.texter.br_indexes.0.get(last_line) {
            Some(&br_index) if offset > br_index && last_line >= 1 => last_line, // Start from cached line if offset is beyond it
            _ => 1, // Start from at least index 1 to avoid incorrect 0 offset issues
        };

        for (i, &br_index) in self.texter.br_indexes.0.iter().skip(start).enumerate() {
            if offset <= br_index {
                // Cache this line for future calls
                LAST_LINE.with(|a| a.store(i + (start - 1), Ordering::Release));

                // Compute column by subtracting the last break index
                let col = offset.saturating_sub(last_br_index);

                return Ok(lsp_types::Position {
                    line: (i + (start - 1)) as u32,
                    character: col as u32,
                });
            }

            last_br_index = br_index + 1; // Move past the EOL character
        }

        if offset <= self.texter.text.len() {
            let last_known_col = self.texter.br_indexes.0.iter().len();
            let last_br = *self.texter.br_indexes.0.last().unwrap();
            Ok(lsp_types::Position {
                line: last_known_col.saturating_sub(1) as u32,
                character: offset.saturating_sub(last_br) as u32,
            })
        } else {
            Err(DocumentError::DocumentPosition { offset })
        }
    }

    /// Converts a byte offset in the document to its corresponding range (start and end positions).
    pub fn range_at(&self, range: Range<usize>) -> Result<lsp_types::Range, DocumentError> {
        let start = self
            .position_at(range.start)
            .map_err(|err| DocumentError::DocumentRange {
                range: range.clone(),
                position_error: Box::new(err),
            })?;
        let end = self
            .position_at(range.end)
            .map_err(|err| DocumentError::DocumentRange {
                range: range.clone(),
                position_error: Box::new(err),
            })?;
        Ok(lsp_types::Range { start, end })
    }

    /// Converts a position (line and character) in the document to its corresponding byte offset.
    pub fn offset_at(&self, position: lsp_types::Position) -> Option<usize> {
        let line_index = self.texter.br_indexes.row_start(position.line as usize)?;
        let line_str = self.texter.get_row(position.line as usize)?;
        let col = position.character as usize;
        if col > line_str.len() {
            None
        } else {
            Some(line_index + col)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lsp_types::Position;
    use rstest::{fixture, rstest};
    use tree_sitter::Parser;

    #[fixture]
    fn parser() -> Parser {
        let mut p = Parser::new();
        p.set_language(&tree_sitter_html::LANGUAGE.into()).unwrap();
        p
    }

    fn get_last_line() -> usize {
        use crate::document::LAST_LINE; // adjust path if needed
        use std::sync::atomic::Ordering;

        LAST_LINE.with(|val| val.load(Ordering::Acquire))
    }

    #[rstest]
    fn position_at(mut parser: Parser) {
        let source = "<div>こんにちは\nGoodbye\r\nSee you!\n</div>";
        let text = Text::new(source.into());
        let document = Document::new(text, parser.parse(source, None).unwrap());

        assert_eq!(&document.texter.br_indexes.0, &[0, 20, 29, 38]);

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
                character: 11
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

        // Offset 40 is at the last line at pos 2
        assert_eq!(
            document.position_at(40).unwrap(),
            Position {
                line: 3,
                character: 2
            }
        );
    }

    #[rstest]
    fn position_at_single_line(mut parser: Parser) {
        let source = "<div>AREALLYREALLYREALLYLONGTEXT<div>";
        let text = Text::new(source.into());
        let document = Document::new(text, parser.parse(source, None).unwrap());

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
                character: 30
            }
        );
    }

    #[rstest]
    fn range_at(mut parser: Parser) {
        let source = "<div>こんにちは\nGoodbye\r\nSee you!\n</div>";
        let text = Text::new(source.into());
        let document = Document::new(text, parser.parse(source, None).unwrap());

        assert_eq!(&document.texter.br_indexes.0, &[0, 20, 29, 38]);

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
                    character: 11
                },
            }
        );

        // Test range spanning multiple lines
        assert_eq!(
            document.range_at(15..28).unwrap(),
            lsp_types::Range {
                start: Position {
                    line: 0,
                    character: 15
                },
                end: Position {
                    line: 1,
                    character: 7
                },
            }
        );

        // Test range from start of a line to another
        assert_eq!(
            document.range_at(21..30).unwrap(),
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

        // Test range entirely in one line
        assert_eq!(
            document.range_at(30..35).unwrap(),
            lsp_types::Range {
                start: Position {
                    line: 2,
                    character: 0
                },
                end: Position {
                    line: 2,
                    character: 5
                },
            }
        );

        // Test out-of-bounds range
        assert_eq!(
            document.range_at(35..50),
            Err(DocumentError::DocumentRange {
                range: 35..50,
                position_error: Box::new(DocumentError::DocumentPosition { offset: 50 })
            })
        );
    }

    #[rstest]
    fn range_at_single_line(mut parser: Parser) {
        let source = "<div>AREALLYREALLYREALLYLONGTEXT<div>";
        let text = Text::new(source.into());
        let document = Document::new(text, parser.parse(source, None).unwrap());

        assert_eq!(&document.texter.br_indexes.0, &[0]);

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
            Err(DocumentError::DocumentRange {
                range: 0..(length + 5),
                position_error: Box::new(DocumentError::DocumentLineOutOfBound {
                    offset: 42,
                    length: 37
                })
            })
        );
    }

    #[rstest]
    fn offset_at(mut parser: Parser) {
        let source = "Apples\nBashdjad\nashdkasdh\nasdsad";
        let text = Text::new(source.into());
        let document = Document::new(text, parser.parse(source, None).unwrap());

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
    fn line_tracking(mut parser: Parser) {
        let source = "one\nline two\nline three\n";
        let text = Text::new(source.into());
        let document = Document::new(text, parser.parse(source, None).unwrap());

        // Offset in line 0
        let pos1 = document.position_at(2).unwrap();
        assert_eq!(pos1.line, 0);
        assert_eq!(get_last_line(), 0);

        // Offset in line 1
        let pos2 = document.position_at(6).unwrap();
        assert_eq!(pos2.line, 1);
        assert_eq!(get_last_line(), 1);

        // Offset in line 2
        let pos3 = document.position_at(18).unwrap();
        assert_eq!(pos3.line, 2);
        assert_eq!(get_last_line(), 2);

        // Offset is ine line 0
        // This should reset the last line index
        let pos3 = document.position_at(0).unwrap();
        assert_eq!(pos3.line, 0);
        assert_eq!(get_last_line(), 0);
    }
}
