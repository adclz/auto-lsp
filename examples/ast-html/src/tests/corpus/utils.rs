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
        let input = format!("{}", $input);
        let mut settings = ::insta::Settings::clone_current();
        settings.set_snapshot_suffix(&format!("{}", stringify!(name)));
        let _guard = settings.bind_to_scope();

        let mut p = ::auto_lsp::tree_sitter::Parser::new();
        p.set_language(&tree_sitter_html::LANGUAGE.into())
            .unwrap();

        let tree = p.parse(input, None).unwrap();
        let mut index = vec![];
        let document = $crate::generated::Document::try_from((&tree.root_node(), &mut index))?;
        ::insta::with_settings!({filters => vec![
            (r"_range: Range \{\s+start_byte: \d+,\s+end_byte: \d+,\s+start_point: Point \{\s+row: \d+,\s+column: \d+,\s+\},\s+end_point: Point \{\s+row: \d+,\s+column: \d+,\s+\},\s+\},", "[RANGE]"),
        ]}, {
            insta::assert_debug_snapshot!(document);
        });
        Ok(())
    }};
}
