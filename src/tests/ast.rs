use crate::core::ast::AstSymbol;
use crate::core::document::Document;
use crate::core::workspace::Workspace;
use lsp_types::Url;
use rstest::{fixture, rstest};

use super::html_workspace::*;

#[fixture]
fn sample_file() -> (Workspace, Document) {
    Workspace::from_utf8(
        &HTML_PARSERS.get("html").unwrap(),
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
fn html_ast(sample_file: (Workspace, Document)) {
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
