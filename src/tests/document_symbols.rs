use super::python_workspace::*;
use crate::core::ast::VecOrSymbol;
use crate::core::document::Document;
use crate::core::workspace::Workspace;
use auto_lsp_core::ast::BuildDocumentSymbols;
use lsp_types::Url;
use rstest::{fixture, rstest};

#[fixture]
fn foo_bar() -> (Workspace, Document) {
    Workspace::from_utf8(
        &PYTHON_PARSERS.get("python").unwrap(),
        Url::parse("file:///test.py").unwrap(),
        r#"# foo comment
def foo(param1, param2: int, param3: int = 5):
    pass

def bar():
    pass  
"#
        .into(),
    )
    .unwrap()
}

/*
#[rstest]
fn foo_bar_document_symbols(foo_bar: (Workspace, Document)) {
    let ast = foo_bar.0.ast.as_ref().unwrap();
    let document = &foo_bar.1;

    let symbols = ast.get_document_symbols(&document).unwrap();

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
*/
