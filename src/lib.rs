/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/
#![allow(rustdoc::private_intra_doc_links)]
//!<div align="center" style="margin-bottom: 50px">
//!  <h1>Auto LSP</h1>
//!  <p>
//!    A Rust crate for creating <a href="https://en.wikipedia.org/wiki/Abstract_syntax_tree">Abstract Syntax Trees</a> (AST)
//! and <a href="https://microsoft.github.io/language-server-protocol/">Language Server Protocol</a> (LSP) servers powered by <a href="https://tree-sitter.github.io/tree-sitter/">Tree-sitter</a> queries
//!  </p>
//!
//! [![CI Status](https://github.com/adclz/auto-lsp/actions/workflows/ast-gen-native.yml/badge.svg)](https://github.com/adclz/auto-lsp/actions/workflows/ast-gen-native.yml)
//! [![CI Status](https://github.com/adclz/auto-lsp/actions/workflows/lsp-server-native.yml/badge.svg)](https://github.com/adclz/auto-lsp/actions/workflows/)
//! [![Book](https://img.shields.io/badge/📚-book-blue)](https://adclz.github.io/auto-lsp/)
//! [![crates.io](https://img.shields.io/crates/v/auto-lsp)](https://crates.io/crates/auto-lsp)
//! ![Rust Version](https://img.shields.io/badge/rustc-1.83.0%2B-orange)
//!
//!</div>
//!
//! `auto_lsp` is designed to be as language-agnostic as possible, allowing any Tree-sitter grammar to be used.
//!
//! Defining a simple AST involves two steps: writing the queries and then defining the corresponding AST structures in Rust.
//!
//! ## Quick example
//!
//! Now that you have your AST defined, you can:
//!  - Implement the [AST traits](https://adclz.github.io/auto-lsp/ast-and-queries/seq.html#seq-attributes) and create a LSP server (with the `lsp_server` feature).
//!  - Add your own logic for testing purposes, code generation, etc.
//!
//! # Documentation
//!
//!  - [book](https://adclz.github.io/auto-lsp/)
//!  - [docs.rs](https://docs.rs/auto-lsp)
//!
//! ## Examples
//!
//! - [HTML Ast](https://github.com/adclz/auto-lsp/blob/main/src/tests/html_workspace/mod.rs)
//! - [Python Ast](https://github.com/adclz/auto-lsp/blob/main/src/tests/python_workspace/ast.rs)
//! - [Simple LSP Server](https://github.com/adclz/auto-lsp/tree/main/examples/native)
//! - [Vscode extension with WASI](https://github.com/adclz/auto-lsp/tree/main/examples/vscode-wasi)
//!
//! # Features
//!
//! - `lsp_server`: Enable the LSP server (uses [`lsp_server`]).
//! - `wasm`: Enable wasm support (only compatible with `wasi-p1-threads`).
//! - `html`: Enable the html root mock for testing purposes.
//! - `python`: Enable the python root mock for testing purposes.
//!
//! # Inspirations / Similar projects
//!
//! - [Volar](https://volarjs.dev/)
//! - [Rust-sitter](https://github.com/hydro-project/rust-sitter)
//! - [StackGraphs](https://github.com/github/stack-graphs)
//! - [airblast-dev](https://github.com/airblast-dev)'s [texter](https://github.com/airblast-dev/texter), which saved hours of headache

// LSP server (enabled with the feature `lsp_server`)
#[cfg(feature = "lsp_server")]
pub mod server;

/// Re-export of the [`auto_lsp_core`] crate
pub mod core {
    pub use auto_lsp_core::*;
}

/// Configuration utilities
#[doc(hidden)]
pub mod configure;

pub use anyhow;
#[cfg(feature = "lsp_server")]
pub use lsp_server;
pub use lsp_types;
pub use salsa;
pub use texter;
pub use tree_sitter;
