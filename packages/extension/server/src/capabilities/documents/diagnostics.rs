use std::str::FromStr;

use crate::{globals::Session, tree_sitter_extend::tree_sitter_lexer};
use lsp_server::Connection;
use lsp_types::{Diagnostic, Url};
use tree_sitter::Tree;

pub fn analyze_document(cst: &Tree, source_code: &[u8]) -> Vec<Diagnostic> {
    tree_sitter_lexer::get_tree_sitter_errors(&cst.root_node(), source_code)
}
