<div align="center" style="margin-bottom: 50px">
  <h1>Auto LSP</h1>
  <p>
    <strong>A Rust crate for creating <a href="https://en.wikipedia.org/wiki/Abstract_syntax_tree">Abstract Syntax Trees</a> (AST)
    and <a href="https://microsoft.github.io/language-server-protocol/">Language Server Protocol</a> (LSP) servers powered by <a href="https://tree-sitter.github.io/tree-sitter/">Tree-sitter</a></strong>
  </p>

  [![CI Status](https://github.com/adclz/auto-lsp/actions/workflows/ast-gen-native.yml/badge.svg)](https://github.com/adclz/auto-lsp/actions/workflows/ast-gen-native.yml)
  [![CI Status](https://github.com/adclz/auto-lsp/actions/workflows/lsp-server-native.yml/badge.svg)](https://github.com/adclz/auto-lsp/actions/workflows/lsp-server-native.yml)
  [![Book](https://img.shields.io/badge/📚-book-blue)](https://adclz.github.io/auto-lsp/)
  [![crates.io](https://img.shields.io/crates/v/auto-lsp)](https://crates.io/crates/auto-lsp)
  ![Rust Version](https://img.shields.io/badge/rustc-1.83.0%2B-orange)
</div>

`auto_lsp` is a generic library for creating Abstract Syntax Trees (AST) and Language Server Protocol (LSP) servers.

It leverages crates such as [lsp_types](https://docs.rs/lsp-types/0.97/lsp_types/), [lsp_server](https://docs.rs/lsp-server/latest/lsp_server/), [salsa](https://docs.rs/salsa/latest/salsa/), and [texter](https://docs.rs/texter/latest/texter/), and generates the AST of a Tree-sitter language to simplify building LSP servers.

`auto_lsp` is not a one-size-fits-all solution, but it provides useful abstractions while remaining flexible. You can override the default database as well as all LSP request and notification handlers.

It is designed to be as language-agnostic as possible, allowing any Tree-sitter grammar to be used.

See [ARCHITECTURE.md](ARCHITECTURE.md) for more information.

## ✨ Features

- Generates a thread-safe, immutable and iterable AST with parent-child relations from a Tree-sitter language.
- Supports downcasting of AST nodes to concrete types.
- Integrates with a Salsa database and parallelize LSP requests and notifications.

## 📚 Documentation

- [Book](https://adclz.github.io/auto-lsp/)
- [docs.rs](https://docs.rs/auto-lsp)

## Examples

- [HTML AST](https://github.com/adclz/auto-lsp/tree/main/examples/ast-html)
- [Python AST](https://github.com/adclz/auto-lsp/tree/main/examples/ast-python)
- [Simple LSP server](https://github.com/adclz/auto-lsp/tree/main/examples/native)
- [VSCode extension](https://github.com/adclz/auto-lsp/tree/main/examples/vscode-native)
- [VSCode extension with WASI](https://github.com/adclz/auto-lsp/tree/main/examples/vscode-wasi)

## Cargo Features

- `wasm`: Enables WASM support (compatible only with `wasi-p1-threads`).

## Inspirations / Similar Projects

- [Volar](https://volarjs.dev/)
- [Type-sitter](https://github.com/Jakobeha/type-sitter/)
- [Rust Analyzer](https://github.com/rust-lang/rust-analyzer)
- [Ruff](https://github.com/astral-sh/ruff)
- [texter](https://github.com/airblast-dev/texter) by [airblast-dev](https://github.com/airblast-dev), which saved hours of headaches.
