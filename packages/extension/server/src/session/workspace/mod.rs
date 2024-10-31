use std::sync::{Arc, RwLock};

use auto_lsp::traits::ast_item::AstItem;
use lsp_textdocument::FullTextDocument;
use lsp_types::Diagnostic;
use tree_sitter::Tree;

use super::parser_provider::ParserProvider;

pub mod add_document;
pub mod delete_document;
pub mod edit_document;
pub mod tree_sitter_extend;

pub struct Workspace<'a> {
    pub provider: &'a ParserProvider,
    pub document: FullTextDocument,
    pub errors: Vec<Diagnostic>,
    pub cst: Tree,
    pub ast: Vec<Arc<RwLock<dyn AstItem>>>,
}
