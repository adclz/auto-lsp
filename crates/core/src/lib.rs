//! # Auto LSP Core
//! Core crate for auto_lsp

pub mod ast;

/// Semantic tokens builder
pub mod semantic_tokens_builder;

/// Document symbols builder
pub mod document_symbols_builder;

/// Document handling
pub mod document;

pub mod errors;
pub mod parsers;
pub mod regex;
pub mod utils;
