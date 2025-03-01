use crate::core::document::Document;
use crate::core::root::Root;
use lsp_types::Url;
use regex::Regex;
use rstest::{fixture, rstest};

use super::html_workspace::*;

#[fixture]
fn comments_with_link() -> (Root, Document) {
    Root::from_utf8(
        HTML_PARSERS.get("html").unwrap(),
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
fn document_links(comments_with_link: (Root, Document)) {
    let root = comments_with_link.0;
    let document = comments_with_link.1;

    let regex = Regex::new(r" source:(\w+\.\w+):(\d+)").unwrap();
    let results = root.find_all_with_regex(&document, &regex);

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].0.as_str(), " source:file1.txt:52");
    assert_eq!(results[0].1, 1); // line 1
    assert_eq!(results[1].0.as_str(), " source:file2.txt:25");
    assert_eq!(results[1].1, 3); // line 3
}

#[fixture]
fn multiline_comment_with_links() -> (Root, Document) {
    Root::from_utf8(
        HTML_PARSERS.get("html").unwrap(),
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
fn multiline_document_links(multiline_comment_with_links: (Root, Document)) {
    let root = multiline_comment_with_links.0;
    let document = multiline_comment_with_links.1;

    let regex = Regex::new(r" source:(\w+\.\w+):(\d+)").unwrap();
    let results = root.find_all_with_regex(&document, &regex);

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].0.as_str(), " source:file1.txt:52");
    assert_eq!(results[0].1, 3); // line 3
    assert_eq!(results[1].0.as_str(), " source:file2.txt:25");
    assert_eq!(results[1].1, 4); // line 4
}
