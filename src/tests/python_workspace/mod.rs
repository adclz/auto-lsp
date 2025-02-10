use crate::{self as auto_lsp};
use auto_lsp::configure_parsers;

pub mod ast;
pub mod check;
pub mod code_actions;
pub mod code_lenses;
pub mod completion_items;
pub mod document_symbols;
pub mod hover;
pub mod inlay_hints;
pub mod semantic_tokens;

use ast::{Module, COMMENT_QUERY, CORE_QUERY};

configure_parsers!(
    PYTHON_PARSERS,
    "python" => {
        language: tree_sitter_python::LANGUAGE,
        node_types: tree_sitter_python::NODE_TYPES,
        ast_root: Module,
        core: CORE_QUERY,
        comment: Some(COMMENT_QUERY),
        fold: None,
        highlights: None
    }
);
