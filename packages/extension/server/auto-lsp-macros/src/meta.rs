use crate::{DocumentSymbolFeature, HoverFeature, SemanticTokenFeature, UniquenessFeature};
use darling::FromMeta;
use syn::Path;

#[derive(Debug, FromMeta)]
pub struct SymbolArgs {
    pub query_name: String,
    pub features: Option<Features>,
}

#[derive(Debug, FromMeta)]
pub struct Features {
    pub uniqueness: Option<UniquenessFeature>,
    // LSP
    pub lsp_document_symbols: Option<DocumentSymbolFeature>,
    pub lsp_hover: Option<HoverFeature>,
    pub lsp_semantic_token: Option<SemanticTokenFeature>,
}
