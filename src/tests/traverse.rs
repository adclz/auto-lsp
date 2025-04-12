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
use auto_lsp_core::ast::{GetSymbolData, Traverse};
use auto_lsp_core::salsa::db::BaseDatabase;
use auto_lsp_core::salsa::tracked::get_ast;
use lsp_types::Url;
use rstest::{fixture, rstest};
use std::ops::Deref;

#[fixture]
fn nested_divs() -> impl BaseDatabase {
    create_html_db(&[r#"<!DOCTYPE html>
<div>
    <div>
         <div>
            <div></div>
        </div>
    </div>
</div>"#])
}

#[rstest]
fn descendant(nested_divs: impl BaseDatabase) {
    let file = nested_divs
        .get_file(&Url::parse("file:///test0.html").unwrap())
        .unwrap();
    let root = get_ast(&nested_divs, file).to_symbol();

    let ast = root.as_ref().unwrap().read();
    let document = ast.downcast_ref::<HtmlDocument>().unwrap();
    let div1 = &document.tags[1];

    if let Node::Element(Element::FullTag(div1)) = div1.read().deref() {
        assert_eq!(div1.get_range().start, 16);
        let div2 = &div1.elements[0].read();
        if let Node::Element(Element::FullTag(div2)) = div2.deref() {
            assert_eq!(div2.get_range().start, 26);
            let div3 = &div2.elements[0].read();
            if let Node::Element(Element::FullTag(div3)) = div3.deref() {
                assert_eq!(div3.get_range().start, 41);
                let div4 = &div3.elements[0].read();
                if let Node::Element(Element::FullTag(div4)) = div4.deref() {
                    assert_eq!(div4.get_range().start, 59);
                } else {
                    panic!("Expected FullTag Element");
                }
            } else {
                panic!("Expected FullTag Element");
            }
        } else {
            panic!("Expected FullTag Element");
        };
    } else {
        panic!("Expected FullTag Element");
    };

    // Find the last element
    let descendant = ast.descendant_at(59);
    assert_eq!(descendant.as_ref().unwrap().read().get_range().start, 59);
}

#[rstest]
fn descendant_at_and_collect(nested_divs: impl BaseDatabase) {
    let file = nested_divs
        .get_file(&Url::parse("file:///test0.html").unwrap())
        .unwrap();
    let root = get_ast(&nested_divs, file).to_symbol();

    let ast = root.as_ref().unwrap();

    let mut collected = vec![];
    let descendant = ast.descendant_at_and_collect(
        59,
        |d| matches!(d.read().downcast_ref::<Node>(), Some(Node::Element(_))),
        &mut collected,
    );

    assert_eq!(descendant.as_ref().unwrap().read().get_range().start, 59);

    assert_eq!(collected.len(), 4);
    assert_eq!(collected[0].read().get_range().start, 16);
    assert_eq!(collected[1].read().get_range().start, 26);
    assert_eq!(collected[2].read().get_range().start, 41);
    assert_eq!(collected[3].read().get_range().start, 59);
}

#[rstest]
fn traverse_and_collect(nested_divs: impl BaseDatabase) {
    let file = nested_divs
        .get_file(&Url::parse("file:///test0.html").unwrap())
        .unwrap();
    let document = file.document(&nested_divs).read();
    let root = get_ast(&nested_divs, file).to_symbol();

    let ast = root.as_ref().unwrap();

    let source_code = document.texter.text.as_bytes();

    let mut collected = vec![];
    ast.traverse_and_collect(|d| d.read().is::<TagName>(), &mut collected);

    assert_eq!(collected.len(), 8);
    assert_eq!(collected[0].read().get_text(source_code).unwrap(), "div");
    assert_eq!(collected[1].read().get_text(source_code).unwrap(), "div");
    assert_eq!(collected[2].read().get_text(source_code).unwrap(), "div");
    assert_eq!(collected[3].read().get_text(source_code).unwrap(), "div");

    // end tags
    assert_eq!(collected[4].read().get_text(source_code).unwrap(), "div");
    assert_eq!(collected[5].read().get_text(source_code).unwrap(), "div");
    assert_eq!(collected[6].read().get_text(source_code).unwrap(), "div");
    assert_eq!(collected[7].read().get_text(source_code).unwrap(), "div");
}
