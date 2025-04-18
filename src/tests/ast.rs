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

use super::{html_utils::create_html_db, html_workspace::*};
use auto_lsp_core::salsa::db::BaseDatabase;
use auto_lsp_core::{ast::AstSymbol, salsa::tracked::get_ast};
use lsp_types::Url;
use rstest::{fixture, rstest};

#[fixture]
fn sample_file() -> impl BaseDatabase {
    create_html_db(&[r#"<!DOCTYPE html>
<script></script>
<style></style>
<div>
    <span> </span>
    <br/>
</div>"#])
}

#[rstest]
fn html_ast(sample_file: impl BaseDatabase) {
    let file = sample_file
        .get_file(&Url::parse("file:///test0.html").unwrap())
        .unwrap();
    let document = file.document(&sample_file).read();
    let root = get_ast(&sample_file, file).to_symbol();

    let ast = root.as_ref().unwrap().read();

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
