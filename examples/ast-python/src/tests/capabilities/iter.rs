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
use crate::db::create_python_db;
use crate::generated::{
    Block, CompoundStatement, CompoundStatement_SimpleStatement, FunctionDefinition, PassStatement,
    SimpleStatement,
};
use auto_lsp::core::ast::AstNode;
use auto_lsp::core::salsa::db::BaseDatabase;
use auto_lsp::core::salsa::tracked::get_ast;
use auto_lsp::lsp_types::Url;
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
fn sort(foo_bar: impl BaseDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    let document = file.document(&foo_bar).read();
    let source_code = document.texter.text.as_bytes();
    let ast = get_ast(&foo_bar, file);

    let items = ast
        .iter()
        .filter_map(|n| n.get_text(source_code).ok())
        .collect::<Vec<_>>();

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
            "int",
            "param3: int = 5",
            "param3",
            "int",
            "int",
            "5",
            // body
            "pass",
            "pass",
            // bar
            "def bar():\n    pass",
            "bar",
            "()",
            "pass",
            "pass"
        ]
    );
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

    match pass_statement.downcast_ref::<CompoundStatement_SimpleStatement>() {
        Some(CompoundStatement_SimpleStatement::SimpleStatement(
            SimpleStatement::PassStatement(PassStatement { .. }),
        )) => {}
        _ => panic!("Expected PassStatement"),
    }

    let pass_statement = ast.descendant_at(88).unwrap();
    match pass_statement.downcast_ref::<CompoundStatement_SimpleStatement>() {
        Some(CompoundStatement_SimpleStatement::SimpleStatement(
            SimpleStatement::PassStatement(PassStatement { .. }),
        )) => {}
        _ => panic!("Expected PassStatement"),
    }

    assert_eq!(pass_statement.get_text(source_code).unwrap(), "pass");
}
