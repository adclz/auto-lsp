use crate::core_build::parse::InvokeParserFn;
use parking_lot::RwLock;
use tree_sitter::{Language, Parser, Query};

/// List of parsing utilities.
///
/// Contains instances of both the [`tree_sitter`] parser and the AST parser.
pub struct Parsers {
    /// The underlying parser, protected by [`RwLock`] for safe concurrent access.
    pub parser: RwLock<Parser>,
    /// The language configuration for this parser.
    pub language: Language,
    /// The core query used to build the AST.
    pub core: Query,
    /// Function to invoke the AST parser.
    pub ast_parser: InvokeParserFn,
}

impl std::fmt::Debug for Parsers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("parsers")
            .field("language", &self.language)
            .field("core query", &self.core)
            .finish()
    }
}
