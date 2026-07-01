use auto_lsp::default::db::BaseDatabase;
use auto_lsp::lsp_types::{SelectionRange, SelectionRangeParams};
use auto_lsp::{anyhow, tree_sitter};

/// Request for selection ranges
///
/// This is a port of [vscode anycode](https://github.com/microsoft/vscode-anycode/blob/main/anycode/server/src/common/features/selectionRanges.ts)
pub fn selection_ranges(
    db: &impl BaseDatabase,
    params: SelectionRangeParams,
) -> anyhow::Result<Option<Vec<SelectionRange>>> {
    let uri = &params.text_document.uri;

    let file = db
        .get_file(uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let document = file.document(db);
    let root_node = document.tree.root_node();

    let mut query_cursor = document.tree.walk();

    let mut results = vec![];

    for position in params.positions.iter() {
        let mut stack: Vec<tree_sitter::Node> = vec![];
        let position = document.normalize_position(position).unwrap();
        let point = tree_sitter::Point {
            row: position.line as usize,
            column: position.character as usize,
        };

        let mut node = root_node;
        loop {
            let child = node.named_children(&mut query_cursor).find(|candidate| {
                candidate.start_position() <= point && candidate.end_position() > point
            });

            if let Some(child) = child {
                stack.push(node);
                node = child;
                continue;
            }
            break;
        }

        let mut parent: Option<SelectionRange> = None;
        for node in stack {
            let Ok(range) = document.denormalize_range(&node.range()) else {
                continue;
            };
            let range = SelectionRange {
                range,
                parent: parent.map(Box::new),
            };
            parent = Some(range);
        }
        if let Some(parent) = parent {
            results.push(parent);
        }
    }

    Ok(Some(results))
}
