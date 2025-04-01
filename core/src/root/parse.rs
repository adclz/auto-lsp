#![allow(unused)]
use std::fmt::Display;

use salsa::Accumulator;

use crate::{
    document::Document,
    salsa::{db::BaseDatabase, tracked::DiagnosticAccumulator},
};

use super::Root;

impl Root {
    /// Parses a document and updates the AST.
    ///
    /// This method assumes the document has already been updated and parsed by the tree-sitter parser.
    pub fn parse(&mut self, db: &dyn BaseDatabase, document: &Document) -> &mut Self {
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
            None => return self.set_ast(db, document),
        };

        self.set_ast(db, document);
        self
    }

    fn set_ast(&mut self, db: &dyn BaseDatabase, document: &Document) -> &mut Self {
        self.unsolved_checks.clear();
        self.unsolved_references.clear();

        let ast_parser = self.parsers.ast_parser;

        self.ast = match ast_parser(db, self, document, None) {
            Ok(ast) => Some(ast),
            Err(e) => {
                DiagnosticAccumulator::accumulate(e.clone().into(), db);
                None
            }
        };
        self
    }
}

impl Display for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.unsolved_checks.is_empty() {
            writeln!(f, "Unsolved checks: {:?}", self.unsolved_checks.len())?;
        };

        if !self.unsolved_references.is_empty() {
            writeln!(
                f,
                "Unsolved references: {:?}",
                self.unsolved_references.len()
            )?;
        };
        Ok(())
    }
}
