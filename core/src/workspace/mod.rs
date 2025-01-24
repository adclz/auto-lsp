use std::sync::Arc;

use crate::{
    core_ast::symbol::{DynSymbol, WeakSymbol},
    core_build::stack_builder::InvokeStackBuilderFn,
    document::Document,
};
use lsp_types::{Diagnostic, Url};
use parking_lot::RwLock;
use texter::core::text::Text;
use tree_sitter::{Language, Parser, Query};

pub mod checks;
pub mod comments;
pub mod lexer;
pub mod parse;

/// Parsers available in a [`Workspace`].
///
/// Contains instances of both the [`tree_sitter`] parser and the AST parser.
pub struct Parsers {
    /// The [`TreeSitter`] parser configuration and queries.
    pub tree_sitter: TreeSitter,
    /// Function to invoke the AST parser.
    pub ast_parser: InvokeStackBuilderFn,
}

/// Tree-sitter configuration for a [`Workspace`].
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

/// Represents the workspace for parsing and analyzing a document.
///
/// This struct manages the parsing process, diagnostics, and AST for a document.
/// Note: The document text and the [`tree_sitter::Tree`] are not stored in this struct.
pub struct Workspace {
    /// The URI of the document associated with this workspace.
    pub url: Arc<Url>,
    /// Parsers used for processing the document.
    pub parsers: &'static Parsers,
    /// Diagnostics collected during parsing or analysis.
    pub diagnostics: Vec<Diagnostic>,
    /// The AST for the document, if available.
    pub ast: Option<DynSymbol>,
    /// Nodes flagged as unresolved during checks.
    pub unsolved_checks: Vec<WeakSymbol>,
    /// References flagged as unresolved during analysis.
    pub unsolved_references: Vec<WeakSymbol>,
}

impl Workspace {
    /// Creates a new [`Workspace`] and [`Document`].
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
    /// A tuple containing the newly created [`Workspace`] and [`Document`].
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
            texter: Text::new(source_code.into()),
            tree,
        };

        // Set up the workspace with basic properties.
        let mut workspace = Workspace {
            url: Arc::new(uri.clone()),
            parsers,
            diagnostics: vec![],
            ast: None,
            unsolved_checks: vec![],
            unsolved_references: vec![],
        };

        // Build the AST using the core query and AST parser function.
        workspace
            .parse(None, &document)
            // Resolve checks, references, and comments in the document.
            .resolve_checks(&document)
            .resolve_references(&document)
            .set_comments(&document)?;

        Ok((workspace, document))
    }

    /// Creates a new [`Workspace`] and [`Document`].
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
    /// A tuple containing the newly created [`Workspace`] and [`Document`].
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

        // Set up the workspace with basic properties.
        let mut workspace = Workspace {
            url: Arc::new(uri.clone()),
            parsers,
            diagnostics: vec![],
            ast: None,
            unsolved_checks: vec![],
            unsolved_references: vec![],
        };

        // Build the AST using the core query and AST parser function.
        workspace
            .parse(None, &document)
            // Resolve checks, references, and comments in the document.
            .resolve_checks(&document)
            .resolve_references(&document)
            .set_comments(&document)?;

        Ok((workspace, document))
    }
}
