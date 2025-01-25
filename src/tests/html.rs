use crate::core::ast::AstSymbol;
use crate::core::document::Document;
use crate::core::workspace::Workspace;
use crate::{self as auto_lsp};
use crate::{choice, seq};
use lsp_types::Url;
use rstest::{fixture, rstest};

use crate::configure_parsers;

static CORE_QUERY: &'static str = "
(document (doctype) @doctype) @document
    
(element (start_tag (tag_name) @tag_name) (end_tag)) @element
(element (self_closing_tag (tag_name) @tag_name)) @element
(script_element (start_tag (tag_name) @tag_name) (end_tag)) @script_tag
(style_element (start_tag (tag_name) @tag_name) (end_tag)) @style_tag
";

configure_parsers!(
    "html" => {
        language: tree_sitter_html::LANGUAGE,
        node_types: tree_sitter_html::NODE_TYPES,
        ast_root: HtmlDocument,
        core: CORE_QUERY,
        comment: None,
        fold: None,
        highlights: None
    }
);

#[seq(query_name = "document", kind(symbol()))]
pub struct HtmlDocument {
    doctype: Option<DocType>,
    tags: Vec<Node>,
}

#[seq(query_name = "doctype", kind(symbol()))]
pub struct DocType {}

#[choice]
pub enum Node {
    Element(Element),
    Script(Script),
    Style(Style),
}

#[seq(query_name = "element", kind(symbol()))]
pub struct Element {
    tag_name: TagName,
    elements: Vec<Element>,
}

#[seq(query_name = "tag_name", kind(symbol()))]
pub struct TagName {}

#[seq(query_name = "script_tag", kind(symbol()))]
pub struct Script {}

#[seq(query_name = "style_tag", kind(symbol()))]
pub struct Style {}

#[fixture]
fn sample_file() -> (Workspace, Document) {
    Workspace::from_utf8(
        &PARSERS.get("html").unwrap(),
        Url::parse("file:///sample_file.py").unwrap(),
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
        Url::parse("file:///sample_file.py").unwrap(),
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
