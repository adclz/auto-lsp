use super::db::{File, BaseDatabase};
use crate::root::Parsers;
use crate::{document::Document, root::Root};
use dashmap::{DashMap, Entry};
use lsp_types::Url;
use parking_lot::RwLock;
use salsa::{Accumulator, Setter};
use salsa::{Database, Storage};
use std::fmt::Formatter;
use std::{hash::Hash, sync::Arc};
use texter::core::text::Text;

#[salsa::tracked(no_eq, return_ref)]
pub fn get_ast<'db>(db: &'db dyn BaseDatabase, file: File) -> ParsedAst {
    let parsers = file.parsers(db);
    let doc = file.document(db);
    let url = file.url(db);

    let ast = Root::from_texter(parsers, url, doc.read().texter.clone())
        .unwrap()
        .0;

    ast.lexer_diagnostics.iter().for_each(|diag| {
        DiagnosticAccumulator::accumulate(diag.into(), db);
    });

    ast.ast_diagnostics.iter().for_each(|diag| {
        DiagnosticAccumulator::accumulate(diag.into(), db);
    });

    ParsedAst::new(ast)
}

#[salsa::accumulator]
pub struct DiagnosticAccumulator(pub lsp_types::Diagnostic);

impl From<&lsp_types::Diagnostic> for DiagnosticAccumulator {
    fn from(diagnostic: &lsp_types::Diagnostic) -> Self {
        Self(diagnostic.clone())
    }
}

impl From<lsp_types::Diagnostic> for DiagnosticAccumulator {
    fn from(diagnostic: lsp_types::Diagnostic) -> Self {
        Self(diagnostic)
    }
}

/// Cheap cloneable wrapper around a parsed AST
#[derive(Clone)]
pub struct ParsedAst {
    inner: Arc<Root>,
}

impl std::fmt::Debug for ParsedAst {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ParsedAst").field(&self.inner).finish()
    }
}

impl PartialEq for ParsedAst {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for ParsedAst {}

impl ParsedAst {
    fn new(root: Root) -> Self {
        Self {
            inner: Arc::new(root),
        }
    }

    pub fn into_inner(self) -> Arc<Root> {
        self.inner
    }
}
