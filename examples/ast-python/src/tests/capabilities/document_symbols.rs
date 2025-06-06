/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/
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
