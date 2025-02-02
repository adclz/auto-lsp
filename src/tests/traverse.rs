use std::ops::Deref;

use crate::core::document::Document;
use crate::core::workspace::Workspace;
use auto_lsp_core::ast::{GetSymbolData, Traverse};
use lsp_types::Url;
use rstest::{fixture, rstest};

use super::html_workspace::*;

#[fixture]
fn nested_divs() -> (Workspace, Document) {
    Workspace::from_utf8(
        &HTML_PARSERS.get("html").unwrap(),
        Url::parse("file:///nested.html").unwrap(),
        r#"<!DOCTYPE html>
<div>
    <div>
         <div>
            <div></div>
        </div>
    </div>
</div>"#
            .into(),
    )
    .unwrap()
}

#[rstest]
fn descendant(nested_divs: (Workspace, Document)) {
    let ast = nested_divs.0.ast.as_ref().unwrap();

    let guard = ast.read();
    let document = guard.downcast_ref::<HtmlDocument>().unwrap();
    let div1 = &document.tags[0];

    if let Node::Element(div1) = div1.read().deref() {
        assert_eq!(div1.get_range().start, 16);
        let div2 = &div1.elements[0].read();
        assert_eq!(div2.get_range().start, 26);
        let div3 = &div2.elements[0].read();
        assert_eq!(div3.get_range().start, 41);
        let div4 = &div3.elements[0].read();
        assert_eq!(div4.get_range().start, 59);
    } else {
        panic!("Expected Element");
    };

    // Find the last div
    let descendant = ast.read().descendant_at(59);

    assert!(descendant.as_ref().unwrap().read().is::<Element>());
    assert_eq!(descendant.as_ref().unwrap().read().get_range().start, 59);
}

#[rstest]
fn descendant_at_and_collect(nested_divs: (Workspace, Document)) {
    let ast = nested_divs.0.ast.as_ref().unwrap();

    let mut collected = vec![];
    let descendant = ast.descendant_at_and_collect(
        59,
        |d| {
            if d.read().is::<Element>() {
                true
            } else if let Some(Node::Element(_)) = d.read().downcast_ref::<Node>() {
                true
            } else {
                false
            }
        },
        &mut collected,
    );

    assert!(descendant.as_ref().unwrap().read().is::<Element>());
    assert_eq!(descendant.as_ref().unwrap().read().get_range().start, 59);

    assert_eq!(collected.len(), 4);
    assert_eq!(collected[0].read().get_range().start, 16);
    assert_eq!(collected[1].read().get_range().start, 26);
    assert_eq!(collected[2].read().get_range().start, 41);
    assert_eq!(collected[3].read().get_range().start, 59);
}

#[rstest]
fn traverse_and_collect(nested_divs: (Workspace, Document)) {
    let ast = nested_divs.0.ast.as_ref().unwrap();
    let source_code = nested_divs.1.texter.text.as_bytes();

    let mut collected = vec![];
    ast.traverse_and_collect(|d| d.read().is::<TagName>(), &mut collected);

    assert_eq!(collected.len(), 4);
    assert_eq!(collected[0].read().get_text(source_code), Some("div"));
    assert_eq!(collected[1].read().get_text(source_code), Some("div"));
    assert_eq!(collected[2].read().get_text(source_code), Some("div"));
    assert_eq!(collected[3].read().get_text(source_code), Some("div"));
}
