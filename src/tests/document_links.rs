use super::html_utils::create_html_db;
use auto_lsp_core::salsa::{db::BaseDatabase, tracked::get_ast};
use lsp_types::Url;
use regex::Regex;
use rstest::{fixture, rstest};

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
    let file = comments_with_link
        .get_file(&Url::parse("file:///test0.html").unwrap())
        .unwrap();
    let document = file.document(&comments_with_link).read();
    let root = get_ast(&comments_with_link, file).clone().into_inner();

    let regex = Regex::new(r" source:(\w+\.\w+):(\d+)").unwrap();
    let results = root.find_all_with_regex(&document, &regex);

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
    let file = multiline_comment_with_links
        .get_file(&Url::parse("file:///test0.html").unwrap())
        .unwrap();
    let document = file.document(&multiline_comment_with_links).read();
    let root = get_ast(&multiline_comment_with_links, file)
        .clone()
        .into_inner();

    let regex = Regex::new(r" source:(\w+\.\w+):(\d+)").unwrap();
    let results = root.find_all_with_regex(&document, &regex);

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].0.as_str(), " source:file1.txt:52");
    assert_eq!(results[0].1, 3); // line 3
    assert_eq!(results[1].0.as_str(), " source:file2.txt:25");
    assert_eq!(results[1].1, 4); // line 4
}
