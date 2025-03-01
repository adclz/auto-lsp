//! # Auto LSP Core
//! Core crate for auto_lsp

mod core_ast;
mod core_build;

/// This module contains everything related to ast symbols already created
pub mod ast {
    pub use crate::core_ast::capabilities::*;
    pub use crate::core_ast::core::*;
    pub use crate::core_ast::data::*;
    pub use crate::core_ast::display::*;
    pub use crate::core_ast::symbol::*;
}

/// This module contains everything related to building ast symbols
pub mod build {
    pub use crate::core_build::buildable::*;
    pub use crate::core_build::downcast::*;
    pub use crate::core_build::parse::*;
    pub use crate::core_build::symbol::*;
}

/// Semantic tokens builder
pub mod semantic_tokens_builder;

// Document symbols builder
pub mod document_symbols_builder;

/// Document handling
pub mod document;
/// Root
pub mod root;
pub mod workspace;
