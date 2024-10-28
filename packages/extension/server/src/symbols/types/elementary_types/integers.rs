use auto_lsp::traits::ast_item::AstItem;
use auto_lsp::traits::ast_item_builder::AstItemBuilder;
use auto_lsp_macros::{ast, ast_struct};
use std::{
    fmt::Debug,
    sync::{Arc, RwLock},
};
#[ast_struct(
    query_name = "signed.integers",
    features(
        lsp_semantic_token(
            token_types = crate::capabilities::semantic_tokens::TOKEN_TYPES,
            token_type_index = "number",
            range = self,
            modifiers_fn = signed_integers_modifiers,
        ),
    )
)]
pub struct SignedInteger {}

pub fn signed_integers_modifiers() -> Vec<u32> {
    use crate::capabilities::semantic_tokens::TOKEN_MODIFIERS;
    vec![TOKEN_MODIFIERS.get_index("defaultLibrary").unwrap() as u32]
}
