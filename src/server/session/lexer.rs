use lsp_types::{Diagnostic, Position, Range};
use tree_sitter::Node;

/// Traverse a tree-sitter syntax tree to collect error nodes.
///
/// This function traverses the syntax tree in a depth-first manner to find error nodes:
/// - If a node `has_error()` but none of its children have errors, it is collected
/// - If a node `has_error()` and some children have errors, traverse those children
pub(crate) fn get_tree_sitter_errors(node: &Node, utf8_str: &[u8], errors: &mut Vec<Diagnostic>) {
    let mut cursor = node.walk();

    if node.has_error() {
        if node.children(&mut cursor).any(|f| f.has_error()) {
            for child in node.children(&mut cursor) {
                get_tree_sitter_errors(&child, utf8_str, errors);
            }
        } else {
            errors.push(format_error(node, utf8_str));
        }
    }
}

fn format_error(node: &Node, utf8_str: &[u8]) -> Diagnostic {
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

    let message = if node.is_missing() {
        // TODO: Use node-types to find the missing node name ?
        format!("Syntax error: Missing")
    } else {
        let children_text: Vec<String> = (0..node.child_count())
            .map(|i| {
                node.child(i)
                    .unwrap()
                    .utf8_text(utf8_str)
                    .unwrap()
                    .to_string()
            })
            .collect();
        format!("Unexpected token(s): '{}'", children_text.join(" "))
    };

    Diagnostic {
        range,
        message,
        ..Default::default()
    }
}
