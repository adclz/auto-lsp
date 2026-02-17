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

use crate::db::PYTHON_PARSERS;
use auto_lsp::default::db::file::File;
use auto_lsp::default::db::{BaseDatabase, BaseDb, FileManager};
use auto_lsp::lsp_types::{self, Url};

fn create_file(db: &BaseDb, name: &str, source: &str) -> File {
    let url = Url::parse(&format!("file:///{name}.py")).expect("Failed to parse URL");
    File::from_string()
        .db(db)
        .source(source.to_string())
        .url(&url)
        .parsers(
            PYTHON_PARSERS
                .get("python")
                .expect("Python parser not found"),
        )
        .encoding(&lsp_types::PositionEncodingKind::UTF8)
        .call()
        .expect("Failed to create file")
}

#[test]
fn on_file_added_allows() {
    let mut db = BaseDb::default();
    db.set_on_file_added_cb(Some(|_| true));

    let file = create_file(&db, "test0", "def foo(): pass");
    db.add_file(file).expect("Failed to add file");

    assert_eq!(db.get_files().len(), 1);
}

#[test]
fn on_file_added_vetoes() {
    let mut db = BaseDb::default();
    db.set_on_file_added_cb(Some(|_| false));

    let file = create_file(&db, "test0", "def foo(): pass");
    db.add_file(file).expect("add_file should return Ok even when vetoed");

    assert_eq!(db.get_files().len(), 0);
}

#[test]
fn on_file_removed_allows() {
    let mut db = BaseDb::default();

    let file = create_file(&db, "test0", "def foo(): pass");
    db.add_file(file).expect("Failed to add file");
    assert_eq!(db.get_files().len(), 1);

    db.set_on_file_removed_cb(Some(|_| true));

    db.remove_file(&Url::parse("file:///test0.py").unwrap())
        .expect("Failed to remove file");

    assert_eq!(db.get_files().len(), 0);
}

#[test]
fn on_file_removed_vetoes() {
    let mut db = BaseDb::default();

    let file = create_file(&db, "test0", "def foo(): pass");
    db.add_file(file).expect("Failed to add file");
    assert_eq!(db.get_files().len(), 1);

    db.set_on_file_removed_cb(Some(|_| false));

    db.remove_file(&Url::parse("file:///test0.py").unwrap())
        .expect("remove_file should return Ok even when vetoed");

    assert_eq!(db.get_files().len(), 1);
}

#[test]
fn no_callback_default_behavior() {
    let mut db = BaseDb::default();

    let file = create_file(&db, "test0", "def foo(): pass");
    db.add_file(file).expect("Failed to add file");
    assert_eq!(db.get_files().len(), 1);

    db.remove_file(&Url::parse("file:///test0.py").unwrap())
        .expect("Failed to remove file");
    assert_eq!(db.get_files().len(), 0);
}
