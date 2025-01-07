use lsp_types::{Diagnostic, Position, Range};
use tree_sitter::Node;

pub fn get_tree_sitter_errors(node: &Node, utf8_str: &[u8]) -> Vec<Diagnostic> {
    let mut errors = Vec::new();
    let mut cursor = node.walk();

    if node.has_error() {
        for child in node.children(&mut cursor) {
            if child.is_error() || child.is_missing() {
                let formatted_error = format_error(child, utf8_str);
                errors.push(formatted_error);
            }
            errors.extend(get_tree_sitter_errors(&child, utf8_str));
        }
    }

    errors
}

fn format_error(node: Node, utf8_str: &[u8]) -> Diagnostic {
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

    let message = if node.has_error() && node.is_missing() {
        format!("Syntax error: {:?}", node)
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
