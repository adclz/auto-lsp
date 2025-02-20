# Lexer

The tree-sitter lexer handles syntax analysis and reports:
- Syntax errors
- Missing nodes
- Invalid token sequences

`auto_lsp` requires a valid Concrete Syntax Tree (CST) from tree-sitter to generate an AST.

## Automatic errors

During AST construction, `auto_lsp` automatically detects and reports errors:

There are 2 types of errors:

- Missing Fields: Occurs when required fields in an AST node aren't matched by the query

- Query Mismatch: Happens when query captures don't align with the AST structure.
