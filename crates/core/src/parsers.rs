use crate::core_build::parse::InvokeParserFn;
use parking_lot::RwLock;
use tree_sitter::{Language, Parser, Query};

/// A list of Parsers.
///
/// Contains instances of both the [`tree_sitter`] parser and the AST parser.
#[derive(Debug)]
pub struct Parsers {
    /// The [`TreeSitter`] parser configuration and queries.
    pub tree_sitter: TreeSitter,
    /// Function to invoke the AST parser.
    pub ast_parser: InvokeParserFn,
}

/// Tree-sitter configuration for a [`Root`].
///
/// Manages the parser, language, and associated queries.
pub struct TreeSitter {
    /// The underlying parser, protected by [`RwLock`] for safe concurrent access.
    pub parser: RwLock<Parser>,
    /// Node types in the language, represented as a JSON string.
    pub node_types: &'static str,
    /// The language configuration for this parser.
    pub language: Language,
    /// A collection of queries used for different features.
    pub queries: Queries,
}

impl std::fmt::Debug for TreeSitter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreeSitter")
            .field("node_types", &self.node_types)
            .field("language", &self.language)
            .field("queries", &self.queries)
            .finish()
    }
}

/// A collection of queries used within [`TreeSitter`].
#[derive(Debug)]
pub struct Queries {
    /// The core query used to build the AST.
    pub core: Query,
    /// Query to identify comments and document links (optional).
    pub comments: Option<Query>,
    /// Query for generating folding ranges (optional).
    pub fold: Option<Query>,
    /// Query for syntax highlighting (optional).
    pub highlights: Option<Query>,
}
