use crate::globals::{Session, Workspace};
use lsp_server::{RequestId, Response};
use lsp_types::{request::SelectionRangeRequest, Range, SelectionRange, SelectionRangeParams};
use streaming_iterator::StreamingIterator;

pub fn get_selection_ranges(
    id: RequestId,
    params: &SelectionRangeParams,
    session: &Session,
) -> Response {
    let uri = &params.text_document.uri;
    let workspace = session.workspaces.get(uri).unwrap();
    let root_node = workspace.cst.root_node();
    let source = workspace.document.get_content(None);
    let query = &workspace.provider.queries.fold;

    let mut query_cursor = workspace.cst.walk();

    let mut results = vec![];

    for position in params.positions.iter() {
        let mut stack: Vec<tree_sitter::Node> = vec![];
        let mut offset = workspace.document.offset_at(*position) as usize;

        let mut node = root_node;
        loop {
            let child = node.named_children(&mut query_cursor).find(|candidate| {
                candidate.start_byte() <= offset && candidate.end_byte() > offset
            });

            match child {
                Some(child) => {
                    stack.push(node.clone());
                    node = child;
                    continue;
                }
                None => (),
            }
            break;
        }

        let mut parent: Option<SelectionRange> = None;
        for node in stack {
            let range = SelectionRange {
                range: Range {
                    start: workspace.document.position_at(node.start_byte() as u32),
                    end: workspace.document.position_at(node.end_byte() as u32),
                },
                parent: parent.map(|p| Box::new(p)),
            };
            parent = Some(range);
        }
        if let Some(parent) = parent {
            results.push(parent);
        }
    }

    let result = Some(results);
    let result = serde_json::to_value(&result).unwrap();
    Response {
        id,
        result: Some(result),
        error: None,
    }
}
