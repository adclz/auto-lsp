use std::ops::Deref;
use lsp_types::{SelectionRange, SelectionRangeParams};
use auto_lsp_core::salsa::db::WorkspaceDatabase;
use crate::server::session::{Session};

impl<Db: WorkspaceDatabase> Session<Db> {
    /// Request for selection ranges
    ///
    /// This is a port of [vscode anycode](https://github.com/microsoft/vscode-anycode/blob/main/anycode/server/src/common/features/selectionRanges.ts)
    pub fn get_selection_ranges(
        &mut self,
        params: SelectionRangeParams,
    ) -> anyhow::Result<Option<Vec<SelectionRange>>> {
        let uri = &params.text_document.uri;

        let db = &*self.db.lock();

        let file = db.get_file(&uri)
            .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

        let document = file.document(db.deref()).read();
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

                if let Some(child) = child {
                    stack.push(node);
                    node = child;
                    continue;
                }
                break;
            }

            let mut parent: Option<SelectionRange> = None;
            for _node in stack {
                let range = match document.node_range_at(offset) {
                    Some(range) => range,
                    None => continue,
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
}
