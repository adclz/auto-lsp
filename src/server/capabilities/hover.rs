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

use auto_lsp_core::salsa::{db::BaseDatabase, tracked::get_ast};
use lsp_types::{Hover, HoverParams};

/// Request to get hover information for a symbol at a position
pub fn get_hover<Db: BaseDatabase>(db: &Db, params: HoverParams) -> anyhow::Result<Option<Hover>> {
    let uri = &params.text_document_position_params.text_document.uri;

    let file = db
        .get_file(uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let document = file.document(db).read();
    let ast = get_ast(db, file);
    let position = params.text_document_position_params.position;

    let offset = document.offset_at(position).unwrap();
    let item = ast.descendant_at(offset);

    match item {
        Some(item) => item.get_hover(&document),
        None => Ok(None),
    }
}
