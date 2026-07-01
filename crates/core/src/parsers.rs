use crate::ast::AstNode;
use crate::document::Document;
use crate::errors::ParseError;
use parking_lot::RwLock;
use tree_sitter::Language;

pub struct Parser {
    /// The underlying parser, protected by [`RwLock`] for safe concurrent access.
    pub parser: RwLock<tree_sitter::Parser>,
    /// The language configuration for this parser.
    pub language: Language,
    /// Function to invoke the AST parser.
    pub ast_parser: InvokeParserFn,
}

impl std::fmt::Debug for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("parsers")
            .field("language", &self.language)
            .finish()
    }
}

pub type InvokeParserFn =
    fn(&dyn salsa::Database, &Document) -> Result<Vec<Box<dyn AstNode>>, ParseError>;
