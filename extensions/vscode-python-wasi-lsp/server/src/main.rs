use std::error::Error;

use auto_lsp::session::{
    init::{InitOptions, LspOptions, SemanticTokensList},
    Session,
};
use auto_lsp::{configure_parsers, define_semantic_token_modifiers, define_semantic_token_types};

mod ast;

use crate::ast::{Module, ModuleBuilder};

// List of semantic token types
define_semantic_token_types![standard {
    "function" => FUNCTION,
    "variable" => VARIABLE,
    "keyword" => KEYWORD,
    "number" => NUMBER
}];

// List of semantic token modifiers
define_semantic_token_modifiers![standard {
    "declaration" => DECLARATION,
    "static" => STATIC,
    "readonly" => READONLY,
    "deprecated" => DEPRECATED,
    "defaultLibrary" => DEFAULT_LIBRARY
}];

static CORE_QUERY: &'static str = "
(module) @module

(function_definition
  name: (identifier) @function.name) @function
";

configure_parsers!(
    "python" => {
        language: tree_sitter_python::LANGUAGE,
        ast_root: Module,
        core: CORE_QUERY,
        comment: None,
        fold: None,
        highlights: None
    }
);

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut session = Session::create(InitOptions {
        parsers: &PARSERS,
        lsp_options: LspOptions {
            document_symbols: true,
            diagnostics: true,
            semantic_tokens: Some(SemanticTokensList {
                semantic_token_modifiers: Some(&SUPPORTED_MODIFIERS),
                semantic_token_types: Some(&SUPPORTED_TYPES),
            }),
            inlay_hints: true,
            ..Default::default()
        },
    })?;

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    session.main_loop()?;
    session.io_threads.join()?;

    // Shut down gracefully.
    eprintln!("Shutting down server");
    Ok(())
}
