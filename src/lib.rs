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
//!    <strong>A Rust crate for creating <a href="https://en.wikipedia.org/wiki/Abstract_syntax_tree">Abstract Syntax Trees</a> (AST)
//!    and <a href="https://microsoft.github.io/language-server-protocol/">Language Server Protocol</a> (LSP) servers powered by <a href="https://tree-sitter.github.io/tree-sitter/">Tree-sitter</a></strong>
//!  </p>
//!
//! [![CI Status](https://github.com/adclz/auto-lsp/actions/workflows/codegen.yml/badge.svg)](https://github.com/adclz/auto-lsp/actions/workflows/ast-gen-native.yml)
//! [![CI Status](https://github.com/adclz/auto-lsp/actions/workflows/test-ast-native.yml/badge.svg)](https://github.com/adclz/auto-lsp/actions/workflows/)
//! [![Book](https://img.shields.io/badge/📚-book-blue)](https://adclz.github.io/auto-lsp/)
//! [![crates.io](https://img.shields.io/crates/v/auto-lsp)](https://crates.io/crates/auto-lsp)
//! ![Rust Version](https://img.shields.io/badge/rustc-1.83.0%2B-orange)
//!
//!</div>
//!
//! `auto_lsp` is a generic library for creating Abstract Syntax Trees (AST) and Language Server Protocol (LSP) servers.//!
//!
//! It leverages crates such as [lsp_types](https://docs.rs/lsp-types/0.97/lsp_types/), [lsp_server](https://docs.rs/lsp-server/latest/lsp_server/), [salsa](https://docs.rs/salsa/latest/salsa/), and [texter](https://docs.rs/texter/latest/texter/), and generates the AST of a Tree-sitter language to simplify building LSP servers.
//!
//! `auto_lsp` provides useful abstractions while remaining flexible. You can override the default database as well as all LSP request and notification handlers.
//!
//! It is designed to be as language-agnostic as possible, allowing any Tree-sitter grammar to be used.
//!
//! See [ARCHITECTURE.md](https://github.com/adclz/auto-lsp/blob/main/ARCHITECTURE.md) for more information.
//!
//! ## ✨ Features
//!
//! - Generates a thread-safe, immutable and iterable AST with parent-child relations from a Tree-sitter language.
//! - Supports downcasting of AST nodes to concrete types.
//! - Integrates with a Salsa database and parallelize LSP requests and notifications.
//!
//! # 📚 Documentation
//!
//!  - [book](https://adclz.github.io/auto-lsp/)
//!  - [docs.rs](https://docs.rs/auto-lsp)
//!
//! ## Examples
//!
//! - [HTML AST](https://github.com/adclz/auto-lsp/tree/main/examples/ast-html)
//! - [Python AST](https://github.com/adclz/auto-lsp/tree/main/examples/ast-python)
//! - [Simple LSP Server](https://github.com/adclz/auto-lsp/tree/main/examples/native)
//! - [Vscode extension](https://github.com/adclz/auto-lsp/tree/main/examples/vscode-native)
//! - [Vscode extension with WASI](https://github.com/adclz/auto-lsp/tree/main/examples/vscode-wasi)
//!
//! # Cargo Features
//!
//! - `lsp_server`: Enables the LSP server (uses [`lsp_server`]).
//! - `wasm`: Enables wasm support (only compatible with `wasi-p1-threads`).
//!
//! # Inspirations / Similar projects
//!
//! - [Volar](https://volarjs.dev/)
//! - [Type-sitter](https://github.com/Jakobeha/type-sitter/)
//! - [Rust Analyzer](https://github.com/rust-lang/rust-analyzer)
//! - [Ruff](https://github.com/astral-sh/ruff)
//! - [airblast-dev](https://github.com/airblast-dev)'s [texter](https://github.com/airblast-dev/texter), which saved hours of headache

#[cfg(feature = "default")]
pub mod default {
    pub use auto_lsp_default::*;
}

// LSP server (enabled with the feature `lsp_server`)
#[cfg(feature = "lsp_server")]
pub mod server {
    pub use auto_lsp_server::*;
}

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
