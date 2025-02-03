use crate::core::document::Document;
use crate::core::workspace::Workspace;
use auto_lsp_core::ast::ChangeReport;
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

#[rstest]
fn empty_document(empty: (Workspace, Document)) {
    let mut workspace = empty.0;
    let mut document = empty.1;

    // Should not have an AST
    assert!(workspace.ast.is_none());
    assert!(workspace.diagnostics.is_empty());
    assert!(workspace.unsolved_checks.is_empty());
    assert!(workspace.unsolved_references.is_empty());

    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 0,
            },
            end: lsp_types::Position {
                line: 0,
                character: 0,
            },
        }),
        range_length: Some(26),
        text: "<div></div>".into(),
    };

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    workspace.parse(Some(&edits), &document);

    // Should have an AST
    assert!(workspace.ast.is_some());

    let html = workspace.ast.unwrap();
    let html = html.read();
    let html = html.downcast_ref::<HtmlDocument>().unwrap();
    let tags = &html.tags;

    // Should contain div

    assert_eq!(tags.len(), 1);
    assert!(matches!(*tags[0].read(), Node::Element(_)));
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
fn replace_first_parameter_name(mut foo: (Workspace, Document)) {
    let mut workspace = foo.0;
    let document = &mut foo.1;

    // Change "param1" to "p" (delete "aram1")
    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 9,
            },
            end: lsp_types::Position {
                line: 0,
                character: 14,
            },
        }),
        range_length: Some(5),
        text: "".into(),
    };

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    workspace.parse(Some(&edits), document);

    assert!(!workspace.changes.is_empty());

    // param1 is at index 0
    assert!(matches!(
        workspace.changes[0],
        ChangeReport::Remove(
            0,
            &["identifier", "typed_parameter", "typed_default_parameter"]
        )
    ));

    assert!(matches!(
        workspace.changes[1],
        ChangeReport::Insert(
            0,
            &["identifier", "typed_parameter", "typed_default_parameter"]
        )
    ));
}

#[rstest]
fn remove_last_parameter(mut foo: (Workspace, Document)) {
    let mut workspace = foo.0;
    let document = &mut foo.1;

    // Change "param3: int = 5" to ""
    // This is a complete deletion
    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 31,
            },
            end: lsp_types::Position {
                line: 0,
                character: 43,
            },
        }),
        range_length: Some(12),
        text: "".into(),
    };

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    workspace.parse(Some(&edits), document);

    assert!(!workspace.changes.is_empty());

    // param3 is at index 2
    assert!(matches!(
        workspace.changes[0],
        ChangeReport::Remove(
            2,
            &["identifier", "typed_parameter", "typed_default_parameter"]
        )
    ));

    assert!(matches!(
        workspace.changes[1],
        ChangeReport::Insert(
            2,
            &["identifier", "typed_parameter", "typed_default_parameter"]
        )
    ));
}

#[rstest]
fn replace_two_last_parameters(mut foo: (Workspace, Document)) {
    let mut workspace = foo.0;
    let document = &mut foo.1;

    // Replace 2 last parameters with the same text
    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 16,
            },
            end: lsp_types::Position {
                line: 0,
                character: 44
            },
        }),
        range_length: Some(28),
        text: "param2: int, param3: int = 5".into(),
    };

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    workspace.parse(Some(&edits), document);

    assert!(!workspace.changes.is_empty());

    assert!(matches!(
        workspace.changes[0],
        ChangeReport::Remove(
            2,
            &["identifier", "typed_parameter", "typed_default_parameter"]
        )
    ));

    assert!(matches!(
        workspace.changes[1],
        ChangeReport::Remove(
            1,
            &["identifier", "typed_parameter", "typed_default_parameter"]
        )
    ));

    assert!(matches!(
        workspace.changes[2],
        ChangeReport::Insert(
            1,
            &["identifier", "typed_parameter", "typed_default_parameter"]
        )
    ));

    assert!(matches!(
        workspace.changes[3],
        ChangeReport::Insert(
            2,
            &["identifier", "typed_parameter", "typed_default_parameter"]
        )
    ));
}
