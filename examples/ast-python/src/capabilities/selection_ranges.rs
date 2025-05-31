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
