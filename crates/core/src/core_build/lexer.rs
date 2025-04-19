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

use lsp_types::{Position, Range};
use salsa::Accumulator;
use tree_sitter::Node;

use crate::{
    errors::{LexerError, ParseErrorAccumulator},
    salsa::db::BaseDatabase,
};

/// Traverse a tree-sitter syntax tree to collect error nodes.
///
/// This function traverses the syntax tree in a depth-first manner to find error nodes:
/// - If a node `has_error()` but none of its children have errors, it is collected
/// - If a node `has_error()` and some children have errors, traverse those children
pub fn get_tree_sitter_errors(db: &dyn BaseDatabase, node: &Node, source_code: &[u8]) {
    let mut cursor = node.walk();

    if node.has_error() {
        if node.children(&mut cursor).any(|f| f.has_error()) {
            for child in node.children(&mut cursor) {
                get_tree_sitter_errors(db, &child, source_code);
            }
        } else {
            ParseErrorAccumulator::accumulate(format_error(node, source_code).into(), db);
        }
    }
}

fn format_error(node: &Node, source_code: &[u8]) -> LexerError {
    let start_position = node.start_position();
    let end_position = node.end_position();
    let range = Range {
        start: Position {
            line: start_position.row as u32,
            character: start_position.column as u32,
        },
        end: Position {
            line: end_position.row as u32,
            character: end_position.column as u32,
        },
    };

    if node.is_missing() {
        LexerError::Missing {
            range,
            error: format!("Syntax error: Missing {:?}", node.grammar_name()),
        }
    } else {
        let children_text: Vec<String> = (0..node.child_count())
            .map(|i| {
                node.child(i)
                    .unwrap()
                    .utf8_text(source_code)
                    .unwrap()
                    .to_string()
            })
            .collect();
        LexerError::Syntax {
            range,
            error: format!("Unexpected token(s): '{}'", children_text.join(" ")),
        }
    }
}
