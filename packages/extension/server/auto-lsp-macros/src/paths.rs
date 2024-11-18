use syn::{parse_quote, Path};

pub struct Paths {
    pub ast_item_builder_trait_path: Path,
    pub deferred_ast_item_builder: Path,
    pub semantic_tokens_builder_path: Path,
    // Features
    pub code_lens_trait_path: Path,
    pub completion_items_trait_path: Path,
    pub document_symbols_trait_path: Path,
    pub hover_info_trait_path: Path,
    pub inlay_hints_trait_path: Path,
    pub semantic_tokens_trait_path: Path,
    pub ast_item_trait_path: Path,
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            ast_item_builder_trait_path: parse_quote!(
                auto_lsp::traits::ast_item_builder::AstItemBuilder
            ),
            ast_item_trait_path: parse_quote!(auto_lsp::traits::ast_item::AstItem),
            deferred_ast_item_builder: parse_quote!(
                auto_lsp::traits::ast_item_builder::DeferredAstItemBuilder
            ),
            semantic_tokens_builder_path: parse_quote!(
                auto_lsp::builders::semantic_tokens::SemanticTokensBuilder
            ),
            code_lens_trait_path: parse_quote!(auto_lsp::traits::ast_item::CodeLens),
            completion_items_trait_path: parse_quote!(auto_lsp::traits::ast_item::CompletionItems),
            document_symbols_trait_path: parse_quote!(auto_lsp::traits::ast_item::DocumentSymbols),
            hover_info_trait_path: parse_quote!(auto_lsp::traits::ast_item::HoverInfo),
            inlay_hints_trait_path: parse_quote!(auto_lsp::traits::ast_item::InlayHints),
            semantic_tokens_trait_path: parse_quote!(auto_lsp::traits::ast_item::SemanticTokens),
        }
    }
}
