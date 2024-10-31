use lsp_types::{Range, SelectionRange, SelectionRangeParams};

use crate::session::Session;

impl<'a> Session<'a> {
    pub fn get_selection_ranges(
        &mut self,
        params: SelectionRangeParams,
    ) -> anyhow::Result<Vec<SelectionRange>> {
        let uri = &params.text_document.uri;
        let workspace = self.workspaces.get(uri).unwrap();
        let root_node = workspace.cst.root_node();

        let mut query_cursor = workspace.cst.walk();

        let mut results = vec![];

        for position in params.positions.iter() {
            let mut stack: Vec<tree_sitter::Node> = vec![];
            let offset = workspace.document.offset_at(*position) as usize;

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

        Ok(results)
    }
}
