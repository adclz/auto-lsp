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

use crate::server::capabilities::common::TraversalKind;
use auto_lsp_core::salsa::tracked::get_ast;
use auto_lsp_core::{ast::AstNode, salsa::db::BaseDatabase};
use lsp_types::{CompletionContext, CompletionItem, CompletionParams, CompletionResponse};
use auto_lsp_core::salsa::db::File;

pub fn get_completion_items<Db: BaseDatabase>(
    db: &Db,
    params: CompletionParams,
    traversal: TraversalKind,
    callback: fn(
        db: &Db,
        file: File,
        node: &dyn AstNode,
        params: &Option<CompletionContext>,
        acc: &mut Vec<CompletionItem>,
    ) -> anyhow::Result<()>,
) -> anyhow::Result<Option<CompletionResponse>> {
    let mut results = vec![];

    let uri = &params.text_document_position.text_document.uri;

    let file = db
        .get_file(uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    match traversal {
        TraversalKind::Iter => {
            get_ast(db, file)
                .iter()
                .try_for_each(|n| callback(db, file, n.lower(), &params.context, &mut results))?;
        }
        TraversalKind::Single => match get_ast(db, file).get_root() {
            Some(f) => callback(db, file, f.lower(), &params.context, &mut results)?,
            None => return Ok(None),
        },
    };

    Ok(Some(results.into()))
}
