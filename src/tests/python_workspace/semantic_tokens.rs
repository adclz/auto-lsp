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
