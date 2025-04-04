use auto_lsp_core::salsa::db::BaseDatabase;
use lsp_types::{FoldingRange, FoldingRangeKind, FoldingRangeParams};
use streaming_iterator::StreamingIterator;

/// Request for folding ranges
///
/// Uses the folding_range [`tree_sitter::Query`] if orovided in the initilization options.
pub fn get_folding_ranges<Db: BaseDatabase>(
    db: &Db,
    params: FoldingRangeParams,
) -> anyhow::Result<Option<Vec<FoldingRange>>> {
    let uri = params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let document = file.document(db).read();

    let query = match file.parsers(db).tree_sitter.queries.fold {
        Some(ref query) => query,
        None => return Ok(None),
    };

    let root_node = document.tree.root_node();
    let source = document.texter.text.as_str();

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

    Ok(Some(ranges))
}
