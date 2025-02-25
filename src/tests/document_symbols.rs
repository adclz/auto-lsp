use super::python_workspace::*;
use crate::core::document::Document;
use crate::core::workspace::Workspace;
use auto_lsp_core::{ast::BuildDocumentSymbols, document_symbols_builder::DocumentSymbolsBuilder};
use lsp_types::Url;
use rstest::{fixture, rstest};

#[fixture]
fn foo_bar() -> (Workspace, Document) {
    Workspace::from_utf8(
        PYTHON_PARSERS.get("python").unwrap(),
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

#[rstest]
fn foo_bar_document_symbols(foo_bar: (Workspace, Document)) {
    let ast = foo_bar.0.ast.as_ref().unwrap();
    let document = &foo_bar.1;

    let mut builder = DocumentSymbolsBuilder::default();
    ast.build_document_symbols(document, &mut builder);
    let symbols = builder.finalize();

    assert_eq!(symbols.len(), 2);

    assert_eq!(symbols[0].kind, lsp_types::SymbolKind::FUNCTION);
    assert_eq!(symbols[0].name, "foo");

    assert_eq!(symbols[1].kind, lsp_types::SymbolKind::FUNCTION);
    assert_eq!(symbols[1].name, "bar");
}

#[fixture]
fn foo_bar_nested_baz() -> (Workspace, Document) {
    Workspace::from_utf8(
        PYTHON_PARSERS.get("python").unwrap(),
        Url::parse("file:///test.py").unwrap(),
        r#"# foo comment
def foo(param1, param2: int, param3: int = 5):
    def baz():
        pass

def bar():
    pass  
"#
        .into(),
    )
    .unwrap()
}

#[rstest]
fn foo_bar_nested_bazdocument_symbols(foo_bar_nested_baz: (Workspace, Document)) {
    let ast = foo_bar_nested_baz.0.ast.as_ref().unwrap();
    let document = &foo_bar_nested_baz.1;

    let mut builder = DocumentSymbolsBuilder::default();
    ast.build_document_symbols(document, &mut builder);
    let symbols = builder.finalize();

    assert_eq!(symbols.len(), 2);

    assert_eq!(symbols[0].kind, lsp_types::SymbolKind::FUNCTION);
    assert_eq!(symbols[0].name, "foo");

    assert_eq!(symbols[1].kind, lsp_types::SymbolKind::FUNCTION);
    assert_eq!(symbols[1].name, "bar");

    let baz_in_foo = &symbols[0].children.as_ref().unwrap()[0];

    assert_eq!(baz_in_foo.kind, lsp_types::SymbolKind::FUNCTION);
    assert_eq!(baz_in_foo.name, "baz");
}
