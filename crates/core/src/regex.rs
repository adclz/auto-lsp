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

use crate::document::Document;
use regex::{Match, Regex};
use streaming_iterator::StreamingIterator;

/// Find matches in the document with the provided regex
///
/// This function identifies comments in the [`tree_sitter::Tree`] of the [`Document`] and then
/// runs a regex search on the comment lines.
///
/// ### Returns
/// A vector of tuples containing the [`Match`] and the line number
pub fn find_all_with_regex<'a>(
    query: &tree_sitter::Query,
    document: &'a Document,
    regex: &Regex,
) -> Vec<(Match<'a>, usize)> {
    let root_node = document.tree.root_node();
    let source = document.texter.text.as_str();

    let mut query_cursor = tree_sitter::QueryCursor::new();
    let mut captures = query_cursor.captures(query, root_node, source.as_bytes());

    let mut results = vec![];

    while let Some((m, capture_index)) = captures.next() {
        let capture = m.captures[*capture_index];
        let range = capture.node.range();

        // Comment is maybe multiline
        let start_line = range.start_point.row;
        let end_line = range.end_point.row;
        let comment_lines = start_line..=end_line;

        for line in comment_lines {
            let text = document.texter.get_row(line).unwrap();
            for m in regex.find_iter(text) {
                results.push((m, line));
            }
        }
    }
    results
}
