use lsp_types::{SelectionRange, SelectionRangeParams};

use crate::server::session::{Session, WORKSPACES};

impl Session {
    /// Request for selection ranges
    ///
    /// This is a port of [vscode anycode](https://github.com/microsoft/vscode-anycode/blob/main/anycode/server/src/common/features/selectionRanges.ts)
    pub fn get_selection_ranges(
        &mut self,
        params: SelectionRangeParams,
    ) -> anyhow::Result<Vec<SelectionRange>> {
        let uri = &params.text_document.uri;
        let workspace = WORKSPACES.lock();

        let (_, document) = workspace
            .get(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;
        let root_node = document.tree.root_node();

        let mut query_cursor = document.tree.walk();

        let mut results = vec![];

        for position in params.positions.iter() {
            let mut stack: Vec<tree_sitter::Node> = vec![];
            let offset = document.offset_at(*position).unwrap();

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
                let range = match document.range_at(offset) {
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
