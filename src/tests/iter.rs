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
use auto_lsp_core::salsa::{db::BaseDatabase, tracked::get_ast};
use lsp_types::Url;
use rstest::{fixture, rstest};

use super::python_utils::create_python_db;
use crate::python::ast::{CompoundStatement, Function, PassStatement, SimpleStatement, Statement};

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
fn sort(foo_bar: impl BaseDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    let document = file.document(&foo_bar).read();
    let source_code = document.texter.text.as_bytes();
    let ast = get_ast(&foo_bar, file);

    // Nodes should be sorted by their position in the source code
    assert_eq!(
        ast.iter()
            .filter_map(|n| n.get_text(source_code).ok())
            .collect::<Vec<_>>(),
        vec![
            // module
            "# foo comment\ndef foo(param1, param2: int, param3: int = 5):\n    pass\n\ndef bar():\n    pass  \n",
            // foo
            "def foo(param1, param2: int, param3: int = 5):\n    pass",
            "foo",
            // parameters
            "(param1, param2: int, param3: int = 5)",
            "param1",
            "param2: int",
            "param2",
            "int",
            "param3: int = 5",
            "param3",
            "int",
            "5",
            // body
            "pass",
            // bar
            "def bar():\n    pass",
            "bar",
            "()",
            "pass",
        ]
    );
}

#[rstest]
fn ids(foo_bar: impl BaseDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    let ast = get_ast(&foo_bar, file);

    // All ids should be unique
    let mut id = 0;
    for node in ast.iter() {
        assert_eq!(node.get_data().id, id);
        id += 1;
    }
}

#[rstest]
fn descendant_at(foo_bar: impl BaseDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    let document = file.document(&foo_bar).read();
    let source_code = document.texter.text.as_bytes();
    let ast = get_ast(&foo_bar, file);

    let pass_statement = ast.descendant_at(66).unwrap();
    assert_eq!(pass_statement.get_text(source_code).unwrap(), "pass");

    match pass_statement.downcast_ref::<Statement>() {
        Some(Statement::Simple(SimpleStatement::PassStatement(PassStatement { .. }))) => {}
        _ => panic!("Expected PassStatement"),
    }

    let pass_statement = ast.descendant_at(88).unwrap();
    match pass_statement.downcast_ref::<Statement>() {
        Some(Statement::Simple(SimpleStatement::PassStatement(PassStatement { .. }))) => {}
        _ => panic!("Expected PassStatement"),
    }

    assert_eq!(pass_statement.get_text(source_code).unwrap(), "pass");
}

#[rstest]
fn parents(foo_bar: impl BaseDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    let ast = get_ast(&foo_bar, file);

    // The root node should have no parent
    assert!(ast.get_root().unwrap().get_parent(ast).is_none());

    // All other nodes should have a parent
    for node in ast[1..ast.len() - 1].iter() {
        assert!(node.get_parent(ast).is_some());
    }

    let pass_statement = ast.descendant_at(88).unwrap();
    let pass_statement_parent = pass_statement.get_parent(ast).unwrap();

    // Parent id should be different from the child id
    assert_ne!(
        pass_statement.get_data().id,
        pass_statement_parent.get_data().id
    );

    // Parent id should be inferior to the child id
    assert!(pass_statement.get_data().id > pass_statement_parent.get_data().id);

    match pass_statement_parent.downcast_ref::<Statement>() {
        Some(Statement::Compound(CompoundStatement::Function(Function { .. }))) => {}
        _ => panic!("Expected Function as parent"),
    }
}
