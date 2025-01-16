pub(crate) fn intersecting_ranges(
    range1: &std::ops::Range<usize>,
    range2: &tree_sitter::Range,
) -> bool {
    range1.start <= range2.start_byte && range1.end >= range2.end_byte
}

pub(crate) fn tree_sitter_range_to_lsp_range(range: &tree_sitter::Range) -> lsp_types::Range {
    let start = range.start_point;
    let end = range.end_point;
    lsp_types::Range {
        start: lsp_types::Position {
            line: start.row as u32,
            character: start.column as u32,
        },
        end: lsp_types::Position {
            line: end.row as u32,
            character: end.column as u32,
        },
    }
}
