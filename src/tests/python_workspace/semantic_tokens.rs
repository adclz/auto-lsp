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

use super::ast::{Function, Module};
use crate::{self as auto_lsp, define_semantic_token_modifiers};
use auto_lsp::core::ast::{AstSymbol, BuildSemanticTokens};
use auto_lsp::define_semantic_token_types;
use auto_lsp_core::document::Document;
use auto_lsp_core::semantic_tokens_builder::SemanticTokensBuilder;

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

impl BuildSemanticTokens for Module {
    fn build_semantic_tokens(
        &self,
        doc: &Document,
        builder: &mut auto_lsp_core::semantic_tokens_builder::SemanticTokensBuilder,
    ) -> anyhow::Result<()> {
        self.statements.build_semantic_tokens(doc, builder)
    }
}

impl BuildSemanticTokens for Function {
    fn build_semantic_tokens(
        &self,
        doc: &Document,
        builder: &mut SemanticTokensBuilder,
    ) -> anyhow::Result<()> {
        builder.push(
            self.name.read().get_lsp_range(doc).unwrap(),
            SUPPORTED_TYPES.iter().position(|x| *x == FUNCTION).unwrap() as u32,
            SUPPORTED_MODIFIERS
                .iter()
                .position(|x| *x == DECLARATION)
                .unwrap() as u32,
        );
        Ok(())
    }
}
