use crate::globals::Session;
use auto_lsp::builders::semantic_tokens::SemanticTokensBuilder;
use auto_lsp::traits::ast_item::AstItem;
use lsp_server::{RequestId, Response};
use lsp_types::{
    Position, Range, SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokens,
    SemanticTokensLegend, SemanticTokensParams, SemanticTokensRangeParams,
    SemanticTokensRangeResult, SemanticTokensResult,
};
use phf::{phf_ordered_map, OrderedMap};

pub fn get_semantic_tokens_full(
    id: RequestId,
    params: SemanticTokensParams,
    session: &Session,
) -> Response {
    let uri = params.text_document.uri.as_str();
    let workspace = session.workspaces.get(uri).unwrap();

    let mut builder = SemanticTokensBuilder::new(id.to_string());

    workspace
        .ast
        .iter()
        .for_each(|p| p.build_semantic_tokens(&mut builder));

    let tokens = builder.build();
    let result = Some(SemanticTokensResult::Tokens(tokens));
    let result = serde_json::to_value(&result).unwrap();
    Response {
        id: id.clone(),
        result: Some(result),
        error: None,
    }
}

pub fn get_semantic_tokens_range(
    id: RequestId,
    params: SemanticTokensRangeParams,
    session: &Session,
) -> Response {
    let uri = params.text_document.uri.as_str();
    let workspace = session.workspaces.get(uri).unwrap();

    let mut builder = SemanticTokensBuilder::new(id.to_string());

    workspace
        .ast
        .iter()
        .for_each(|p| p.build_semantic_tokens(&mut builder));

    let tokens = builder.build();
    let result = Some(SemanticTokensResult::Tokens(tokens));
    let result = serde_json::to_value(&result).unwrap();
    Response {
        id: id.clone(),
        result: Some(result),
        error: None,
    }
}

macro_rules! define_semantic_token_types {
    (
        standard {
            $($ts_name: expr => $standard:ident),*$(,)?
        }

    ) => {
        $(pub(crate) const $standard: SemanticTokenType = SemanticTokenType::$standard;)*

        pub(crate) const SUPPORTED_TYPES: &[SemanticTokenType] = &[
            $(SemanticTokenType::$standard,)*
        ];

        pub(crate) static TOKEN_TYPES: OrderedMap<&'static str, SemanticTokenType> = phf_ordered_map! {
            $( $ts_name => SemanticTokenType::$standard,)*
        };
    };
}

define_semantic_token_types![standard {
    "function" => FUNCTION,
    "variable" => VARIABLE,
    "keyword" => KEYWORD,
    "number" => NUMBER
}];

macro_rules! define_semantic_token_modifiers {
    (
        standard {
            $($ts_name: expr => $standard:ident),*$(,)?
        }

    ) => {

        $(pub(crate) const $standard: SemanticTokenModifier = SemanticTokenModifier::$standard;)*

        pub(crate) const SUPPORTED_MODIFIERS: &[SemanticTokenModifier] = &[
            $(SemanticTokenModifier::$standard,)*
        ];

        pub(crate) static TOKEN_MODIFIERS: OrderedMap<&'static str, SemanticTokenModifier> = phf_ordered_map! {
            $( $ts_name => SemanticTokenModifier::$standard,)*
        };
    };
}

define_semantic_token_modifiers![standard {
    "declaration" => DECLARATION,
    "static" => STATIC,
    "readonly" => READONLY,
    "deprecated" => DEPRECATED,
    "defaultLibrary" => DEFAULT_LIBRARY
}];
