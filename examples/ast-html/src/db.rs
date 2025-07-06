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
use crate::generated::Document;
use auto_lsp::default::db::{BaseDatabase, BaseDb, FileManager};
use auto_lsp::lsp_types::Url;
use auto_lsp::texter::core::text::Text;
use auto_lsp::{configure_parsers, lsp_types};

configure_parsers!(
    HTML_PARSERS,
    "html" => {
        language: tree_sitter_html::LANGUAGE,
        ast_root: Document
    }
);

pub fn create_html_db(source_code: &'static [&str]) -> impl BaseDatabase {
    let mut db = BaseDb::default();
    source_code.iter().enumerate().for_each(|(i, source_code)| {
        let url = Url::parse(&format!("file:///test{i}.html")).expect("Failed to parse URL");

        db.add_file_from_texter(
            HTML_PARSERS.get("html").expect("Html parser not found"),
            &url,
            &lsp_types::PositionEncodingKind::UTF8,
            Text::new(source_code.to_string()),
        )
        .expect("Failed to add file");
    });

    db
}
