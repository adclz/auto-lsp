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

pub(crate) type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[macro_export]
macro_rules! snap {
    ($input: expr, $name: expr) => {{
        let mut settings = ::insta::Settings::clone_current();
        settings.set_snapshot_suffix(&format!("{}", stringify!(name)));
        let _guard = settings.bind_to_scope();

       use ::auto_lsp::core::salsa::db::BaseDatabase;
        use ::auto_lsp::core::salsa::tracked::get_ast;

        let db = $crate::db::create_html_db(&[$input]);
        let file = db
            .get_file(&::auto_lsp::lsp_types::Url::parse("file:///test0.py").unwrap())
            .unwrap();
        let root = get_ast(&db, file).get_root();

        let document = root.as_ref().unwrap(); ::insta::with_settings!({filters => vec![
            (r"_range: Range \{\s+start_byte: \d+,\s+end_byte: \d+,\s+start_point: Point \{\s+row: \d+,\s+column: \d+,\s+\},\s+end_point: Point \{\s+row: \d+,\s+column: \d+,\s+\},\s+\},", "[RANGE]"),
        ]}, {
            insta::assert_debug_snapshot!(document);
        });
        Ok(())
    }};
}
