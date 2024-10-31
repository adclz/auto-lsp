use auto_lsp::builders::semantic_tokens::SemanticTokensBuilder;
use auto_lsp::traits::ast_item::AstItem;

use lsp_types::{SemanticTokensParams, SemanticTokensRangeParams, SemanticTokensResult};

use crate::session::Session;

impl<'a> Session<'a> {
    pub fn get_semantic_tokens_full(
        &mut self,
        params: SemanticTokensParams,
    ) -> anyhow::Result<SemanticTokensResult> {
        let uri = &params.text_document.uri;
        let workspace = self.workspaces.get(uri).unwrap();

        let mut builder = SemanticTokensBuilder::new(0.to_string());

        workspace
            .ast
            .iter()
            .for_each(|p| p.build_semantic_tokens(&mut builder));

        Ok(SemanticTokensResult::Tokens(builder.build()))
    }

    pub fn get_semantic_tokens_range(
        &mut self,
        params: SemanticTokensRangeParams,
    ) -> anyhow::Result<SemanticTokensResult> {
        let uri = &params.text_document.uri;
        let workspace = self.workspaces.get(uri).unwrap();

        let mut builder = SemanticTokensBuilder::new(0.to_string());

        workspace
            .ast
            .iter()
            .for_each(|p| p.build_semantic_tokens(&mut builder));

        Ok(SemanticTokensResult::Tokens(builder.build()))
    }
}

#[macro_export]
macro_rules! define_semantic_token_types {
    (
        standard {
            $($ts_name: expr => $standard:ident),*$(,)?
        }

    ) => {
        $(pub const $standard: lsp_types::SemanticTokenType = lsp_types::SemanticTokenType::$standard;)*

        pub const SUPPORTED_TYPES: &[lsp_types::SemanticTokenType] = &[
            $(lsp_types::SemanticTokenType::$standard,)*
        ];

        pub static TOKEN_TYPES: phf::OrderedMap<&'static str, lsp_types::SemanticTokenType> = phf::phf_ordered_map! {
            $( $ts_name => lsp_types::SemanticTokenType::$standard,)*
        };
    };
}

#[macro_export]
macro_rules! define_semantic_token_modifiers {
    (
        standard {
            $($ts_name: expr => $standard:ident),*$(,)?
        }

    ) => {

        $(pub const $standard: lsp_types::SemanticTokenModifier = lsp_types::SemanticTokenModifier::$standard;)*

        pub const SUPPORTED_MODIFIERS: &[lsp_types::SemanticTokenModifier] = &[
            $(lsp_types::SemanticTokenModifier::$standard,)*
        ];

        pub static TOKEN_MODIFIERS: phf::OrderedMap<&'static str, lsp_types::SemanticTokenModifier> = phf::phf_ordered_map! {
            $( $ts_name => lsp_types::SemanticTokenModifier::$standard,)*
        };
    };
}
