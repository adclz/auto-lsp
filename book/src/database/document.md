# Document

## Acknowledgement

Thanks to [`texter`](https://github.com/airblast-dev/texter) crate, text in any encoding is supported.

`texter` also provides an efficient way to update documents incrementally.

The Document struct has the following fields:

 - `texter`: a texter struct that stores the document.
 - `tree`: The tree-sitter syntax tree.

## Creating a document

Document can be created using either the  `from_utf8` or `from_texter` methods of `FileManager`.

## Updating a document

The database support updating a document using the `update` method of `FileManager`.

`update` takes 2 parameters:
 - The `Url` of the document to update.
 - A list of [`lsp_types::TextDocumentChangeEvent`](https://docs.rs/lsp-types/latest/lsp_types/struct.TextDocumentContentChangeEvent.html) changes.

These changes are sent by the client when the document is modified.

```rust, ignore
registry.on_mut::<DidChangeTextDocument, _>(|session, params| {
    Ok(session.db.update(&params.text_document.uri, &params.content_changes)?)
})
```

`update` may return a `DataBaseError` if the update fails.