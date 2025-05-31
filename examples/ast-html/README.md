# HTML AST Example
This example demonstrates how to generate and use an HTML AST with [tree_sitter_html](https://github.com/tree-sitter/tree-sitter-html), using auto_lsp.

## Project Structure
- `src/db.rs`
Defines the Salsa database used to store parsed documents and ASTs.

- `src/generated.rs`
Contains the generated AST types and nodes from tree_sitter_html.

- `src/tests/corpus/*`
Snapshot tests using the HTML corpus from tree_sitter_html. These help ensure the generated AST remains stable and accurate.

- `src/tests/document_links.rs`
A simple example that demonstrates extracting document links using a combination of Tree-sitter queries and regular expressions.

## Notes

- `generated.rs` is only generated if **AST_GEN** env variable is set to a value higher than 0. 