# Auto LSP

A Rust crate for creating [Abstract Syntax Trees](https://en.wikipedia.org/wiki/Abstract_syntax_tree) (AST)
and [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) (LSP) servers.

[![CI Status](https://github.com/adclz/auto-lsp/actions/workflows/ast-gen-native.yml/badge.svg)](https://github.com/adclz/auto-lsp/actions/workflows/ast-gen-native.yml)
[![CI Status](https://github.com/adclz/auto-lsp/actions/workflows/lsp-server-native.yml/badge.svg)](https://github.com/adclz/auto-lsp/actions/workflows/lsp-server-native.yml)
[![Book](https://img.shields.io/badge/📚-book-blue)](https://adclz.github.io/auto-lsp/)
[![crates.io](https://img.shields.io/crates/v/auto-lsp)](https://crates.io/crates/auto-lsp)
![Rust Version](https://img.shields.io/badge/rustc-1.83.0%2B-orange)

`auto_lsp` is designed to be as language-agnostic as possible, allowing any Tree-sitter grammar to be used.

Defining a simple AST involves two steps: writing the queries and then defining the corresponding AST structures in Rust.

```sh
cargo add auto_lsp
```

## Quick example

Let's say you have a toy language with a root node named **document** containing a list of **function** nodes,
each containing a unique **name**.

A simple query file to capture the root document and function names:


```lisp
(document) @document
(function
    (name) @name) @function
```

The corresponding AST definition in Rust:

```rust, ignore
use auto_lsp::seq;

#[seq(query = "document")]
struct Document {
   functions: Vec<Function>
}

#[seq(query = "function")]
struct Function {
   name: Name
}

#[seq(query = "name")]
struct Name {}
```

Now that you have your AST defined, you can:
 - Implement the [AST traits](/auto-lsp/ast-and-queries/seq.html#seq-attributes) and create a LSP server (with the `lsp_server` feature).
 - Add your own logic for testing purposes, code generation, etc.

## Simplicity

`auto-lsp` only has 2 macros to define an AST:
 - [`#seq`](/auto-lsp/ast-and-queries/seq.html)
 - [`#choice`](/auto-lsp/ast-and-queries/choice.html)

All symbols are thread-safe and have their own parse function via blanket implementations. This means any symbol can be used as a root node, allowing you to:

 - Create a full AST from any Tree-sitter grammar.
 - Derive a subset of the grammar, depending on your needs.

However, this level of flexibility and permissiveness comes with some caveats.
It can be more prone to errors and requires careful attention when writing your queries.

To address this, `auto_lsp`  provides testing and logging utilities to help you ensure that the AST behaves as intended.

## Examples

 - [HTML Ast](https://github.com/adclz/auto-lsp/blob/main/src/tests/html_workspace/mod.rs)
 - [Python Ast](https://github.com/adclz/auto-lsp/blob/main/src/tests/python_workspace/ast.rs)
 - [Simple LSP Server](https://github.com/adclz/auto-lsp/tree/main/examples/native)
 - [Vscode extension with WASI](https://github.com/adclz/auto-lsp/tree/main/examples/vscode-wasi)

## Features


- `lsp_server`: Enable the LSP server (uses [`lsp_server`](https://crates.io/crates/lsp-server)).
- `wasm`: Enable wasm support (only compatible with `wasi-p1-threads`).
- `html`: Enable the html workspace mock for testing purposes.
- `python`: Enable the python workspace mock for testing purposes.

# Inspirations / Similar projects

- [Volar](https://volarjs.dev/)
- [Rust-sitter](https://github.com/hydro-project/rust-sitter)
- [StackGraphs](https://github.com/github/stack-graphs)
- [airblast-dev](https://github.com/airblast-dev)'s [texter](https://github.com/airblast-dev/texter), which saved hours of headache
