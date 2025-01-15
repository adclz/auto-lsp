use auto_lsp_core::builders::BuilderParams;
use auto_lsp_core::symbol::{DocumentSymbols, IsComment, SymbolData, VecOrSymbol};
use auto_lsp_core::workspace::{Document, Workspace};
use auto_lsp_macros::seq;
use lsp_types::Url;
use std::sync::{Arc, LazyLock};
use texter::core::text::Text;

use crate::session::Session;
use crate as auto_lsp;
use crate::auto_lsp_core::symbol::{AstSymbol, StaticSwap, Symbol};

use crate::configure_parsers;

static CORE_QUERY: &'static str = "
(module) @module

(function_definition
  name: (identifier) @function.name) @function
";

static COMMENT_QUERY: &'static str = "
(comment) @comment
";

configure_parsers!(
    "python" => {
        language: tree_sitter_python::LANGUAGE,
        ast_root: Module,
        core: CORE_QUERY,
        comment: Some(COMMENT_QUERY),
        fold: None,
        highlights: None
    }
);

#[seq(query_name = "module", kind(symbol(lsp_document_symbols(user))))]
struct Module {
    functions: Vec<Function>,
}

impl DocumentSymbols for Module {
    fn get_document_symbols(&self, doc: &Document) -> Option<VecOrSymbol> {
        self.functions.get_document_symbols(doc)
    }
}

#[seq(query_name = "function", kind(symbol(
    lsp_document_symbols( 
        code_gen(
            name = self::name,
            kind = auto_lsp::lsp_types::SymbolKind::FUNCTION,
        )
    ),
    comment(user)
)))]
struct Function {
    name: FunctionName,
}

#[seq(query_name = "function.name", kind(symbol()))]
struct FunctionName {}

fn create_python_workspace(uri: Url, source_code: String) -> Workspace {
    let parse = PARSERS.get("python").unwrap();

    let tree = parse
        .cst_parser
        .parser
        .write()
        .parse(source_code.as_bytes(), None)
        .unwrap();

    let document = Document {
        document: Text::new(source_code.into()),
        cst: tree,
    };

    let mut diagnostics = vec![];
    let mut unsolved_checks = vec![];
    let mut unsolved_references = vec![];

    let mut params = BuilderParams {
        query: &parse.cst_parser.queries.core,
        document: &document,
        url: Arc::new(uri),
        diagnostics: &mut diagnostics,
        unsolved_checks: &mut unsolved_checks,
        unsolved_references: &mut unsolved_references,
    };

    let ast_parser = parse.ast_parser;
    let ast = ast_parser(&mut params, None).unwrap();

    let workspace = Workspace {
        parsers: parse,
        document,
        errors: diagnostics,
        ast: Some(ast),
        unsolved_checks,
        unsolved_references,
    };

    Session::add_comments(&workspace).unwrap();

    workspace
}

static TEST_FILE: LazyLock<Workspace> = LazyLock::new(|| {
    create_python_workspace(
        Url::parse("file:///test.py").unwrap(),
        r#"
# This is a comment
def foo():
    pass

def bar():
    pass  
"#
        .into(),
    )
});

#[test]
fn check_ast() {
    let workspace = &TEST_FILE;
    let ast = workspace.ast.as_ref().unwrap();
    let document = &workspace.document;

    // Root node should be module

    assert!(ast.read().is::<Module>());
    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    // Both bar and foo should be found
    assert_eq!(module.functions.len(), 2);
    let function = module.functions[0].read();
    assert_eq!(
        function
            .name
            .read()
            .get_text(document.document.text.as_bytes())
            .unwrap(),
        "foo"
    );

    let function = module.functions[1].read();
    assert_eq!(
        function
            .name
            .read()
            .get_text(document.document.text.as_bytes())
            .unwrap(),
        "bar"
    );

    // Checks if bar's parent is module
    assert!(function.name.read().get_parent().is_some());
    let parent = function.name.read().get_parent().unwrap();
    assert!(parent.to_dyn().unwrap().read().is::<Function>());
}

#[test]
fn check_comment() {
    let test_file = &TEST_FILE;
    let ast = test_file.ast.as_ref().unwrap();
    let document = &test_file.document;

    // Root node should be module

    assert!(ast.read().is::<Module>());
    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    let function = module.functions[0].read();
    assert!(function.is_comment());
    assert_eq!(function.get_comment(document.document.text.as_bytes()), Some("# This is a comment"));
}

#[test]
fn check_document_symbols() {
    let test_file = &TEST_FILE;
    let ast = test_file.ast.as_ref().unwrap();

    let symbols = ast
        .read()
        .get_document_symbols(&test_file.document)
        .unwrap();

    // Symbols should be a Vec (boo and far)
    assert!(matches!(symbols, VecOrSymbol::Vec(_)));

    if let VecOrSymbol::Vec(symbols) = symbols {
        assert_eq!(symbols.len(), 2);

        assert_eq!(symbols[0].kind, lsp_types::SymbolKind::FUNCTION);
        assert_eq!(symbols[0].name, "foo");

        assert_eq!(symbols[1].kind, lsp_types::SymbolKind::FUNCTION);
        assert_eq!(symbols[1].name, "bar");
    } else {
        panic!("Expected VecOrSymbol::Vec");
    }
}