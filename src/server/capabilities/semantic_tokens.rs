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
use auto_lsp_core::salsa::{db::BaseDatabase, tracked::get_ast};
use auto_lsp_core::semantic_tokens_builder::SemanticTokensBuilder;
use lsp_types::{DocumentSymbolResponse, SemanticTokensParams, SemanticTokensRangeParams, SemanticTokensRangeResult, SemanticTokensResult};
use auto_lsp_core::salsa::db::File;
use crate::server::capabilities::TraversalKind;

/// Get all semantic tokens for a document.
pub fn get_semantic_tokens_full<Db: BaseDatabase>(
    db: &Db,
    params: SemanticTokensParams,
    traversal: TraversalKind,
    callback: fn(db: &Db, file: File, node: &dyn AstNode, builder: &mut SemanticTokensBuilder) -> anyhow::Result<()>,
) -> anyhow::Result<Option<SemanticTokensResult>> {
    let uri = &params.text_document.uri;

    let file = db
        .get_file(uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let mut builder = SemanticTokensBuilder::new(0.to_string());

    match traversal {
        TraversalKind::Iter => {
            get_ast(db, file)
                .iter()
                .try_for_each(|n| callback(db, file, n.lower(), &mut builder))?;
        }
        TraversalKind::Single => {
            match get_ast(db, file).get_root() {
                Some(f) => callback(db, file, f.lower(), &mut builder)?,
                None => return Ok(Some(SemanticTokensResult::Tokens(builder.build()))),
            };
        }
    };
    Ok(Some(SemanticTokensResult::Tokens(builder.build())))
}

/// Get semantic tokens for a range in a document.
pub fn get_semantic_tokens_range<Db: BaseDatabase>(
    db: &Db,
    params: SemanticTokensRangeParams,
    callback: fn(db: &Db, file: File, node: &dyn AstNode, builder: &mut SemanticTokensBuilder) -> anyhow::Result<()>,
) -> anyhow::Result<Option<SemanticTokensRangeResult>> {
    let uri = &params.text_document.uri;

    let file = db
        .get_file(uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let mut builder = SemanticTokensBuilder::new(0.to_string());

    get_ast(db, file)
        .iter()
        .filter(|f| {
            let range = f.get_lsp_range();
            let start = range.start;
            let end = range.end;

            start.line >= params.range.start.line
                && start.character >= params.range.start.character
                && end.line <= params.range.end.line
                && end.character <= params.range.end.character
        })
        .try_for_each(|f| callback(db, file, f.lower(), &mut builder))?;

    Ok(Some(SemanticTokensRangeResult::Tokens(builder.build())))
}
