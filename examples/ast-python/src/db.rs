use std::sync::{Arc, Mutex};

/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/
use crate::generated::Module;
use auto_lsp::default::db::file::File;
use auto_lsp::default::db::{BaseDatabase, BaseDb, FileManager};
use auto_lsp::lsp_types::Url;
use auto_lsp::{configure_parsers, lsp_types};

configure_parsers!(
    PYTHON_PARSERS,
    "python" => {
        language: tree_sitter_python::LANGUAGE,
        ast_root: Module
    }
);

pub fn create_python_db(source_code: &'static [&str]) -> impl BaseDatabase {
    let mut db = BaseDb::default();
    source_code.iter().enumerate().for_each(|(i, source_code)| {
        let url = Url::parse(&format!("file:///test{i}.py")).expect("Failed to parse URL");

        let file = File::from_string()
            .db(&db)
            .source(source_code.to_string())
            .url(&url)
            .parsers(
                PYTHON_PARSERS
                    .get("python")
                    .expect("Python parser not found"),
            )
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
            .parsers(
                PYTHON_PARSERS
                    .get("python")
                    .expect("Python parser not found"),
            )
            .encoding(&lsp_types::PositionEncodingKind::UTF8)
            .call()
            .expect("Failed to create file");

        db.add_file(file).expect("Failed to add file");
    });

    (db, logs)
}
