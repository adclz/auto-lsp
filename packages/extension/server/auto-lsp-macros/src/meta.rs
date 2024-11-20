use syn::Path;

use crate::features::{
    lsp_code_lens::CodeLensFeature, lsp_completion_item::CompletionItemFeature,
    lsp_document_symbol::DocumentSymbolFeature, lsp_hover_info::HoverFeature,
    lsp_inlay_hint::InlayHintFeature, lsp_semantic_token::SemanticTokenFeature,
    scope::ScopeFeature,
};
use darling::FromMeta;

#[derive(Debug, FromMeta)]
pub enum Feature<T>
where
    T: Sized + FromMeta,
{
    User,
    CodeGen(T),
}

#[derive(Debug, FromMeta)]
pub struct AstStruct {
    pub query_name: String,
    pub kind: AstStructKind,
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
