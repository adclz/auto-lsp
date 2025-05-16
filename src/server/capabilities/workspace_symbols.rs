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

use auto_lsp_core::ast::AstNode;
use auto_lsp_core::salsa::db::{BaseDatabase, File};
use auto_lsp_core::{document_symbols_builder::DocumentSymbolsBuilder, salsa::tracked::get_ast};
use lsp_types::{
    DocumentSymbolResponse, Location, OneOf, WorkspaceSymbol, WorkspaceSymbolParams,
    WorkspaceSymbolResponse,
};
use crate::server::capabilities::TraversalKind;

/// Request to get root symbols
///
/// This function will return all symbols found in the root recursively
pub fn get_workspace_symbols<Db: BaseDatabase>(
    db: &Db,
    params: WorkspaceSymbolParams,
    traversal: TraversalKind,
    callback: fn(
        db: &Db,
        file: File,
        node: &dyn AstNode,
        builder: &mut DocumentSymbolsBuilder,
    ) -> anyhow::Result<()>,
) -> anyhow::Result<Option<WorkspaceSymbolResponse>> {
    if params.query.is_empty() {
        return Ok(None);
    }

    let mut symbols = vec![];

    db.get_files().iter().try_for_each(|file| {
        let file = *file;
        let url = file.url(db);

        let mut builder = DocumentSymbolsBuilder::default();

        match traversal {
            TraversalKind::Iter => {
                get_ast(db, file)
                    .iter()
                    .try_for_each(|n| callback(db, file, n.lower(), &mut builder))?;
            }
            TraversalKind::Single => {
                match get_ast(db, file).get_root() {
                    Some(f) => callback(db, file, f.lower(), &mut builder)?,
                    None => (),
                };
            }
        };

        symbols.extend(
            builder
                .finalize()
                .into_iter()
                .map(|p| WorkspaceSymbol {
                    name: p.name,
                    kind: p.kind,
                    tags: None,
                    container_name: None,
                    location: OneOf::Left(Location {
                        uri: url.to_owned(),
                        range: p.range,
                    }),
                    data: None,
                })
                .collect::<Vec<_>>(),
        );
        Ok::<(), anyhow::Error>(())
    })?;

    Ok(Some(WorkspaceSymbolResponse::Nested(symbols)))
}
