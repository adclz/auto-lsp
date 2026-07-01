use std::sync::{Arc, Mutex};

use crate::generated::Module;
use auto_lsp::default::db::file::File;
use auto_lsp::default::db::{BaseDatabase, BaseDb, FileManager};
use auto_lsp::lsp_types::Url;
use auto_lsp::{configure_parser, lsp_types};

configure_parser!(
    PYTHON,
        language: tree_sitter_python::LANGUAGE,
        ast_root: Module
);

pub fn create_python_db(source_code: &'static [&str]) -> impl BaseDatabase {
    let mut db = BaseDb::default();
    source_code.iter().enumerate().for_each(|(i, source_code)| {
        let url = Url::parse(&format!("file:///test{i}.py")).expect("Failed to parse URL");

        let file = File::from_string()
            .db(&db)
            .source(source_code.to_string())
            .url(&url)
            .parsers(&PYTHON)
            .encoding(&lsp_types::PositionEncodingKind::UTF8)
            .call()
            .expect("Failed to create file");

        db.add_file(file).expect("Failed to add file");
    });

    db
}

pub fn create_python_db_with_logger(
    source_code: &'static [&str],
) -> (impl BaseDatabase, Arc<Mutex<Vec<String>>>) {
    let logs = Arc::new(Mutex::new(Vec::new()));
    let logs_clone = logs.clone();
    let mut db = BaseDb::with_logger(Some(Box::new(move |event| {
        if let salsa::EventKind::WillExecute { .. } = event.kind {
            logs_clone.lock().unwrap().push(format!("{event:?}"));
        }
    })));

    source_code.iter().enumerate().for_each(|(i, source_code)| {
        let url = Url::parse(&format!("file:///test{i}.py")).expect("Failed to parse URL");

        let file = File::from_string()
            .db(&db)
            .source(source_code.to_string())
            .url(&url)
            .parsers(&PYTHON)
            .encoding(&lsp_types::PositionEncodingKind::UTF8)
            .call()
            .expect("Failed to create file");

        db.add_file(file).expect("Failed to add file");
    });

    (db, logs)
}
