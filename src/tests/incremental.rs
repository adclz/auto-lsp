use crate::core::document::Document;
use crate::core::workspace::Workspace;
use lsp_types::Url;
use rstest::{fixture, rstest};

use super::html_workspace::*;

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
    assert!(edits[0].1);
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
    assert!(edits[0].1);
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
    assert!(edits[0].1);
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
    assert!(edits[0].1);
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
