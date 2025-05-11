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

use rstest::{fixture, rstest};
use auto_lsp::core::salsa::db::BaseDatabase;
use auto_lsp::core::salsa::tracked::get_ast;
use auto_lsp::lsp_types;
use auto_lsp::lsp_types::Url;
use crate::db::create_python_db;

#[fixture]
fn foo_bar() -> impl BaseDatabase {
    create_python_db(&[
        r#"def foo():
    pass

def bar():
    pass
"#,
        r#"def foo2():
    def foo3():
        pass

def bar2():
    pass
"#,
    ])
}

#[rstest]
fn query_ast(foo_bar: impl BaseDatabase) {
    let file0 = foo_bar
        .get_file(&Url::parse("file:///test0.py").expect("Invalid URL"))
        .expect("Expected file0 to exist");

    let file1 = foo_bar
        .get_file(&Url::parse("file:///test1.py").expect("Invalid URL"))
        .expect("Expected file1 to exist");

    let file0_ast = get_ast(&foo_bar, file0).get_root();
    assert!(file0_ast.is_some());

    let file1_ast = get_ast(&foo_bar, file1).get_root();
    assert!(file1_ast.is_some());

    let logs = foo_bar.take_logs();

    assert_eq!(logs.len(), 2);
    assert!(logs[0].contains("WillExecute { database_key: get_ast(Id(0)) }"));
    assert!(logs[1].contains("WillExecute { database_key: get_ast(Id(1)) }"));
}

#[rstest]
fn update_file(mut foo_bar: impl BaseDatabase) {
    let file0 = foo_bar
        .get_file(&Url::parse("file:///test0.py").expect("Invalid URL"))
        .expect("Expected file0 to exist");

    let file1 = foo_bar
        .get_file(&Url::parse("file:///test1.py").expect("Invalid URL"))
        .expect("Expected file1 to exist");

    let file0_ast = get_ast(&foo_bar, file0).get_root();
    assert!(file0_ast.is_some());

    let file1_ast = get_ast(&foo_bar, file1).get_root();
    assert!(file1_ast.is_some());

    let logs = foo_bar.take_logs();

    assert_eq!(logs.len(), 2);
    assert!(logs[0].contains("WillExecute { database_key: get_ast(Id(0)) }"));
    assert!(logs[1].contains("WillExecute { database_key: get_ast(Id(1)) }"));

    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 0,
            },
            end: lsp_types::Position {
                line: 0,
                character: 3,
            },
        }),
        range_length: Some(3),
        text: "def".into(),
    };

    foo_bar
        .update(
            &Url::parse("file:///test0.py").expect("Invalid URL"),
            &[change],
        )
        .expect("Failed to update file");

    let file0_ast = get_ast(&foo_bar, file0).get_root();
    assert!(file0_ast.is_some());

    let file1_ast = get_ast(&foo_bar, file1).get_root();
    assert!(file1_ast.is_some());

    let logs = foo_bar.take_logs();
    assert!(logs[0].contains("WillExecute { database_key: get_ast(Id(0)) }"));
}

#[rstest]
fn remove_file(mut foo_bar: impl BaseDatabase) {
    let file0 = foo_bar
        .get_file(&Url::parse("file:///test0.py").expect("Invalid URL"))
        .expect("Expected file0 to exist");

    let file1 = foo_bar
        .get_file(&Url::parse("file:///test1.py").expect("Invalid URL"))
        .expect("Expected file1 to exist");

    let file0_ast = get_ast(&foo_bar, file0).get_root();
    assert!(file0_ast.is_some());

    let file1_ast = get_ast(&foo_bar, file1).get_root();
    assert!(file1_ast.is_some());

    let logs = foo_bar.take_logs();

    assert_eq!(logs.len(), 2);
    assert!(logs[0].contains("WillExecute { database_key: get_ast(Id(0)) }"));
    assert!(logs[1].contains("WillExecute { database_key: get_ast(Id(1)) }"));

    foo_bar
        .remove_file(&Url::parse("file:///test0.py").expect("Invalid URL"))
        .expect("Failed to remove file");

    assert_eq!(foo_bar.get_files().len(), 1);

    assert!(foo_bar
        .get_file(&Url::parse("file:///test0.py").expect("Invalid URL"))
        .is_none());
    assert!(foo_bar
        .get_file(&Url::parse("file:///test1.py").expect("Invalid URL"))
        .is_some());

    let file1_ast = get_ast(&foo_bar, file1).get_root();
    assert!(file1_ast.is_some());

    assert!(foo_bar.take_logs().is_empty());
}
