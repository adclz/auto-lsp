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
//! # use auto_lsp::macros::seq;
//!
//! #[seq(query_name = "document", kind(symbol()))]
//! struct Document {
//!    functions: Vec<Function>
//! }
//!
//! #[seq(query_name = "function", kind(symbol()))]
//! struct Function {
//!    name: Name
//! }
//!
//! #[seq(query_name = "name", kind(symbol()))]
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
//! - `assertions`: Enable compile-time checks for conflicting queries.
//! - `deadlock_detection`: Enable [`parking_lot`]'s deadlock detection (not compatible with `wasm`).
//! - `log`: Enable logging. (uses [`stderrlog`])
//! - `lsp_server`: Enable the LSP server (uses [`lsp_server`]).
//! - `python_test`: Enable the python workspace mock for testing purposes.
//! - `rayon`: Enable [`rayon`] support (not compatible with `wasm`).
//! - `wasm`: Enable wasm support.

#[cfg(doc)]
use lsp_server;
/// A mock python workspace used for testing purposes.
/// This module is only available with the `python_test` feature enabled or during tests.
#[cfg(any(feature = "python_test", test))]
pub mod python_workspace;
/// LSP server (enabled with feature `lsp_server`)
#[cfg(any(feature = "lsp_server", test))]
pub mod server;
#[cfg(test)]
pub mod tests;

/// Re-export of the [`auto_lsp_core`] crate
pub mod core {
    // Not public API. Referenced by macro-generated code.
    #[doc(hidden)]
    pub mod build {
        pub use auto_lsp_core::build::*;
    }

    pub use auto_lsp_core::ast;
    pub use auto_lsp_core::semantic_tokens;
    pub use auto_lsp_core::workspace;
    #[doc(hidden)]
    pub use auto_lsp_core::{builder_error, builder_warning};
}
/// [`macros::seq`] and [`macros::choice`] macros
pub use auto_lsp_macros as macros;

#[doc(hidden)]
pub use constcat;
pub use lsp_types;
pub use parking_lot;
#[cfg(feature = "rayon")]
pub use rayon;
#[cfg(any(feature = "lsp_server", test))]
pub use texter;
pub use tree_sitter;

#[macro_export]
macro_rules! nested_struct {
    // [MAIN] Primary rule to generate the struct
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $(@nested(#[$field_nested_meta:meta]))*
                $field_vis:vis $field_name:ident : $field_ty:ident $({
                    $($field_ty_inner:tt)*
                })?
            ),*
        $(,)? }
    ) => {
        // Generate our primary struct
        $(#[$meta])* $vis struct $name {
            $(
                $(#[$field_meta])*
                $field_vis $field_name : $field_ty
            ),*
        }

        // Generate our inner structs for fields
        $(nested_struct! {
            @nested
            $(#[$field_nested_meta])*
            $field_vis $field_ty $({
                $($field_ty_inner)*
            })?
        })*
    };

    // [INCLUDE] Used to filter out struct generation to only nested types
    (@nested $(#[$meta:meta])* $vis:vis $name:ident {$($fields:tt)*}) => {
        nested_struct! {
            $(#[$meta])*
            $vis struct $name {
                $($fields)*
            }
        }
    };

    // [EXCLUDE] Used to filter out struct generation to only nested types
    (@nested $(#[$meta:meta])* $vis:vis $name:ident) => {};

    // Any garbage we will ignore, including generating an invalid struct
    /* ($($other:tt)*) => {
        compile_error!(stringify!($($other)*));
    }; */
}

nested_struct! {
    pub struct MyStruct {
        pub data: MyStructData {
            pub data: u32
        }
    }
}
