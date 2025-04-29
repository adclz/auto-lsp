# Configuring Parsers

To create a workspace, you must configure the parsers that will generate the AST.


To simplify parser configuration, you can use the [`configure_parsers!`](https://docs.rs/auto-lsp/latest/auto_lsp/macro.configure_parsers.html) macro to define a list of parsers.

`configure_parsers!` takes as first argument the name of the list, then each entry is a parser configuration.

A parser requires the following informations:
 - The tree-sitter language fn.
 - The AST root node (often Module, Document, SourceFile nodes ...).
 - The core query associated with the AST.

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

#[seq(query = "module")]
struct Module {}

configure_parsers!(
    MY_PARSER_LIST,
    "python" => {
        language: tree_sitter_python::LANGUAGE,
        ast_root: Module,
        core: CORE_QUERY,
    }
);

```
