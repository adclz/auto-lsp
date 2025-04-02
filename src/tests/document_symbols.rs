use super::python_utils::create_python_db;
use auto_lsp_core::{
    ast::BuildDocumentSymbols,
    document_symbols_builder::DocumentSymbolsBuilder,
    salsa::{db::BaseDatabase, tracked::get_ast},
};
use lsp_types::Url;
use rstest::{fixture, rstest};

#[fixture]
fn foo_bar() -> impl BaseDatabase {
    create_python_db(&[r#"# foo comment
def foo(param1, param2: int, param3: int = 5):
    pass

def bar():
    pass  
"#])
}

#[rstest]
fn foo_bar_document_symbols(foo_bar: impl BaseDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    let document = file.document(&foo_bar).read();
    let root = get_ast(&foo_bar, file).to_symbol();

    let ast = root.as_ref().unwrap();

    let mut builder = DocumentSymbolsBuilder::default();
    ast.build_document_symbols(&document, &mut builder);
    let symbols = builder.finalize();

    assert_eq!(symbols.len(), 2);

    assert_eq!(symbols[0].kind, lsp_types::SymbolKind::FUNCTION);
    assert_eq!(symbols[0].name, "foo");

    assert_eq!(symbols[1].kind, lsp_types::SymbolKind::FUNCTION);
    assert_eq!(symbols[1].name, "bar");
}

#[fixture]
fn foo_bar_nested_baz() -> impl BaseDatabase {
    create_python_db(&[r#"# foo comment
def foo(param1, param2: int, param3: int = 5):
    def baz():
        pass

def bar():
    pass  
"#])
}

#[rstest]
fn foo_bar_nested_bazdocument_symbols(foo_bar_nested_baz: impl BaseDatabase) {
    let file = foo_bar_nested_baz
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    let document = file.document(&foo_bar_nested_baz).read();
    let root = get_ast(&foo_bar_nested_baz, file).to_symbol();

    let ast = root.as_ref().unwrap();

    let mut builder = DocumentSymbolsBuilder::default();
    ast.build_document_symbols(&document, &mut builder);
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
