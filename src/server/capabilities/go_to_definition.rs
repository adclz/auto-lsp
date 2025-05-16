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

use std::ops::Deref;

use auto_lsp_core::{
    ast::AstNode,
    salsa::{db::BaseDatabase, tracked::get_ast},
};
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse};
use auto_lsp_core::salsa::db::File;

/// Request to go to the definition of a symbol
///
/// The trait [`crate::core::ast::GetGoToDefinition`] needs to be implemented otherwise this will return None.
pub fn go_to_definition<Db: BaseDatabase>(
    db: &Db,
    params: GotoDefinitionParams,
    callback: fn(db: &Db, file: File, node: &dyn AstNode) -> anyhow::Result<Option<GotoDefinitionResponse>>,
) -> anyhow::Result<Option<GotoDefinitionResponse>> {
    let uri = &params.text_document_position_params.text_document.uri;

    let file = db
        .get_file(uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let document = file.document(db).read();

    get_ast(db, file)
        .descendant_at(
            document
                .offset_at(params.text_document_position_params.position)
                .unwrap(),
        )
        .map(|n| callback(db, file, n.lower()))
        .transpose()
        .map(|r| r.and_then(|o| o))
}
