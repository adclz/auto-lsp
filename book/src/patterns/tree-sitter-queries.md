# Tree-sitter queries

`Document` gives you access to the `tree_sitter::Tree` via the `tree` field.

From there, you can run any query you want instead of using the AST.

## Example: Folding ranges in Python

```rust, ignore
// from: https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/python/folds.scm
static FOLD: &str = r#"
[
  (function_definition)
  (class_definition)
  (while_statement)
  (for_statement)
  (if_statement)
  (with_statement)
  (try_statement)
  (match_statement)
  (import_from_statement)
  (parameters)
  (argument_list)
  (parenthesized_expression)
  (generator_expression)
  (list_comprehension)
  (set_comprehension)
  (dictionary_comprehension)
  (tuple)
  (list)
  (set)
  (dictionary)
  (string)
] @fold

(comment) @fold.comment

[
  (import_statement)
  (import_from_statement)
]+ @fold"#;

// Precompile the query
pub static FOLD_QUERY: LazyLock<tree_sitter::Query> = LazyLock::new(|| {
    tree_sitter::Query::new(&tree_sitter_python::LANGUAGE.into(), FOLD)
        .expect("Failed to create fold query")
});

/// Request for folding ranges
pub fn folding_ranges(
    db: &impl BaseDatabase,
    params: FoldingRangeParams,
) -> anyhow::Result<Option<Vec<FoldingRange>>> {
    // Get the file in DB
    let uri = params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let document = file.document(db);

    let root_node = document.tree.root_node();
    let source = document.texter.text.as_str();

    // Creates a new query cursor
    let mut query_cursor = tree_sitter::QueryCursor::new();
    let mut captures = query_cursor.captures(&FOLD_QUERY, root_node, source.as_bytes());

    let mut ranges = vec![];

    // Iterate over the captures
    while let Some((m, capture_index)) = captures.next() {
        let capture = m.captures[*capture_index];
        let kind = match FOLD_QUERY.capture_names()[capture.index as usize] {
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

```