use super::integers::*;
use auto_lsp::traits::ast_item::AstItem;
use auto_lsp::traits::ast_item_builder::AstItemBuilder;
use auto_lsp_macros::ast_enum;
use std::cell::RefCell;
use std::rc::Rc;

#[ast_enum]
pub enum ElementaryTypes {
    SignedInteger(SignedInteger),
}
