use crate::capabilities::document_symbols::document_symbols;
use crate::db::create_python_db;
use auto_lsp::default::db::BaseDatabase;
use auto_lsp::lsp_types::{
    self, DocumentSymbolParams, DocumentSymbolResponse, PartialResultParams,
};
use auto_lsp::lsp_types::{Url, WorkDoneProgressParams};
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

#[fixture]
fn params() -> DocumentSymbolParams {
    DocumentSymbolParams {
        text_document: lsp_types::TextDocumentIdentifier {
            uri: Url::parse("file:///test0.py").unwrap(),
        },
        work_done_progress_params: WorkDoneProgressParams {
            work_done_token: None,
        },
        partial_result_params: PartialResultParams {
            partial_result_token: None,
        },
    }
}

#[rstest]
fn foo_bar_document_symbols(foo_bar: impl BaseDatabase, params: DocumentSymbolParams) {
    let symbols = document_symbols(&foo_bar, params)
        .expect("Failed to build document symbols")
        .unwrap();

    let symbols = if let DocumentSymbolResponse::Nested(symbols) = symbols {
        symbols
    } else {
        panic!("Expected nested document symbols");
    };

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
fn nested_document_symbols(foo_bar_nested_baz: impl BaseDatabase, params: DocumentSymbolParams) {
    let symbols = document_symbols(&foo_bar_nested_baz, params)
        .expect("Failed to build document symbols")
        .unwrap();

    let symbols = if let DocumentSymbolResponse::Nested(symbols) = symbols {
        symbols
    } else {
        panic!("Expected nested document symbols");
    };

    assert_eq!(symbols.len(), 2);

    assert_eq!(symbols[0].kind, lsp_types::SymbolKind::FUNCTION);
    assert_eq!(symbols[0].name, "foo");

    assert_eq!(symbols[1].kind, lsp_types::SymbolKind::FUNCTION);
    assert_eq!(symbols[1].name, "bar");

    let baz_in_foo = &symbols[0].children.as_ref().unwrap()[0];

    assert_eq!(baz_in_foo.kind, lsp_types::SymbolKind::FUNCTION);
    assert_eq!(baz_in_foo.name, "baz");
}
