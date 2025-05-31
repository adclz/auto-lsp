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

use crate::db::{create_html_db, HTML_PARSERS};
use auto_lsp::core::regex::find_all_with_regex;
use auto_lsp::default::db::tracked::get_ast;
use auto_lsp::default::db::BaseDatabase;
use auto_lsp::lsp_types::Url;
use auto_lsp::tree_sitter;
use regex::Regex;
use rstest::{fixture, rstest};

pub static COMMENT_QUERY: &str = "
(comment) @comment
";

#[fixture]
fn comments_with_link() -> impl BaseDatabase {
    create_html_db(&[r#"<!DOCTYPE html>
<!-- source:file1.txt:52 -->         
<div>
    <!-- source:file2.txt:25 -->    
</div>"#])
}

#[rstest]
fn document_links(comments_with_link: impl BaseDatabase) {
    let comment_query =
        tree_sitter::Query::new(&HTML_PARSERS.get("html").unwrap().language, COMMENT_QUERY)
            .unwrap();

    let file = comments_with_link
        .get_file(&Url::parse("file:///test0.html").unwrap())
        .unwrap();
    let document = file.document(&comments_with_link);

    let regex = Regex::new(r" source:(\w+\.\w+):(\d+)").unwrap();
    let results = find_all_with_regex(&comment_query, &document, &regex);

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].0.as_str(), " source:file1.txt:52");
    assert_eq!(results[0].1, 1); // line 1
    assert_eq!(results[1].0.as_str(), " source:file2.txt:25");
    assert_eq!(results[1].1, 3); // line 3
}

#[fixture]
fn multiline_comment_with_links() -> impl BaseDatabase {
    create_html_db(&[r#"<!DOCTYPE html>
<div>
    <!-- 
        source:file1.txt:52
        source:file2.txt:25
    -->    
</div>"#])
}

#[rstest]
fn multiline_document_links(multiline_comment_with_links: impl BaseDatabase) {
    let comment_query =
        tree_sitter::Query::new(&HTML_PARSERS.get("html").unwrap().language, COMMENT_QUERY)
            .unwrap();

    let file = multiline_comment_with_links
        .get_file(&Url::parse("file:///test0.html").unwrap())
        .unwrap();

    let document = file.document(&multiline_comment_with_links);

    let regex = Regex::new(r" source:(\w+\.\w+):(\d+)").unwrap();
    let results = find_all_with_regex(&comment_query, &document, &regex);

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].0.as_str(), " source:file1.txt:52");
    assert_eq!(results[0].1, 3); // line 3
    assert_eq!(results[1].0.as_str(), " source:file2.txt:25");
    assert_eq!(results[1].1, 4); // line 4
}
