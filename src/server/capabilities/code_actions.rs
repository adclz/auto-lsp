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

use auto_lsp_core::salsa::db::BaseDatabase;
use auto_lsp_core::{ast::BuildCodeActions, salsa::tracked::get_ast};
use lsp_types::{CodeActionOrCommand, CodeActionParams};

pub fn get_code_actions<Db: BaseDatabase>(
    db: &Db,
    params: CodeActionParams,
) -> anyhow::Result<Option<Vec<CodeActionOrCommand>>> {
    let mut results = vec![];

    let uri = params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let document = file.document(db).read();
    let root = get_ast(db, file).get_root();

    if let Some(root) = root {
        root.build_code_actions(&document, &mut results)?
    }

    Ok(Some(results))
}
