# Dispatch

When working with the AST, you can either:

- Manually walk the tree through concrete node types.
- Iterate over node lists.

To make traversal easier, auto_lsp provides two macros: `dispatch_once!` and `dispatch!`, which call methods on nodes matching a given type.

## dispatch_once

Calls the method on the **first** node that matches one of the specified types and returns early.

```rust, ignore
use ast::generated::{FunctionDefinition, ClassDefinition};
use auto_lsp::dispatch_once;

dispatch_once!(node.lower(), [
    FunctionDefinition => return_something(db, param),
    ClassDefinition => return_something(db, param)
]);
Ok(None)
```

## dispatch

Calls the method on **all** matching node types.

```rust, ignore
use ast::generated::{FunctionDefinition, ClassDefinition};
use auto_lsp::dispatch;

dispatch!(node.lower(), [
    FunctionDefinition => build_something(db, param),
    ClassDefinition => build_something(db, param)
]);
Ok(())
```

## Lower Method

The `.lower()` method retrieves the lowest-level (most concrete) AST node for a given input.

This avoids matching on enum variants by directly returning the most specific node type.

It behaves similarly to `enum_dispatch`, but instead of returning a concrete type, it returns a `&dyn AstNode`.

```admonish
lower() always returns the most specific variant.
If an enum wraps another enum, lower() will recursively unwrap to reach the innermost node.
```

## Example: dispatch_once! in a Hover Request

```rust, ignore
// Request for hover
pub fn hover(db: &impl BaseDatabase, params: HoverParams) -> anyhow::Result<Option<Hover>> {
    // Get the file in DB
    let uri = &params.text_document_position_params.text_document.uri;

    let file = db
        .get_file(uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let document = file.document(db);

    // Find the node at the position using `offset_at` method
    // Note that we could also iterate over the AST to find the node
    let offset = document
        .offset_at(params.text_document_position_params.position)
        .ok_or_else(|| {
            anyhow::format_err!(
                "Invalid position, {:?}",
                params.text_document_position_params.position
            )
        })?;

    // Get the node at the given offset
    if let Some(node) = get_ast(db, file).descendant_at(offset) {
        // Call the `get_hover` method on the node if it matches the type.
        dispatch_once!(node.lower(), [
            PassStatement => get_hover(db, file),
            Identifier => get_hover(db, file)
        ]);
    }
    Ok(None)
}

// Implementation of the `get_hover` method for `PassStatement` and `Identifier`

impl PassStatement {
    fn get_hover(
        &self,
        _db: &impl BaseDatabase,
        _file: File,
    ) -> anyhow::Result<Option<lsp_types::Hover>> {
        Ok(Some(lsp_types::Hover {
            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::Markdown,
                value: r#"This is a pass statement

[See python doc](https://docs.python.org/3/reference/simple_stmts.html#the-pass-statement)"#
                    .into(),
            }),
            range: None,
        }))
    }
}

impl Identifier {
    fn get_hover(
        &self,
        db: &impl BaseDatabase,
        file: File,
    ) -> anyhow::Result<Option<lsp_types::Hover>> {
        let doc = file.document(db);
        Ok(Some(lsp_types::Hover {
            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::PlainText,
                value: format!("hover {}", self.get_text(doc.texter.text.as_bytes())?),
            }),
            range: None,
        }))
    }
}
```
