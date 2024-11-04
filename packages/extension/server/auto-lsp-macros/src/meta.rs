use crate::features::{
    borrowable::BorrowableFeature, lsp_code_lens::CodeLensFeature,
    lsp_completion_item::CompletionItemFeature, lsp_document_symbol::DocumentSymbolFeature,
    lsp_hover_info::HoverFeature, lsp_inlay_hint::InlayHintFeature,
    lsp_semantic_token::SemanticTokenFeature,
};
use darling::FromMeta;

#[derive(Debug, FromMeta)]
pub struct SymbolArgs {
    pub query_name: String,
    pub features: Option<Features>,
}

#[derive(Debug, FromMeta)]
pub struct Features {
    pub borrowable: Option<BorrowableFeature>,
    // LSP
    pub lsp_document_symbols: Option<DocumentSymbolFeature>,
    pub lsp_hover: Option<HoverFeature>,
    pub lsp_semantic_token: Option<SemanticTokenFeature>,
    pub lsp_inlay_hint: Option<InlayHintFeature>,
    pub lsp_code_lens: Option<CodeLensFeature>,
    pub lsp_completion_item: Option<CompletionItemFeature>,
}
