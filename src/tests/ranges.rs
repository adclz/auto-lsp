use crate::core::document::Document;
use crate::core::workspace::Workspace;
use auto_lsp_core::ast::GetSymbolData;
use auto_lsp_core::document::ChangeKind;
use lsp_types::Url;
use rstest::{fixture, rstest};

use super::html_workspace::*;
use super::python_workspace::*;

#[fixture]
fn divs() -> (Workspace, Document) {
    Workspace::from_utf8(
        &HTML_PARSERS.get("html").unwrap(),
        Url::parse("file:///sample_file.html").unwrap(),
        r#"<div> </div>"#.into(),
    )
    .unwrap()
}

#[rstest]
fn insert_whitespace(divs: (Workspace, Document)) {
    let workspace = divs.0;
    let mut document = divs.1;

    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 5,
            },
            end: lsp_types::Position {
                line: 0,
                character: 5,
            },
        }),
        range_length: Some(1),
        text: " ".into(),
    };

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    assert_eq!(edits.len(), 1);
    assert!(matches!(edits[0].kind, ChangeKind::Insert));
    assert!(edits[0].is_whitespace);
}

#[rstest]
fn insert_newline(divs: (Workspace, Document)) {
    let workspace = divs.0;
    let mut document = divs.1;

    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 5,
            },
            end: lsp_types::Position {
                line: 0,
                character: 5,
            },
        }),
        range_length: Some(1),
        text: "\n".into(),
    };

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    assert_eq!(edits.len(), 1);
    assert!(matches!(edits[0].kind, ChangeKind::Insert));
    assert!(edits[0].is_whitespace);
}

#[rstest]
fn insert_tabulation(divs: (Workspace, Document)) {
    let workspace = divs.0;
    let mut document = divs.1;

    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 5,
            },
            end: lsp_types::Position {
                line: 0,
                character: 5,
            },
        }),
        range_length: Some(1),
        text: "\t".into(),
    };

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    assert_eq!(edits.len(), 1);
    assert!(matches!(edits[0].kind, ChangeKind::Insert));
    assert!(edits[0].is_whitespace);
}

#[rstest]
fn delete_whitespace(divs: (Workspace, Document)) {
    let workspace = divs.0;
    let mut document = divs.1;

    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 5,
            },
            end: lsp_types::Position {
                line: 0,
                character: 6,
            },
        }),
        range_length: Some(1),
        text: "".into(),
    };

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    assert_eq!(edits.len(), 1);
    assert!(matches!(edits[0].kind, ChangeKind::Delete));
    assert!(edits[0].is_whitespace);
}

#[fixture]
fn empty() -> (Workspace, Document) {
    Workspace::from_utf8(
        &HTML_PARSERS.get("html").unwrap(),
        Url::parse("file:///sample_file.html").unwrap(),
        r#""#.into(),
    )
    .unwrap()
}

#[fixture]
fn foo() -> (Workspace, Document) {
    Workspace::from_utf8(
        &PYTHON_PARSERS.get("python").unwrap(),
        Url::parse("file:///test.py").unwrap(),
        r#"def foo(param1, param2: int, param3: int = 5):
    pass"#
            .into(),
    )
    .unwrap()
}

#[rstest]
fn add_space_after_pass(foo: (Workspace, Document)) {
    let mut workspace = foo.0;
    let mut document = foo.1;

    let ast = workspace.ast.as_ref().unwrap();
    let ast = ast.read();
    let module = ast.downcast_ref::<Module>().unwrap();
    let foo = &module.functions[0];
    assert_eq!(foo.read().get_range(), 0..55);
    drop(ast);

    // We add a space after the pass keyword
    // Normally, this would not change the range of the function
    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 1,
                character: 8,
            },
            end: lsp_types::Position {
                line: 1,
                character: 8,
            },
        }),
        range_length: Some(0),
        text: " ".into(),
    };

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    assert!(edits[0].is_whitespace);

    workspace.parse(Some(&edits), &document);

    let ast = workspace.ast.as_ref().unwrap();
    let ast = ast.read();
    let module = ast.downcast_ref::<Module>().unwrap();
    let foo = &module.functions[0];
    assert_eq!(foo.read().get_range(), 0..55);
}

#[fixture]
fn foo_bar() -> (Workspace, Document) {
    Workspace::from_utf8(
        &PYTHON_PARSERS.get("python").unwrap(),
        Url::parse("file:///test.py").unwrap(),
        r#"def bar(param1, param2: int, param3: int = 5):
    pass

def foo():
    pass"#
            .into(),
    )
    .unwrap()
}

#[rstest]
fn add_space_between(foo_bar: (Workspace, Document)) {
    let mut workspace = foo_bar.0;
    let mut document = foo_bar.1;

    let ast = workspace.ast.as_ref().unwrap();
    let ast = ast.read();
    let module = ast.downcast_ref::<Module>().unwrap();
    let foo = &module.functions[1];
    assert_eq!(foo.read().get_range(), 57..76);
    drop(ast);

    // add a new line between the two functions
    // This should not change the range of the first function
    // but it should change the range of the second function
    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 2,
                character: 0,
            },
            end: lsp_types::Position {
                line: 2,
                character: 0,
            },
        }),
        range_length: Some(0),
        text: "\n".into(),
    };

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    assert!(edits[0].is_whitespace);

    workspace.parse(Some(&edits), &document);

    let ast = workspace.ast.as_ref().unwrap();
    let ast = ast.read();
    let module = ast.downcast_ref::<Module>().unwrap();
    let foo = &module.functions[1];
    assert_eq!(foo.read().get_range(), 58..77);
}
