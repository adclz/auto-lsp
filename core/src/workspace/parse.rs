#![allow(unused)]
use crate::document::Document;

use super::Workspace;

impl Workspace {
    fn set_ast(&mut self, document: &Document) -> &mut Self {
        self.unsolved_checks.clear();
        self.unsolved_references.clear();

        let ast_parser = self.parsers.ast_parser;

        self.ast = match ast_parser(self, document, None) {
            Ok(ast) => Some(ast),
            Err(e) => {
                self.diagnostics.push(e);
                None
            }
        };
        self.set_comments(document)
            .resolve_checks(document)
            .resolve_references(document);
        #[cfg(feature = "log")]
        self.log_unsolved();

        self
    }
    /// Parses a document and updates the AST.
    ///
    /// This method assumes the document has already been updated and parsed by the tree-sitter parser.
    pub fn parse(&mut self, document: &Document) -> &mut Self {
        // Clear diagnostics
        self.diagnostics.clear();

        // Get new diagnostics from tree sitter
        Workspace::get_tree_sitter_errors(
            &document.tree.root_node(),
            document.texter.text.as_bytes(),
            &mut self.diagnostics,
        );

        // Clear AST if document is empty
        if document.texter.text.is_empty() {
            self.ast = None;
            self.unsolved_checks.clear();
            self.unsolved_references.clear();
            return self;
        }

        // Create a new AST if none exists and returns
        let root = match self.ast.clone() {
            Some(root) => root,
            None => return self.set_ast(document),
        };

        self.set_ast(document);
        self
    }

    #[cfg(feature = "log")]
    fn log_unsolved(&self) -> &Self {
        {
            if !self.unsolved_checks.is_empty() {
                log::info!("");
                log::warn!("Unsolved checks: {:?}", self.unsolved_checks.len());
            }

            if !self.unsolved_references.is_empty() {
                log::info!("");
                log::warn!("Unsolved references: {:?}", self.unsolved_references.len());
            }
            self
        }
    }
}
