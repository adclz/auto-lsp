#![allow(dead_code)]
use auto_lsp_core::{document::Document, root::Root, workspace::Workspace};
use lsp_types::Url;

use super::html_workspace::HTML_PARSERS;

pub fn create_html_workspace(source_code: &'static str) -> Workspace {
    let mut workspace = Workspace::default();

    let mut root = Root::from_utf8(
        HTML_PARSERS.get("html").unwrap(),
        Url::parse("file:///test.html").unwrap(),
        source_code.into(),
    )
    .unwrap();
    root.0.set_comments(&root.1);

    workspace
        .roots
        .insert(Url::parse("file:///test.html").unwrap(), root);

    workspace.resolve_references();
    workspace.resolve_checks();
    workspace
}

pub fn get_html_file(workspace: &Workspace) -> (&Root, &Document) {
    let (_url, (root, document)) = workspace.roots.iter().next().unwrap();
    (&root, &document)
}
