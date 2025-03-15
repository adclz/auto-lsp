#![allow(unused)]
use darling::util::Flag;
use darling::{ast, util, FromDeriveInput, FromField, FromMeta};
use syn::{Ident, Type};

/// Struct input when `seq` macro is used
#[derive(Debug, FromDeriveInput)]
pub struct StructInput {
    pub data: ast::Data<util::Ignored, StructHelpers>,
}

#[derive(FromField, Debug)]
#[darling(attributes(ast))]
pub struct StructHelpers {
    pub ident: Option<Ident>,
    pub ty: Type,
}

#[derive(Debug, FromMeta)]
pub struct DarlingInput {
    /// The query name
    pub query: String,
    // Lsp
    #[darling(default)]
    pub declaration: Flag,
    #[darling(default)]
    pub definition: Flag,
    #[darling(default)]
    pub hover: Flag,
    #[darling(default)]
    pub document_symbols: Flag,
    #[darling(default)]
    pub code_actions: Flag,
    #[darling(default)]
    pub code_lenses: Flag,
    #[darling(default)]
    pub completions: Flag,
    #[darling(default)]
    pub triggered_completions: Flag,
    #[darling(default)]
    pub inlay_hints: Flag,
    #[darling(default)]
    pub semantic_tokens: Flag,
    // Special
    #[darling(default)]
    pub check: Flag,
    #[darling(default)]
    pub comment: Flag,
    #[darling(default)]
    pub scope: Flag,
    #[darling(default)]
    pub reference: Flag,
}
