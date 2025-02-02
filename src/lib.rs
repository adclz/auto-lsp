#![allow(rustdoc::private_intra_doc_links)]
//! # Auto LSP
//!
//! A Rust crate for creating [Abstract Syntax Trees](https://en.wikipedia.org/wiki/Abstract_syntax_tree) (AST)
//! and [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) (LSP) servers.
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
//! ```
//! # use auto_lsp::core::ast::*;
//! # use auto_lsp::seq;
//!
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
//!  - Implement the [LSP traits](core::ast) and create a LSP server (with the `lsp_server` feature).
//!  - Add your own logic for testing purposes, code_generation, etc.
//!
//! You can find more examples in the `tests` folder.
//!
//! ## Features
//! - `deadlock_detection`: Enable [`parking_lot`]'s deadlock detection (not compatible with `wasm`).
//! - `log`: Enable logging. (uses [`stderrlog`])
//! - `lsp_server`: Enable the LSP server (uses [`lsp_server`]).
//! - `rayon`: Enable [`rayon`] support (not compatible with `wasm`).
//! - `wasm`: Enable wasm support.
//! - `html`: Enable the html workspace mock for testing purposes.
//! - `python`: Enable the python workspace mock for testing purposes.

#[cfg(doc)]
use lsp_server;
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

#[doc(hidden)]
pub use constcat;
pub use lsp_types;
pub use parking_lot;
#[cfg(feature = "rayon")]
pub use rayon;
#[cfg(any(feature = "lsp_server", test))]
pub use texter;
pub use tree_sitter;
