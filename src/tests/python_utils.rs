#![allow(dead_code)]
use super::python_workspace::PYTHON_PARSERS;
use auto_lsp_core::salsa::db::{WorkspaceDatabase, WorkspaceDb};
use lsp_types::Url;
use texter::core::text::Text;

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
