use crate::core::ast::AstSymbol;
use crate::core::document::Document;
use crate::core::workspace::Workspace;
use lsp_types::Url;
use regex::Regex;
use rstest::{fixture, rstest};

use super::html_workspace::*;

#[fixture]
fn sample_file() -> (Workspace, Document) {
    Workspace::from_utf8(
        &PARSERS.get("html").unwrap(),
        Url::parse("file:///sample_file.html").unwrap(),
        r#"<!DOCTYPE html>
<script></script>
<style></style>
<div>
	<span> </span>
    <br/>
</div>"#
            .into(),
    )
    .unwrap()
}

#[rstest]
fn check_ast(sample_file: (Workspace, Document)) {
    let workspace = sample_file.0;
    let document = sample_file.1;

    let ast = workspace.ast.as_ref().unwrap();
    let ast = ast.read();

    // Root node should be HtmlDocument

    assert!(ast.is::<HtmlDocument>());
    let html = ast.downcast_ref::<HtmlDocument>().unwrap();
    let tags = &html.tags;

    // Should contain Script, Style, and Element

    assert_eq!(tags.len(), 3);
    assert!(matches!(*tags[0].read(), Node::Script(_)));
    assert!(matches!(*tags[1].read(), Node::Style(_)));
    assert!(matches!(*tags[2].read(), Node::Element(_)));

    let tag_3 = tags[2].read();

    // Checks if Element node is a div

    if let Node::Element(ref element) = *tag_3 {
        let tag_name = element.tag_name.read();
        assert_eq!(
            tag_name.get_text(document.texter.text.as_bytes()).unwrap(),
            "div"
        );

        // Checks if Element node contains 2 children (span and self closing br)

        let elements = &element.elements;
        assert_eq!(elements.len(), 2);

        // Tag name should be span

        assert_eq!(
            elements[0]
                .read()
                .tag_name
                .read()
                .get_text(document.texter.text.as_bytes())
                .unwrap(),
            "span"
        );

        // Tag name should be br

        assert_eq!(
            elements[1]
                .read()
                .tag_name
                .read()
                .get_text(document.texter.text.as_bytes())
                .unwrap(),
            "br"
        );
    } else {
        panic!("Expected Element node");
    }
}

#[fixture]
fn divs() -> (Workspace, Document) {
    Workspace::from_utf8(
        &PARSERS.get("html").unwrap(),
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
        &PARSERS.get("html").unwrap(),
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
fn comments_with_link() -> (Workspace, Document) {
    Workspace::from_utf8(
        &PARSERS.get("html").unwrap(),
        Url::parse("file:///sample_file.html").unwrap(),
        r#"<!DOCTYPE html>
<!-- source:file1.txt:52 -->         
<div>
    <!-- source:file2.txt:25 -->    
</div>"#
            .into(),
    )
    .unwrap()
}

#[rstest]
fn document_links(comments_with_link: (Workspace, Document)) {
    let workspace = comments_with_link.0;
    let document = comments_with_link.1;

    let regex = Regex::new(r" source:(\w+\.\w+):(\d+)").unwrap();
    let results = workspace.find_all_with_regex(&document, &regex);

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].0.as_str(), " source:file1.txt:52");
    assert_eq!(results[0].1, 1); // line 1
    assert_eq!(results[1].0.as_str(), " source:file2.txt:25");
    assert_eq!(results[1].1, 3); // line 3
}

#[fixture]
fn multiline_comment_with_links() -> (Workspace, Document) {
    Workspace::from_utf8(
        &PARSERS.get("html").unwrap(),
        Url::parse("file:///sample_file.html").unwrap(),
        r#"<!DOCTYPE html>
<div>
    <!-- 
        source:file1.txt:52
        source:file2.txt:25
    -->    
</div>"#
            .into(),
    )
    .unwrap()
}

#[rstest]
fn multiline_document_links(multiline_comment_with_links: (Workspace, Document)) {
    let workspace = multiline_comment_with_links.0;
    let document = multiline_comment_with_links.1;

    let regex = Regex::new(r" source:(\w+\.\w+):(\d+)").unwrap();
    let results = workspace.find_all_with_regex(&document, &regex);

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].0.as_str(), " source:file1.txt:52");
    assert_eq!(results[0].1, 3); // line 3
    assert_eq!(results[1].0.as_str(), " source:file2.txt:25");
    assert_eq!(results[1].1, 4); // line 4
}
