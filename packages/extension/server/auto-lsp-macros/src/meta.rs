use syn::{Ident, Path, Type};

use crate::features::{
    duplicate::DuplicateCheck, lsp_code_lens::CodeLensFeature,
    lsp_completion_item::CompletionItemFeature, lsp_document_symbol::DocumentSymbolFeature,
    lsp_hover_info::HoverFeature, lsp_inlay_hint::InlayHintFeature,
    lsp_semantic_token::SemanticTokenFeature, scope::ScopeFeature,
};
use darling::{ast, util, FromDeriveInput, FromField, FromMeta};

#[derive(Debug, FromDeriveInput)]
pub struct StructInput {
    pub data: ast::Data<util::Ignored, StructHelpers>,
}

#[derive(Debug, FromMeta)]
pub struct UserFeatures {
    pub query_name: String,
    pub kind: AstStructKind,
}

#[derive(FromField, Debug)]
#[darling(attributes(ast))]
pub struct StructHelpers {
    pub ident: Option<Ident>,
    pub ty: Type,
    #[darling(default)]
    pub dup: Option<DuplicateCheck>,
}

#[derive(Debug, FromMeta)]
pub enum Feature<T>
where
    T: Sized + FromMeta,
{
    User,
    CodeGen(T),
}

#[derive(Debug, FromMeta)]
pub enum AstStructKind {
    Accessor,
    Symbol(SymbolFeatures),
}

#[derive(Debug, FromMeta)]
pub struct SymbolFeatures {
    pub scope: Option<Feature<ScopeFeature>>,
    // LSP
    pub lsp_document_symbols: Option<Feature<DocumentSymbolFeature>>,
    pub lsp_hover_info: Option<Feature<HoverFeature>>,
    pub lsp_semantic_tokens: Option<Feature<SemanticTokenFeature>>,
    pub lsp_inlay_hints: Option<Feature<InlayHintFeature>>,
    pub lsp_code_lens: Option<Feature<CodeLensFeature>>,
    pub lsp_completion_items: Option<Feature<CompletionItemFeature>>,
}
