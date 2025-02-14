# Workspace and Document

To test or run your queries and your AST, you need to create a [`Workspace`](https://docs.rs/auto-lsp/latest/auto_lsp/core/workspace/struct.Workspace.html) .

The `Workspace` in `auto-lsp` is not related to a Visual Studio Code workspace.
Instead, it refers to an internal structure that contains the AST and related information, such as diagnostics and parsers."

The document and tree-sitter parts are stored in the [`Document`](https://docs.rs/auto-lsp/latest/auto_lsp/core/document/struct.Document.html) struct.

## Workspace struct

`Workspace` contains the following fields:

 - `url`: The url of the document associated with the workspace.
 - `parsers`: A static `Parsers` struct that contains all the necessary tools to generate an AST.
 - `diagnostics`: A list of diagnostics kept in sync with the AST.
 - `ast`: The AST (if any).
 - `unsolved_checks`: A list of symbols that still need to be resolved.
 - `unsolved_references`: A list of references that still need to be resolved.
 - `changes`: A list of last changes made to the AST.

Additionally, a `Workspace` has also a few useful methods:

- `parse`: Parse a given `Document` and generate the AST.

```admonish
It's preferable to use `from_utf8` or `from_texter` to create a workspace, see [#creating-a-workspace](/workspace-and-document/creating-a-workspace.html).
```

- `get_tree_sitter_errors`: Get all errors from the tree-sitter parser and converts them to Diagnostics (acts as a lexer).
- `set_comments`: Find all comments in the document and attempts to attach them to corresponding nodes.
- `find_all_with_regex`: Find all slices that match a given regex.

## Document struct

Struct has the following fields:

 - `texter`: a texter struct that stores the document.
 - `tree`: The tree-sitter tree.
