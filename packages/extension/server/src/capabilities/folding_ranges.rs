use lsp_types::{FoldingRange, FoldingRangeKind, FoldingRangeParams};
use streaming_iterator::StreamingIterator;

use crate::session::Session;

impl Session {
    pub fn get_folding_ranges(
        &mut self,
        params: FoldingRangeParams,
    ) -> anyhow::Result<Vec<FoldingRange>> {
        let uri = &params.text_document.uri;
        let workspace = self.workspaces.get(uri).unwrap();
        let root_node = workspace.cst.root_node();
        let source = workspace.document.get_content(None);
        let query = &workspace.cst_parser.queries.fold;

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
