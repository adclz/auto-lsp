# Configuring Parsers

To create a workspace, you must configure the parsers that will generate the AST.


To simplify parser configuration, you can use the [`configure_parsers!`](https://docs.rs/auto-lsp/latest/auto_lsp/macro.configure_parsers.html) macro to define a list of parsers.

`configure_parsers!` takes as first argument the name of the list, then each entry is a parser configuration.

A parser requires the following informations:
 - The tree-sitter language fn.
 - The node types.
 - The AST root node (often Module, Document, SourceFile nodes ...).
 - The core query associated with the AST.

Optional fields include:
 - The comment query.
 - The fold query.
 - The highlights query.

The `fold` query is used to define code folding regions, while the `highlights` query can be used to specify syntax highlighting rules.

## Example

The following example demonstrates how to configure a parser for the Python language:

```rust, ignore
use auto_lsp::seq;
use auto_lsp::configure_parsers;
use auto_lsp::core::ast::*;

static CORE_QUERY: &'static str = "
(module) @module
(function_definition
   name: (identifier) @function.name) @function
";

static COMMENT_QUERY: &'static str = "
(comment) @comment
";

#[seq(query = "module")]
struct Module {}

configure_parsers!(
    MY_PARSER_LIST,
    "python" => {
        language: tree_sitter_python::LANGUAGE,
        node_types: tree_sitter_python::NODE_TYPES,
        ast_root: Module,
        core: CORE_QUERY,
        comment: Some(COMMENT_QUERY),
        fold: None,
        highlights: None
    }
);

```
