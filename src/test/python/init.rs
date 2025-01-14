use auto_lsp_core::builders::BuilderParams;
use auto_lsp_core::symbol::SymbolData;
use auto_lsp_core::workspace::Document;
use auto_lsp_macros::seq;
use lsp_types::Url;
use std::error::Error;
use std::sync::Arc;
use texter::core::text::Text;
use tree_sitter_python::LANGUAGE;

use crate as auto_lsp;
use crate::auto_lsp_core::pending_symbol::{AddSymbol, AstBuilder, Finalize, TryDownCast};
use crate::auto_lsp_core::symbol::{AstSymbol, StaticSwap, Symbol};

use crate::session::{
    init::{InitOptions, LspOptions, SemanticTokensList},
    Session,
};
use crate::{configure_parsers, define_semantic_token_modifiers, define_semantic_token_types};

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

#[seq(query_name = "module", kind(symbol()))]
struct Module {
    functions: Vec<Function>,
}

#[seq(query_name = "function", kind(symbol()))]
struct Function {
    name: FunctionName,
}

#[seq(query_name = "function.name", kind(symbol()))]
struct FunctionName {}

#[test]
fn main() {
    assert_eq!(PARSERS.len(), 1);

    let parse = PARSERS.get("python").unwrap();
    let text = Text::new(
        r#"
def foo():
    pass

def bar():
    pass    
"#
        .into(),
    );

    let tree = parse
        .cst_parser
        .parser
        .write()
        .parse(text.text.as_bytes(), None)
        .unwrap();

    let document = Document {
        document: text.clone(),
        cst: tree,
    };

    let mut params = BuilderParams {
        query: &parse.cst_parser.queries.core,
        document: &document,
        url: Arc::new(Url::parse("file:///test.py").unwrap()),
        diagnostics: &mut vec![],
        unsolved_checks: &mut vec![],
        unsolved_references: &mut vec![],
    };

    let ast_parser = parse.ast_parser;
    let ast = ast_parser(&mut params, None).unwrap();

    assert_eq!(params.diagnostics.len(), 0);
    assert_eq!(params.unsolved_checks.len(), 0);
    assert_eq!(params.unsolved_references.len(), 0);
    assert!(ast.read().is::<Module>());

    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();
    assert_eq!(module.functions.len(), 2);

    let function = module.functions[0].read();
    assert_eq!(
        function.name.read().get_text(text.text.as_bytes()).unwrap(),
        "foo"
    );

    let function = module.functions[1].read();
    assert_eq!(
        function.name.read().get_text(text.text.as_bytes()).unwrap(),
        "bar"
    );

    assert!(function.name.read().get_parent().is_some());
    let parent = function.name.read().get_parent().unwrap();
    assert!(parent.to_dyn().unwrap().read().is::<Function>());
}
