#![allow(dead_code)]
use auto_lsp_core::{document::Document, root::Root, workspace::Workspace};
use lsp_types::Url;

use super::python_workspace::PYTHON_PARSERS;

pub fn create_python_workspace(source_code: &'static str) -> Workspace {
    let mut workspace = Workspace::default();

    let mut root = Root::from_utf8(
        PYTHON_PARSERS.get("python").unwrap(),
        Url::parse("file:///test.py").unwrap(),
        source_code.into(),
    )
    .unwrap();

    root.0.set_comments(&root.1);

    workspace
        .roots
        .insert(Url::parse("file:///test.py").unwrap(), root);

    workspace.resolve_checks();
    workspace
}

pub fn get_python_file(workspace: &Workspace) -> (&Root, &Document) {
    let (_url, (root, document)) = workspace.roots.iter().next().unwrap();
    (root, document)
}

pub fn get_mut_python_file(workspace: &mut Workspace) -> (&mut Root, &mut Document) {
    let (_url, (root, document)) = workspace.roots.iter_mut().next().unwrap();
    (root, document)
}

pub fn into_python_file(workspace: Workspace) -> (Root, Document) {
    let (_url, (root, document)) = workspace.roots.into_iter().next().unwrap();
    (root, document)
}
