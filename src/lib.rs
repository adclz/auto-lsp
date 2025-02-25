#![allow(rustdoc::private_intra_doc_links)]
//!<div align="center" style="margin-bottom: 50px">
//!  <h1>Auto LSP</h1>
//!  <p>
//!    A Rust crate for creating <a href="https://en.wikipedia.org/wiki/Abstract_syntax_tree">Abstract Syntax Trees</a> (AST)
//! and <a href="https://microsoft.github.io/language-server-protocol/">Language Server Protocol</a> (LSP) servers powered by <a href="https://tree-sitter.github.io/tree-sitter/">Tree-sitter</a> queries
//!  </p>
//!
//! [![CI Status](https://github.com/adclz/auto-lsp/actions/workflows/ci.yml/badge.svg)](https://github.com/adclz/auto-lsp/actions/workflows/ci.yml)
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
//! Let's say you have a toy language with a root node named **document** containing a list of **function** nodes,
//! each containing a unique **name**.
//!
//! A simple query file to capture the root document and function names:
//!
//! ```lisp
//! (document) @document
//! (function
//!     (name) @name) @function
//! ```
//!
//! The corresponding AST definition in Rust:
//!
//! ```rust
//! # use auto_lsp::core::ast::*;
//! # use auto_lsp::seq;
//! #[seq(query = "document")]
//! struct Document {
//!    functions: Vec<Function>
//! }
//!
//! #[seq(query = "function")]
//! struct Function {
//!    name: Name
//! }
//!
//! #[seq(query = "name")]
//! struct Name {}
//! ```
//!
//! Now that you have your AST defined, you can:
//!  - Implement [traits](core::ast) and create a LSP server (with the `lsp_server` feature).
//!  - Add your own logic for testing purposes, code_generation, etc.
//!
//! You can find more examples in the `tests` folder.
//! # Features
//! - `deadlock_detection`: Enable [`parking_lot`]'s deadlock detection (not compatible with `wasm`).
//! - `log`: Enable logging. (uses [`stderrlog`])
//! - `lsp_server`: Enable the LSP server (uses [`lsp_server`]).
//! - `rayon`: Enable [`rayon`] support (not compatible with `wasm`).
//! - `wasm`: Enable wasm support.
//! - `html`: Enable the html workspace mock for testing purposes.
//! - `python`: Enable the python workspace mock for testing purposes.
//! - `incremental`: Enable incremental parsing.

/// LSP server (enabled with feature `lsp_server`)
#[cfg(any(feature = "lsp_server", test))]
pub mod server;

mod tests;

/// A mock implementation of a python AST
#[cfg(any(feature = "python", test))]
pub mod python {
    pub use crate::tests::python_workspace::*;
}

/// A mock implementation of a html AST
#[cfg(any(feature = "html", test))]
pub mod html {
    pub use crate::tests::html_workspace::*;
}

/// Re-export of the [`auto_lsp_core`] crate
pub mod core {
    // Not public API. Referenced by macro-generated code.
    #[doc(hidden)]
    pub mod build {
        pub use auto_lsp_core::build::*;
    }

    pub use auto_lsp_core::ast;
    pub use auto_lsp_core::document;
    pub use auto_lsp_core::document_symbols_builder;
    pub use auto_lsp_core::semantic_tokens_builder;
    pub use auto_lsp_core::workspace;
    #[doc(hidden)]
    pub use auto_lsp_core::{builder_error, builder_warning};
}

/// Configuration utilities
#[doc(hidden)]
pub mod configure;

// Re-export of [`seq`] and [`choice`] macros
pub use auto_lsp_macros::*;

pub use ariadne;
#[doc(hidden)]
pub use constcat;
pub use lsp_types;
pub use parking_lot;
#[cfg(feature = "rayon")]
pub use rayon;
#[cfg(any(feature = "lsp_server", test))]
pub use texter;
pub use tree_sitter;
