use crate::{
    core_ast::symbol::{DynSymbol, WeakSymbol},
    core_build::parse::InvokeParserFn,
    document::Document,
};
use lsp_types::{Diagnostic, Url};
use parking_lot::RwLock;
use std::sync::Arc;
use texter::core::text::Text;
use tree_sitter::{Language, Parser, Query};

pub mod comments;
pub mod lexer;
pub mod parse;
pub mod regex;

/// Parsers available in a [`Root`].
///
/// Contains instances of both the [`tree_sitter`] parser and the AST parser.
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

/// A collection of queries used within [`TreeSitter`].
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

/// Contains diagnostics, parser lists, URL, and AST for a document.
/// Note: The document text and the [`tree_sitter::Tree`] are not stored in this struct.
pub struct Root {
    /// The URI of the document associated with this Root.
    pub url: Arc<Url>,
    /// Parsers used for processing the document.
    pub parsers: &'static Parsers,
    /// Diagnostics collected during parsing by tree-sitter.
    pub lexer_diagnostics: Vec<Diagnostic>,
    /// Diagnostics collected during AST parsing and/or running checks.
    pub ast_diagnostics: Vec<Diagnostic>,
    /// The AST for the document, if available.
    pub ast: Option<DynSymbol>,
    /// Nodes flagged as unresolved during checks.
    pub unsolved_checks: Vec<WeakSymbol>,
    /// References flagged as unresolved during analysis.
    pub unsolved_references: Vec<WeakSymbol>,
}

impl Root {
    /// Creates a new [`Root`] and [`Document`].
    ///
    /// This function:
    /// 1. Parses the source code using the [`TreeSitter`] parser.
    /// 2. Builds the AST using the core query and AST parser.
    /// 3. Resolves checks, references, and comments in the document.
    ///
    /// # Arguments
    /// * `parsers` - Language parsers (created using the `configure_parsers!` macro).
    /// * `uri` - The URI of the document to process.
    /// * `source_code` - The source code of the document as a UTF-8 string.
    ///
    /// # Returns
    /// A tuple containing the newly created [`Root`] and [`Document`].
    pub fn from_utf8(
        parsers: &'static Parsers,
        uri: Url,
        source_code: String,
    ) -> anyhow::Result<(Self, Document)> {
        // Parse the source code into a syntax tree.
        let tree = parsers
            .tree_sitter
            .parser
            .write()
            .parse(source_code.as_bytes(), None)
            .ok_or_else(|| anyhow::format_err!("Tree-sitter failed to parse source code"))?;

        // Initialize the document with the source code and syntax tree.
        let document = Document {
            texter: Text::new(source_code),
            tree,
        };

        // Set up the Root with basic properties.
        let mut root = Root {
            url: Arc::new(uri.clone()),
            parsers,
            ast_diagnostics: vec![],
            lexer_diagnostics: vec![],
            ast: None,
            unsolved_checks: vec![],
            unsolved_references: vec![],
        };

        // Build the AST using the core query and AST parser function.
        root.parse(&document);

        Ok((root, document))
    }

    /// Creates a new [`Root`] and [`Document`].
    ///
    /// This function:
    /// 1. Parses the source code using the [`TreeSitter`] parser.
    /// 2. Builds the AST using the core query and AST parser.
    /// 3. Resolves checks, references, and comments in the document.
    ///
    /// # Arguments
    /// * `parsers` - Language parsers (created using the `configure_parsers!` macro).
    /// * `uri` - The URI of the document to process.
    /// * `texter` - [`texter::core::text::Text`].
    ///
    /// # Returns
    /// A tuple containing the newly created [`Root`] and [`Document`].
    pub fn from_texter(
        parsers: &'static Parsers,
        uri: Url,
        texter: texter::core::text::Text,
    ) -> anyhow::Result<(Self, Document)> {
        // Parse the source code into a syntax tree.
        let tree = parsers
            .tree_sitter
            .parser
            .write()
            .parse(texter.text.as_bytes(), None)
            .ok_or_else(|| anyhow::format_err!("Tree-sitter failed to parse source code"))?;

        // Initialize the document with the source code and syntax tree.
        let document = Document { texter, tree };

        // Set up the Root with basic properties.
        let mut root = Root {
            url: Arc::new(uri.clone()),
            parsers,
            ast_diagnostics: vec![],
            lexer_diagnostics: vec![],
            ast: None,
            unsolved_checks: vec![],
            unsolved_references: vec![],
        };

        // Build the AST using the core query and AST parser function.
        root.parse(&document);

        Ok((root, document))
    }

    /// Finds the symbol at the given offset in the AST.
    pub fn descendant_at(&self, offset: usize) -> Option<DynSymbol> {
        let ast = self.ast.as_ref()?;
        if let Some(symbol) = ast.read().descendant_at(offset) {
            return Some(symbol);
        }
        if ast.read().is_inside_offset(offset) {
            return Some(ast.clone());
        }
        None
    }
}
