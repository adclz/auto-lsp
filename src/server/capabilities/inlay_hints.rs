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

use auto_lsp_core::{
    ast::AstNode,
    salsa::{db::BaseDatabase, tracked::get_ast},
};
use lsp_types::{InlayHint, InlayHintParams};
use std::ops::Deref;
use auto_lsp_core::salsa::db::File;
use crate::server::capabilities::TraversalKind;

/// Get inlay hints for a document.
pub fn get_inlay_hints<Db: BaseDatabase>(
    db: &Db,
    params: InlayHintParams,
    traversal: TraversalKind,
    callback: fn(db: &Db, file: File, node: &dyn AstNode, acc: &mut Vec<InlayHint>) -> anyhow::Result<()>,
) -> anyhow::Result<Option<Vec<InlayHint>>> {
    let mut results = vec![];

    let uri = params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    match traversal {
        TraversalKind::Iter => {
            get_ast(db, file)
                .iter() 
                .try_for_each(|n| callback(db, file, n.lower(), &mut results))?;
        }
        TraversalKind::Single => match get_ast(db, file).get_root() {
            Some(f) => callback(db, file, f.lower(), &mut results)?,
            None => return Ok(None),
        },
    };
    Ok(Some(results))
}
