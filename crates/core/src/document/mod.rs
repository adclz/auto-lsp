use std::{
    ops::Range,
    sync::atomic::{AtomicUsize, Ordering},
};
use texter::core::text::Text;
use texter_impl::{change::WrapChange, updateable::WrapTree};
use tree_sitter::{Point, Tree};

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

pub static LAST_LINE: AtomicUsize = AtomicUsize::new(0);

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
    pub fn position_at(&self, offset: usize) -> Option<lsp_types::Position> {
        let mut last_br_index = 0;
        let last_line = LAST_LINE.load(Ordering::SeqCst);

        // If the document is a single line, we can avoid the loop
        if self.texter.br_indexes.0.len() == 1 {
            return if offset > self.texter.text.len() {
                None
            } else {
                Some(lsp_types::Position {
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
                LAST_LINE.store(i + (start - 1), Ordering::Release);

                // Compute column by subtracting the last break index
                let col = offset.saturating_sub(last_br_index);

                return Some(lsp_types::Position {
                    line: (i + (start - 1)) as u32,
                    character: col as u32,
                });
            }

            last_br_index = br_index + 1; // Move past the EOL character
        }

        if offset <= self.texter.text.len() {
            let last_known_col = self.texter.br_indexes.0.iter().len();
            let last_br = *self.texter.br_indexes.0.last().unwrap();
            Some(lsp_types::Position {
                line: last_known_col as u32,
                character: offset.saturating_sub(last_br) as u32,
            })
        } else {
            None
        }
    }

    /// Converts a byte offset in the document to its corresponding range (start and end positions).
    pub fn range_at(&self, range: Range<usize>) -> Option<lsp_types::Range> {
        let start = self.position_at(range.start)?;
        let end = self.position_at(range.end)?;
        Some(lsp_types::Range { start, end })
    }

    /// Converts a position (line and character) in the document to its corresponding byte offset.
    pub fn offset_at(&self, position: lsp_types::Position) -> Option<usize> {
        let line = self.texter.br_indexes.row_start(position.line as usize)?;
        let col = position.character as usize;
        if col > line {
            None
        } else {
            Some(line + col)
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

    #[rstest]
    fn position_at(mut parser: Parser) {
        let source = "<div>こんにちは\nGoodbye\r\nSee you!\n</div>";
        let text = Text::new(source.into());
        let document = Document::new(text, parser.parse(source, None).unwrap());

        assert_eq!(&document.texter.br_indexes.0, &[0, 20, 29, 38]);

        assert_eq!(
            document.position_at(0),
            Some(Position {
                line: 0,
                character: 0
            })
        );

        // Offset 11 is inside the Japanese text "こんにちは"
        assert_eq!(
            document.position_at(11),
            Some(Position {
                line: 0,
                character: 11
            })
        );

        // Offset 21 is at the beginning of "Goodbye" (after '\n')
        assert_eq!(
            document.position_at(21),
            Some(Position {
                line: 1,
                character: 0
            })
        );

        // Offset 28 is in "Goodbye" (before '\r')
        assert_eq!(
            document.position_at(28),
            Some(Position {
                line: 1,
                character: 7
            })
        );

        // Offset 30 is the last byte of "\r\n", meaning we move to the next line
        assert_eq!(
            document.position_at(30),
            Some(Position {
                line: 2,
                character: 0
            })
        );

        // Offset 40 is at the last line at pos 2
        assert_eq!(
            document.position_at(40),
            Some(Position {
                line: 4,
                character: 2
            })
        );
    }

    #[rstest]
    fn position_at_single_line(mut parser: Parser) {
        let source = "<div>AREALLYREALLYREALLYLONGTEXT<div>";
        let text = Text::new(source.into());
        let document = Document::new(text, parser.parse(source, None).unwrap());

        assert_eq!(&document.texter.br_indexes.0, &[0]);

        assert_eq!(
            document.position_at(0),
            Some(Position {
                line: 0,
                character: 0
            })
        );
        assert_eq!(
            document.position_at(5),
            Some(Position {
                line: 0,
                character: 5
            })
        );
        assert_eq!(
            document.position_at(30),
            Some(Position {
                line: 0,
                character: 30
            })
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
            document.range_at(0..11),
            Some(lsp_types::Range {
                start: Position {
                    line: 0,
                    character: 0
                },
                end: Position {
                    line: 0,
                    character: 11
                },
            })
        );

        // Test range spanning multiple lines
        assert_eq!(
            document.range_at(15..28),
            Some(lsp_types::Range {
                start: Position {
                    line: 0,
                    character: 15
                },
                end: Position {
                    line: 1,
                    character: 7
                },
            })
        );

        // Test range from start of a line to another
        assert_eq!(
            document.range_at(21..30),
            Some(lsp_types::Range {
                start: Position {
                    line: 1,
                    character: 0
                },
                end: Position {
                    line: 2,
                    character: 0
                },
            })
        );

        // Test range entirely in one line
        assert_eq!(
            document.range_at(30..35),
            Some(lsp_types::Range {
                start: Position {
                    line: 2,
                    character: 0
                },
                end: Position {
                    line: 2,
                    character: 5
                },
            })
        );

        // Test out-of-bounds range
        assert_eq!(document.range_at(35..50), None);
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
            document.range_at(0..5),
            Some(lsp_types::Range {
                start: Position {
                    line: 0,
                    character: 0
                },
                end: Position {
                    line: 0,
                    character: 5
                }
            })
        );

        // Check range covering the entire line
        let length = source.len();
        assert_eq!(
            document.range_at(0..length),
            Some(lsp_types::Range {
                start: Position {
                    line: 0,
                    character: 0
                },
                end: Position {
                    line: 0,
                    character: length as u32
                }
            })
        );

        // Out-of-bounds check
        assert_eq!(document.range_at(0..(length + 5)), None);
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
}
