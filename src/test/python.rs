use auto_lsp_core::build::MainBuilder;
use auto_lsp_core::ast::{AstSymbol, DocumentSymbols, GetSymbolData, IsComment, SemanticTokens, StaticUpdate, Symbol, VecOrSymbol};
use auto_lsp_core::workspace::{Document, Workspace};
use auto_lsp_macros::seq;
use lsp_types::Url;
use std::sync::{Arc, LazyLock};
use texter::core::text::Text;

use crate::session::Session;
use crate::{self as auto_lsp, define_semantic_token_types};

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

define_semantic_token_types!(standard {
    "Function" => FUNCTION,
});

#[seq(query_name = "module", kind(symbol(lsp_document_symbols(user), lsp_semantic_tokens(user))))]
struct Module {
    functions: Vec<Function>,
}

impl DocumentSymbols for Module {
    fn get_document_symbols(&self, doc: &Document) -> Option<VecOrSymbol> {
        self.functions.get_document_symbols(doc)
    }
}

impl SemanticTokens for Module {
    fn build_semantic_tokens(&self, doc: &Document, builder: &mut auto_lsp_core::semantic_tokens::SemanticTokensBuilder) {
        for function in &self.functions {
            function.read().build_semantic_tokens(doc, builder);
        }
    }
}

#[seq(query_name = "function", kind(symbol(
    lsp_document_symbols( 
        code_gen(
            name = self::name,
            kind = auto_lsp::lsp_types::SymbolKind::FUNCTION,
        )
    ),
    lsp_semantic_tokens(
        code_gen(
            range = self::name,
            token_types = TOKEN_TYPES,
            token_type_index = "Function"
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

    let mut params = MainBuilder {
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
r#"# This is a comment
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

#[test]
fn check_semantic_tokens() {
    let test_file = &TEST_FILE;
    let ast = test_file.ast.as_ref().unwrap();

    let mut builder = auto_lsp_core::semantic_tokens::SemanticTokensBuilder::new("".into());
    ast.read().build_semantic_tokens(&test_file.document, &mut builder);

    let tokens = builder.build().data;

    // Tokens should be a Vec (boo and far)
    assert_eq!(tokens.len(), 2);

    assert_eq!(tokens[0].token_type, TOKEN_TYPES.get_index("Function").unwrap() as u32);
    // foo is at line 1
    assert_eq!(tokens[0].delta_line, 1);
    // char 4
    assert_eq!(tokens[0].delta_start, 4);
    assert_eq!(tokens[0].length, 3); // def

    assert_eq!(tokens[1].token_type, TOKEN_TYPES.get_index("Function").unwrap() as u32);
    // bar is at line 3
    assert_eq!(tokens[1].delta_line, 3);
    // char 4
    assert_eq!(tokens[1].delta_start, 4);
    assert_eq!(tokens[1].length, 3); // def
}
