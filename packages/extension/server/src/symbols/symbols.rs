use super::common::name::*;
use super::pous::function::*;
use super::pous::variables::*;
use super::types::elementary_types::*;
use crate::symbols::types::types::*;
use auto_lsp::traits::ast_item::AstItem;
use auto_lsp::traits::ast_item_builder::AstItemBuilder;
use auto_lsp_macros::{ast, ast_enum, ast_struct};
use elementary::*;
use integers::*;
use std::sync::{Arc, RwLock};

#[ast]
pub enum Symbol {
    Function,
    Name,
    InputVariable,
    OutputVariable,

    // Elementary types
    Types,
    ElementaryTypes,
    SignedInteger,
}
