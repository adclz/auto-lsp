use auto_lsp_core::errors::{LexerError, ParseErrorAccumulator};
use salsa::Accumulator;
use tree_sitter::Node;

use super::BaseDatabase;

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
    if node.is_missing() {
        LexerError::Missing {
            range: node.range(),
            error: format!("Syntax error: Missing '{}'", node.grammar_name()),
            grammar_name: node.grammar_name(),
        }
    } else {
        let children_text: Vec<String> = (0..node.child_count())
            .map(|i| {
                node.child(i as u32)
                    .unwrap()
                    .utf8_text(source_code)
                    .unwrap()
                    .to_string()
            })
            .collect();
        LexerError::Syntax {
            range: node.range(),
            error: format!("Unexpected token(s): '{}'", children_text.join(" ")),
            affected: children_text.join(" "),
        }
    }
}
