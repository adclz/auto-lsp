use super::super::common::name::*;
use super::variables::*;
use auto_lsp::traits::ast_item::AstItem;
use auto_lsp::traits::ast_item_builder::AstItemBuilder;
use auto_lsp_macros::ast_struct;
use std::{
    fmt::Debug,
    sync::{Arc, RwLock},
};

#[ast_struct(
    query_name = "function",
    features(
        lsp_document_symbols(
            kind = lsp_types::SymbolKind::FUNCTION,
            strategy(
                name = self::name,
                childrens(self::inputs, self::outputs),
            ),
        ),
        lsp_semantic_token(
            token_types = crate::TOKEN_TYPES,
            token_type_index = "function",
            range = self::name
        ),
    ),
)]
pub struct Function {
    name: Name,
    test: Option<Name>,
    inputs: Vec<InputVariable>,
    outputs: Vec<OutputVariable>,
}
