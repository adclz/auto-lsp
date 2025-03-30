#![allow(dead_code)]
use lsp_types::Url;
use texter::core::text::Text;
use auto_lsp_core::salsa::db::{WorkspaceDatabase, WorkspaceDb};
use super::html_workspace::HTML_PARSERS;

pub fn create_html_db(source_code: &'static [&str]) -> impl WorkspaceDatabase {
    let mut db = WorkspaceDb::default();
    source_code.iter().enumerate().for_each(|(i, source_code)| {
        let url = Url::parse(&format!("file:///test{}.html", i)).expect("Failed to parse URL");

        db.add_file_from_texter(
            HTML_PARSERS
                .get("html")
                .expect("Html parser not found"),
            &url,
            Text::new(source_code.to_string()),
        )
            .expect("Failed to add file");
    });

    db
}