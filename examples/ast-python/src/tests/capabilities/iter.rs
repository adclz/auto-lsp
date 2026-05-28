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
use crate::generated::{CompoundStatement_SimpleStatement, PassStatement, SimpleStatement};
use auto_lsp::default::db::BaseDatabase;
use auto_lsp::default::db::tracked::get_ast;
use auto_lsp::lsp_types::{Position, Url};
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

fn position(line: u32, character: u32) -> Position {
    Position { line, character }
}

#[rstest]
fn sort(foo_bar: impl BaseDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    let document = file.document(&foo_bar);
    let source_code = document.as_bytes();
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

    // Nodes should be sorted by their id
    // ids should be unique
    assert_eq!(
        ast.iter().map(|n| n.get_id()).collect::<Vec<_>>(),
        vec![
            // module
            0, // foo
            1, 2, // parameters
            3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, // body
            14, 15, // bar
            16, 17, 18, 19, 20
        ]
    )
}

#[rstest]
fn descendant_at(foo_bar: impl BaseDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    let document = file.document(&foo_bar);
    let source_code = document.as_bytes();
    let ast = get_ast(&foo_bar, file);

    // (2, 5) is inside the first `pass` ("    pass" on line 2)
    let pass_statement = ast
        .descendant_for_position(document, &position(2, 5))
        .unwrap();
    assert_eq!(pass_statement.get_text(source_code).unwrap(), "pass");

    match pass_statement.downcast_ref::<CompoundStatement_SimpleStatement>() {
        Some(CompoundStatement_SimpleStatement::SimpleStatement(
            SimpleStatement::PassStatement(PassStatement { .. }),
        )) => {}
        _ => panic!("Expected PassStatement"),
    }

    // (5, 6) is inside the second `pass` ("    pass  " on line 5)
    let pass_statement = ast
        .descendant_for_position(document, &position(5, 6))
        .unwrap();
    match pass_statement.downcast_ref::<CompoundStatement_SimpleStatement>() {
        Some(CompoundStatement_SimpleStatement::SimpleStatement(
            SimpleStatement::PassStatement(PassStatement { .. }),
        )) => {}
        _ => panic!("Expected PassStatement"),
    }

    assert_eq!(pass_statement.get_text(source_code).unwrap(), "pass");
}

#[rstest]
fn parents(foo_bar: impl BaseDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    let document = file.document(&foo_bar);
    let ast = get_ast(&foo_bar, file);

    // The root node should have no parent
    assert!(ast.get_root().unwrap().get_parent(ast).is_none());

    // All other nodes should have a parent
    for node in ast[1..ast.len() - 1].iter() {
        assert!(node.get_parent(ast).is_some());
    }

    // (5, 6) is inside the second `pass`
    let pass_statement = ast
        .descendant_for_position(document, &position(5, 6))
        .unwrap();
    let pass_statement_parent = pass_statement.get_parent(ast).unwrap();

    // Parent id should be different from the child id
    assert_ne!(pass_statement.get_id(), pass_statement_parent.get_id());

    // Parent id should be inferior to the child id
    assert!(pass_statement.get_id() > pass_statement_parent.get_id());
}

/// Regression test for https://github.com/adclz/auto-lsp/issues/39
///
/// When an offset falls between two siblings but still within their parent,
/// the old binary search would miss the parent node due to broken monotonicity.
#[rstest]
fn descendant_at_between_siblings(foo_bar: impl BaseDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    let document = file.document(&foo_bar);
    let source_code = document.as_bytes();
    let ast = get_ast(&foo_bar, file);

    // (3, 0) is the blank line between foo and bar, between two siblings but still
    // within the Module parent. The old binary search could miss this.
    let node = ast
        .descendant_for_position(document, &position(3, 0))
        .unwrap();
    let text = node.get_text(source_code).unwrap();
    assert!(
        text.contains("def foo") && text.contains("def bar"),
        "Expected Module node containing both functions, got: {:?}",
        text
    );

    // (1, 0) is the start of "def foo", should find foo, not the comment
    let node = ast
        .descendant_for_position(document, &position(1, 0))
        .unwrap();
    assert_eq!(
        node.get_text(source_code).unwrap(),
        "def foo(param1, param2: int, param3: int = 5):\n    pass"
    );

    // (0, 0) is the start of the comment, should find the module
    let node = ast
        .descendant_for_position(document, &position(0, 0))
        .unwrap();
    let text = node.get_text(source_code).unwrap();
    assert!(
        text.contains("# foo comment"),
        "Expected Module node at offset 0, got: {:?}",
        text
    );

    // Last line of the file, should still find a node
    assert!(
        ast.descendant_for_position(document, &position(5, 6))
            .is_some(),
        "descendant_at should find a node on the last line"
    );

    // Past the end (line out of bounds), should return None
    assert!(
        ast.descendant_for_position(document, &position(100, 0))
            .is_none(),
        "descendant_at should return None past the end"
    );
}
