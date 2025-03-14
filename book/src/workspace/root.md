# Root

`Root` contains the following fields:

 - `url`: The url of the document associated with the workspace.
 - `parsers`: A static `Parsers` struct that contains all the necessary tools to generate an AST.
 - `tree_diagnostics`: A list of tree sitter parse errors
 - `ast_diagnostics`: A list of  diagnostics kept in sync with the AST.
 - `ast`: The AST (if any).
 - `unsolved_checks`: A list of symbols that still need to be resolved.
 - `unsolved_references`: A list of references that still need to be resolved.
 - `changes`: A list of last changes made to the AST.

Additionally, a `Root` has also a few useful methods:

- `parse`: Parse a given `Document` and generates the AST.
- `get_tree_sitter_errors`: Get all errors from the tree-sitter parser and converts them to Diagnostics (acts as a lexer).
- `set_comments`: Find all comments in the document and attempts to attach them to corresponding nodes.
- `find_all_with_regex`: Find all slices that match a given regex.

# Creating a Root

Use `from_utf8` method when you want to create a root from raw source code as a string.
If you have a Texter instance, use `from_texter` instead.

`Root` will create both the AST and virtual document, returned as a tuple of ([`Root`](https://docs.rs/auto-lsp/latest/auto_lsp/core/root/struct.Root.html), [`Document`](https://docs.rs/auto-lsp/latest/auto_lsp/core/document/struct.Document.html)).

```admonish
`Root` requires a properly configured AST parser. For details on setting up parsers, see, see [`#configuring-parsers`](auto_lsp/workspace/configuring-parsers.html)
```

### Example: Creating a Root from a String

```rust, ignore
use auto_lsp::core::root::Root;
use lsp_types::Url;

let source_code = r#"function foo() {}"#;

let (root, document) = Root::from_utf8(
    &PARSER_LIST.get("python").unwrap(),
    Url::parse("file://test").unwrap(),
    source_code.into(),
).unwrap();
```

### Example: Creating a Root from a Texter instance

```rust, ignore
use auto_lsp::core::root::Root;
use auto_lsp::texter::core::text::Texter;

let texter = Texter::new(source_code);

let (root, document) = Root::from_texter(
    &PARSER_LIST.get("python").unwrap(),
     Url::parse("file://test").unwrap(),
    texter,
).unwrap();
```

## Updating a Root

Once the document is updated, you can update the Root by re-parsing it:


```rust
root.parse(&document);
```