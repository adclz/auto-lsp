# Range Requests

Some LSP requests, such as semantic tokens, support ranges, meaning you should request information for a specific range of the document instead of the whole document.

To support this, you can use the `get_ast` method from the `default` crate to get the AST of a file.

Since the nodes are sorted by position, it is possible to iterate over the AST and perform operations only on a portion of the AST that contains the range.
 
## Example: Semantic tokens for a range

```rust, ignore
pub fn semantic_tokens_range(
    db: &impl BaseDatabase,
    params: SemanticTokensRangeParams,
) -> anyhow::Result<Option<SemanticTokensResult>> {
    // Get the file in DB
    let uri = params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let mut builder = SemanticTokensBuilder::new("".into());

    // Iterate over the AST
    for node in get_ast(db, file).iter() {
        // Skip nodes that are before the range
        if node.get_lsp_range().end <= params.range.start {
            continue;
        }
        // Stop at nodes that are after the range
        if node.get_lsp_range().start >= params.range.end {
            break;
        }
        // Dispatch on the node
        dispatch!(node.lower(),
            [
                FunctionDefinition => build_semantic_tokens(db, file, &mut builder)
            ]
        );
    }

    Ok(Some(SemanticTokensResult::Tokens(builder.build())))
}
```
 