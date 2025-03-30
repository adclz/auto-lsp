use lsp_types::Url;
use auto_lsp_core::ast::{AstSymbol};
use rstest::{fixture, rstest};
use auto_lsp_core::salsa::db::WorkspaceDatabase;
use super::{html_utils::create_html_db, html_workspace::*};

#[fixture]
fn sample_file() -> impl WorkspaceDatabase {
    create_html_db(&[
        r#"<!DOCTYPE html>
<script></script>
<style></style>
<div>
    <span> </span>
    <br/>
</div>"#],
    )
}

#[rstest]
fn html_ast(sample_file: impl WorkspaceDatabase) {
    let file = sample_file
        .get_file(&Url::parse("file:///test0.html").unwrap())
        .unwrap();
    let document = file.document(&sample_file).read();
    let root = file.get_ast(&sample_file).clone().into_inner();

    let ast = root.ast.as_ref().unwrap().read();

    // Root node should be HtmlDocument

    assert!(ast.is::<HtmlDocument>());
    let html = ast.downcast_ref::<HtmlDocument>().unwrap();
    let tags = &html.tags;

    // Should contain DocType, Script, Style, and Element

    assert_eq!(tags.len(), 4);
    assert!(matches!(*tags[0].read(), Node::DocType(_)));
    assert!(matches!(*tags[1].read(), Node::Script(_)));
    assert!(matches!(*tags[2].read(), Node::Style(_)));
    assert!(matches!(
        *tags[3].read(),
        Node::Element(Element::FullTag(_))
    ));

    let tag_3 = tags[3].read();

    // Checks if Element node is a div

    if let Node::Element(Element::FullTag(ref element)) = *tag_3 {
        let start_tag = element.start_tag.read();
        let tag_name = start_tag.tag_name.read();
        assert_eq!(
            tag_name.get_text(document.texter.text.as_bytes()).unwrap(),
            "div"
        );

        // Checks if Element node contains 2 children (span and self closing br)

        let elements = &element.elements;
        assert_eq!(elements.len(), 2);

        // Tag name should be span

        if let Node::Element(Element::FullTag(ref element)) = *elements[0].read() {
            let start_tag = element.start_tag.read();
            let tag_name = start_tag.tag_name.read();

            assert_eq!(
                tag_name.get_text(document.texter.text.as_bytes()).unwrap(),
                "span"
            );
        } else {
            panic!("Expected Element node");
        }

        // Tag name should be br

        if let Node::Element(Element::SelfClosingTag(ref element)) = *elements[1].read() {
            assert_eq!(
                element
                    .tag_name
                    .read()
                    .get_text(document.texter.text.as_bytes())
                    .unwrap(),
                "br"
            );
        } else {
            panic!("Expected Element node");
        }
    } else {
        panic!("Expected Element node");
    }
}
