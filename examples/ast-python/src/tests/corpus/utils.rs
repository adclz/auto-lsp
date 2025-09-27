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
    ($input: expr) => {{
        use ::auto_lsp::default::db::tracked::get_ast;
        use ::auto_lsp::default::db::BaseDatabase;

        let db = $crate::db::create_python_db(&[$input]);
        let file = db
            .get_file(&::auto_lsp::lsp_types::Url::parse("file:///test0.py").unwrap())
            .unwrap();

        insta::assert_debug_snapshot!(get_ast(&db, file));

        let errors =
            get_ast::accumulated::<auto_lsp::core::errors::ParseErrorAccumulator>(&db, file);
        if !errors.is_empty() {
            panic!("Errors found: {:#?}", errors);
        }
        Ok(())
    }};
}
