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

## Updating a workspace

If you want to update a workspace, you can use the `update` method.

`update` takes a list of `edits` made to the document and a reference to the document itself.

Make sure the document has been updated before calling `update`.
