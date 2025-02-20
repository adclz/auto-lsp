# Creating a workspace

Once you are done configuring the parsers, you can create a workspace using the [`Workspace`](https://docs.rs/auto-lsp/latest/auto_lsp/core/workspace/struct.Workspace.html) struct.

`Workspace` will create both the AST and virtual document, returned as a tuple of ([`Workspace`](https://docs.rs/auto-lsp/latest/auto_lsp/core/workspace/struct.Workspace.html), [`Document`](https://docs.rs/auto-lsp/latest/auto_lsp/core/document/struct.Document.html)).

Use `from_utf8` method when you want to create a workspace from raw source code as a string.
If you have a Texter instance, use `from_texter` instead.

```rust, ignore
use auto_lsp::core::ast::*;
use auto_lsp::core::workspace::Workspace;
use lsp_types::Url;

let source_code = r#"function foo() {}"#;

// From a string
let (workspace, document) = Workspace::from_utf8(
    &PARSER_LIST.get("python").unwrap(),
    Url::parse("file://test").unwrap(),
    source_code.into(),
).unwrap();

// From Texter
use auto_lsp::texter::core::text::Texter;

let texter = Texter::new(source_code);

let (workspace, document) = Workspace::from_texter(
    &PARSER_LIST.get("python").unwrap(),
     Url::parse("file://test").unwrap(),
    texter,
).unwrap();
```

## Updating a document

Use `document.update()` to process document changes:

`update` takes two parameters:
 - The tree-sitter parser instance.
 - A list of `lsp_types::TextDocumentChangeEvent` changes.

```rust
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

// Apply changes and get edits
// this list can then be passed to a Workspace
let edits = document
    .update(
        &mut workspace.parsers.tree_sitter.parser.write(),
        &vec![change],
    )
    .unwrap();

```

## Updating a workspace

After document changes, update the workspace using the `parse` method.

```admonish
If you have not enabled the `incremental` feature, you can pass `None`.
```

```rust
workspace.parse(Some(&edits), &document);
```