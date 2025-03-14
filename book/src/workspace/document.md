# Document

## Acknowledgement

Thanks to [`texter`](https://github.com/airblast-dev/texter) crate, text in any encoding is supported.

`texter` also provides an efficient way to update documents incrementally.

the Document struct has the following fields:

 - `texter`: a texter struct that stores the document.
 - `tree`: The tree-sitter syntax tree.

## Creating a document

Document can be created using either the  `from_utf8` or `from_texter` methods of `Root`.

## Updating a document

Use `document.update()` to process document changes:

`update` takes two parameters:
 - The tree-sitter parser instance.
 - A list of `lsp_types::TextDocumentChangeEvent` changes.

```rust, ignore
let change = lsp_types::TextDocumentContentChangeEvent {
    range: Some(lsp_types::Range {
        start: lsp_types::Position {
            line: 0,
            character: 0,
        },
        end: lsp_types::Position {
            line: 0,
            character: 0,
        },
    }),
    range_length: Some(26),
    text: "<div></div>".into(),
};

let edits = document
    .update(
        &mut root.parsers.tree_sitter.parser.write(),
        &vec![change],
    )
    .unwrap();

```
