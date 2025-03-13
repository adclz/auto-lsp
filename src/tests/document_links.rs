use crate::tests::html_utils::get_html_file;
use auto_lsp_core::workspace::Workspace;
use regex::Regex;
use rstest::{fixture, rstest};

use super::html_utils::create_html_workspace;

#[fixture]
fn comments_with_link() -> Workspace {
    create_html_workspace(
        r#"<!DOCTYPE html>
<!-- source:file1.txt:52 -->         
<div>
    <!-- source:file2.txt:25 -->    
</div>"#,
    )
}

#[rstest]
fn document_links(comments_with_link: Workspace) {
    let (root, document) = get_html_file(&comments_with_link);

    let regex = Regex::new(r" source:(\w+\.\w+):(\d+)").unwrap();
    let results = root.find_all_with_regex(&document, &regex);

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].0.as_str(), " source:file1.txt:52");
    assert_eq!(results[0].1, 1); // line 1
    assert_eq!(results[1].0.as_str(), " source:file2.txt:25");
    assert_eq!(results[1].1, 3); // line 3
}

#[fixture]
fn multiline_comment_with_links() -> Workspace {
    create_html_workspace(
        r#"<!DOCTYPE html>
<div>
    <!-- 
        source:file1.txt:52
        source:file2.txt:25
    -->    
</div>"#,
    )
}

#[rstest]
fn multiline_document_links(multiline_comment_with_links: Workspace) {
    let (root, document) = get_html_file(&multiline_comment_with_links);

    let regex = Regex::new(r" source:(\w+\.\w+):(\d+)").unwrap();
    let results = root.find_all_with_regex(&document, &regex);

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].0.as_str(), " source:file1.txt:52");
    assert_eq!(results[0].1, 3); // line 3
    assert_eq!(results[1].0.as_str(), " source:file2.txt:25");
    assert_eq!(results[1].1, 4); // line 4
}
