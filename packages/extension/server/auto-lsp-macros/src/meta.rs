use syn::Path;

use crate::features::{
    lsp_code_lens::CodeLensFeature, lsp_completion_item::CompletionItemFeature,
    lsp_document_symbol::DocumentSymbolFeature, lsp_hover_info::HoverFeature,
    lsp_inlay_hint::InlayHintFeature, lsp_semantic_token::SemanticTokenFeature,
    scope::ScopeFeature,
};
use darling::FromMeta;

#[derive(Debug, FromMeta)]
pub struct AstStructParams {
    pub query_name: String,
    pub reference_seq: Option<bool>,
    pub features: Option<AstStructFeatures>,
}

#[derive(Debug, FromMeta)]
pub struct AstStructFeatures {
    pub scope: Option<ScopeFeature>,
    // LSP
    pub lsp_document_symbols: Option<DocumentSymbolFeature>,
    pub lsp_hover: Option<HoverFeature>,
    pub lsp_semantic_token: Option<SemanticTokenFeature>,
    pub lsp_inlay_hint: Option<InlayHintFeature>,
    pub lsp_code_lens: Option<CodeLensFeature>,
    pub lsp_completion_item: Option<CompletionItemFeature>,
}
