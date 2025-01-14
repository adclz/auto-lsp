use lsp_types::{FoldingRange, FoldingRangeKind, FoldingRangeParams};
use streaming_iterator::StreamingIterator;

use crate::session::{Session, WORKSPACES};

impl Session {
    pub fn get_folding_ranges(
        &mut self,
        params: FoldingRangeParams,
    ) -> anyhow::Result<Vec<FoldingRange>> {
        let uri = &params.text_document.uri;
        let workspace = WORKSPACES.lock();

        let workspace = workspace
            .get(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        let query = match workspace.parsers.cst_parser.queries.fold {
            Some(ref query) => query,
            None => return Ok(vec![]),
        };

        let root_node = workspace.document.cst.root_node();
        let source = workspace.document.document.text.as_str();

        let mut query_cursor = tree_sitter::QueryCursor::new();
        let mut captures = query_cursor.captures(query, root_node, source.as_bytes());

        let mut ranges = vec![];

        while let Some((m, capture_index)) = captures.next() {
            let capture = m.captures[*capture_index];
            let kind = match query.capture_names()[capture.index as usize] {
                "fold.comment" => FoldingRangeKind::Comment,
                _ => FoldingRangeKind::Region,
            };
            let range = capture.node.range();
            ranges.push(FoldingRange {
                start_line: range.start_point.row as u32,
                start_character: Some(range.start_point.column as u32),
                end_line: range.end_point.row as u32,
                end_character: Some(range.end_point.column as u32),
                kind: Some(kind),
                collapsed_text: None,
            });
        }

        Ok(ranges)
    }
}
