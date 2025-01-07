use auto_lsp_core::symbol::{DynSymbol, WeakSymbol};
use lsp_textdocument::FullTextDocument;
use lsp_types::Diagnostic;
use tree_sitter::Tree;

use super::Parsers;

pub mod add_document;
pub mod delete_document;
pub mod edit_document;
pub mod tree_sitter_extend;

pub struct Workspace {
    pub parsers: &'static Parsers,
    pub document: FullTextDocument,
    pub errors: Vec<Diagnostic>,
    pub cst: Tree,
    pub ast: Option<DynSymbol>,
    pub unsolved_checks: Vec<WeakSymbol>,
    pub unsolved_references: Vec<WeakSymbol>,
}
