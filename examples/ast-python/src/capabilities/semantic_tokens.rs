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
use crate::generated::FunctionDefinition;
use auto_lsp::core::ast::AstNode;
use auto_lsp::core::dispatch;
use auto_lsp::core::semantic_tokens_builder::SemanticTokensBuilder;
use auto_lsp::default::db::tracked::get_ast;
use auto_lsp::default::db::{BaseDatabase, File};
use auto_lsp::lsp_types::{SemanticTokensParams, SemanticTokensRangeParams, SemanticTokensResult};
use auto_lsp::{anyhow, define_semantic_token_modifiers, define_semantic_token_types};

define_semantic_token_types![
    standard {
        FUNCTION,
    }

    custom {}
];

define_semantic_token_modifiers![
    standard {
        DECLARATION,
    }

    custom {}
];

pub fn semantic_tokens_full(
    db: &impl BaseDatabase,
    params: SemanticTokensParams,
) -> anyhow::Result<Option<SemanticTokensResult>> {
    let uri = params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let mut builder = SemanticTokensBuilder::new("".into());

    get_ast(db, file).iter().try_for_each(|node| {
        dispatch!(
            node.lower(),
            [
                FunctionDefinition => build_semantic_tokens(db, file, &mut builder)
            ]
        );
        anyhow::Ok(())
    })?;
    Ok(Some(SemanticTokensResult::Tokens(builder.build())))
}

pub fn semantic_tokens_range(
    db: &impl BaseDatabase,
    params: SemanticTokensRangeParams,
) -> anyhow::Result<Option<SemanticTokensResult>> {
    let uri = params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let mut builder = SemanticTokensBuilder::new("".into());

    for node in get_ast(db, file).iter() {
        if node.get_lsp_range().end <= params.range.start {
            continue;
        }
        if node.get_lsp_range().start >= params.range.end {
            break;
        }
        dispatch!(node.lower(),
            [
                FunctionDefinition => build_semantic_tokens(db, file, &mut builder)
            ]
        );
    }

    Ok(Some(SemanticTokensResult::Tokens(builder.build())))
}

impl FunctionDefinition {
    fn build_semantic_tokens(
        &self,
        _db: &impl BaseDatabase,
        _file: File,
        builder: &mut SemanticTokensBuilder,
    ) -> anyhow::Result<()> {
        builder.push(
            self.name.get_lsp_range(),
            SUPPORTED_TYPES.iter().position(|x| *x == FUNCTION).unwrap() as u32,
            SUPPORTED_MODIFIERS
                .iter()
                .position(|x| *x == DECLARATION)
                .unwrap() as u32,
        );
        Ok(())
    }
}
