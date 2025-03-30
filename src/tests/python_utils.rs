#![allow(dead_code)]
use super::python_workspace::PYTHON_PARSERS;
use auto_lsp_core::salsa::db::{WorkspaceDatabase, WorkspaceDb};
use auto_lsp_core::{document::Document, root::Root, workspace::Workspace};
use lsp_types::Url;
use texter::core::text::Text;

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

    workspace
}

pub fn create_python_workspace2(source_code: &'static [&str]) -> Workspace {
    let mut workspace = Workspace::default();

    source_code.iter().enumerate().for_each(|(i, source_code)| {
        let url = Url::parse(&format!("file:///test{}.py", i)).unwrap();
        let mut root = Root::from_utf8(
            PYTHON_PARSERS.get("python").unwrap(),
            url.clone(),
            source_code.to_string(),
        )
        .unwrap();

        root.0.set_comments(&root.1);

        workspace.roots.insert(url.clone(), root);
    });

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

pub fn create_python_db(source_code: &'static [&str]) -> impl WorkspaceDatabase {
    let mut db = WorkspaceDb::default();
    source_code.iter().enumerate().for_each(|(i, source_code)| {
        let url = Url::parse(&format!("file:///test{}.py", i)).expect("Failed to parse URL");

        db.add_file_from_texter(
            PYTHON_PARSERS
                .get("python")
                .expect("Python parser not found"),
            &url,
            Text::new(source_code.to_string()),
        )
        .expect("Failed to add file");
    });

    db
}
