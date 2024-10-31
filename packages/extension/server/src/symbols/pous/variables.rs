use std::{
    fmt::Debug,
    sync::{Arc, RwLock},
};
use auto_lsp_macros::ast_struct;
use auto_lsp::traits::ast_item::AstItem;
use auto_lsp::traits::ast_item_builder::AstItemBuilder;

use crate::symbols::common::name::*;
use crate::symbols::types::types::*;

#[ast_struct(
    query_name = "variable.input",
    features(
        lsp_document_symbols(
            kind = lsp_types::SymbolKind::VARIABLE,
            strategy(
                name = self::name,
            ),
        ),
        lsp_semantic_token(
            token_types = crate::TOKEN_TYPES,
            token_type_index = "variable",
            range = self::name,
            modifiers_fn = io_variables_modifiers,        
        ),
    )
)]
pub struct InputVariable {
    name: Name,
    of_type: Types
}

#[ast_struct(
    query_name = "variable.output",
    features(
        lsp_document_symbols(
            kind = lsp_types::SymbolKind::VARIABLE,
            strategy(
                name = self::name,
            ),
        ),
        lsp_semantic_token(
            token_types = crate::TOKEN_TYPES,
            token_type_index = "variable",
            range = self::name,  
            modifiers_fn = io_variables_modifiers,      
        ),
    )
)]
pub struct OutputVariable {
    name: Name,
    of_type: Types
}

fn io_variables_modifiers() -> Vec<u32> {
    use crate::TOKEN_MODIFIERS;
    vec![
        TOKEN_MODIFIERS.get_index("readonly").unwrap() as u32
    ]
}