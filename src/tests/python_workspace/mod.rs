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

use ast::{Module, CORE_QUERY};

configure_parsers!(
    PYTHON_PARSERS,
    "python" => {
        language: tree_sitter_python::LANGUAGE,
        core: CORE_QUERY,
        ast_root: Module
    }
);
