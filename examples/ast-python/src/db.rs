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
use auto_lsp::configure_parsers;
use auto_lsp::core::salsa::db::{BaseDatabase, BaseDb, FileManager};
use auto_lsp::lsp_types::Url;
use auto_lsp::texter::core::text::Text;
use crate::generated::Module;

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