use auto_lsp::{builders::BuilderFn, symbol::DynSymbol};
use lsp_textdocument::FullTextDocument;
use lsp_types::Diagnostic;
use tree_sitter::Tree;

use super::cst_parser::CstParser;

pub mod add_document;
pub mod delete_document;
pub mod edit_document;
pub mod tree_sitter_extend;

pub struct Workspace {
    pub cst_parser: &'static CstParser,
    pub ast_builder: &'static BuilderFn,
    pub document: FullTextDocument,
    pub check: Vec<DynSymbol>,
    pub errors: Vec<Diagnostic>,
    pub cst: Tree,
    pub ast: Option<DynSymbol>,
}
