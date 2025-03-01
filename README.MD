<div align="center" style="margin-bottom: 50px">
  <h1>Auto LSP</h1>
  <p>
    <strong>A Rust crate for creating <a href="https://en.wikipedia.org/wiki/Abstract_syntax_tree">Abstract Syntax Trees</a> (AST)
and <a href="https://microsoft.github.io/language-server-protocol/">Language Server Protocol</a> (LSP) servers powered by <a href="https://tree-sitter.github.io/tree-sitter/">Tree-sitter</a> queries</strong>
  </p>

[![CI Status](https://github.com/adclz/auto-lsp/actions/workflows/ci.yml/badge.svg)](https://github.com/adclz/auto-lsp/actions/workflows/ci.yml)
[![Book](https://img.shields.io/badge/📚-book-blue)](https://adclz.github.io/auto-lsp/)
[![crates.io](https://img.shields.io/crates/v/auto-lsp)](https://crates.io/crates/auto-lsp)
![Rust Version](https://img.shields.io/badge/rustc-1.83.0%2B-orange)


</div>

> `auto_lsp` is at an early stage, expect frequent breaking changes.

# Quick Example

`auto_lsp` is designed to be as language-agnostic as possible, allowing any Tree-sitter grammar to be used.

Defining a simple AST involves two steps: writing the queries and then defining the corresponding AST structures in Rust.

Let's say you have a toy language with a root node named **document** containing a list of **function** nodes, each containing a unique **name**.

A simple query file to capture the root document and function names:

```lisp
(document) @document
(function
    (name) @name) @function
```

The corresponding AST definition in Rust:

```rust
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
 - Implement the LSP traits and create a LSP server (with the `lsp_server` feature).
 - Add your own logic for testing purposes, code_generation, etc.

You can find more examples in the `src/tests` folder.

# Features

- `deadlock_detection`: Enable [`parking_lot`](https://crates.io/crates/parking_lot)'s deadlock detection (not compatible with `wasm`).
- `log`: Enable logging. (uses [`stderrlog`](https://crates.io/crates/stderrlog))
- `lsp_server`: Enable the LSP server (uses [`lsp_server`](https://crates.io/crates/lsp-server)).
- `rayon`: Enable [`rayon`](https://crates.io/crates/rayon) support (not compatible with `wasm`).
- `wasm`: Enable wasm support.
- `html`: Enable the html workspace mock for testing purposes.
- `python`: Enable the python workspace mock for testing purposes.

# Documentation

 - [book](https://adclz.github.io/auto-lsp/)
 - [docs.rs](https://docs.rs/auto-lsp)

# Inspirations / Similar projects

- [Volar](https://volarjs.dev/)
- [Rust-sitter](https://github.com/hydro-project/rust-sitter)
- [StackGraphs](https://github.com/github/stack-graphs)
- [airblast-dev](https://github.com/airblast-dev)'s [texter](https://github.com/airblast-dev/texter), which saved hours of headache
