use lsp_types::{Range, SelectionRange, SelectionRangeParams};

use crate::session::Session;

impl Session {
    pub fn get_selection_ranges(
        &mut self,
        params: SelectionRangeParams,
    ) -> anyhow::Result<Vec<SelectionRange>> {
        let uri = &params.text_document.uri;
        let workspace = self.workspaces.get(uri).unwrap();
        let root_node = workspace.document.cst.root_node();

        let mut query_cursor = workspace.document.cst.walk();

        let mut results = vec![];

        for position in params.positions.iter() {
            let mut stack: Vec<tree_sitter::Node> = vec![];
            let offset = workspace.document.offset_at(*position).unwrap();

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
            for _node in stack {
                let range = match workspace.document.range_at(offset) {
                    Some(range) => range,
                    None => continue,
                };
                let range = SelectionRange {
                    range,
                    parent: parent.map(|p| Box::new(p)),
                };
                parent = Some(range);
            }
            if let Some(parent) = parent {
                results.push(parent);
            }
        }

        Ok(results)
    }
}
