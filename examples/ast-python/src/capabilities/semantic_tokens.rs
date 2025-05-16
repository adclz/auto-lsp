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

use crate::generated::{
    CompoundStatement, CompoundStatement_SimpleStatement, FunctionDefinition, Module,
};
use auto_lsp::core::ast::{AstNode};
use auto_lsp::core::document::Document;
use auto_lsp::core::semantic_tokens_builder::SemanticTokensBuilder;
use auto_lsp::{anyhow, define_semantic_token_modifiers, define_semantic_token_types, lsp_types};
use auto_lsp::core::dispatch;
use auto_lsp::core::salsa::db::{BaseDatabase, BaseDb, File};

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

pub fn dispatch_semantic_tokens(db: &impl BaseDatabase, file: File, node: &dyn AstNode, builder: &mut SemanticTokensBuilder) -> anyhow::Result<()> {
    dispatch!(node, [
        FunctionDefinition => build_semantic_tokens(db, file, builder)
    ]);
    Ok(())
}

impl  FunctionDefinition {
    fn build_semantic_tokens(
        &self,
        db: &impl BaseDatabase,
        file: File,
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
