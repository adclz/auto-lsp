use auto_lsp_core::{document::Document, root::Root};
use lsp_types::Url;

use super::html_workspace::HTML_PARSERS;

pub fn create_html_workspace(source_code: &'static str) -> (Root, Document) {
    let mut root = Root::from_utf8(
        HTML_PARSERS.get("html").unwrap(),
        Url::parse("file:///html.py").unwrap(),
        source_code.into(),
    )
    .unwrap();

    root.0.resolve_references(&root.1);
    root.0.resolve_checks(&root.1);
    root.0.set_comments(&root.1);
    root
}
