use syn::{parse_quote, Path};

pub struct Paths {
    pub ast_item_trait: Path,
    pub ast_item_trait_object_arc: Path,
    pub ast_item_trait_object_weak: Path,
    pub ast_item_builder_trait: Path,
    pub deferred_ast_item_builder: Path,
    pub ast_item_builder_trait_object: Path,
    pub semantic_tokens_builder: Path,
    // Features
    pub code_lens_trait: Path,
    pub completion_items_trait: Path,
    pub document_symbols_trait: Path,
    pub hover_info_trait: Path,
    pub inlay_hints_trait: Path,
    pub semantic_tokens_trait: Path,
    pub scope_trait: Path,
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            ast_item_trait: parse_quote!(auto_lsp::traits::ast_item::AstItem),
            ast_item_trait_object_arc: parse_quote!(
                std::sync::Arc<std::sync::RwLock<dyn auto_lsp::traits::ast_item::AstItem>>
            ),
            ast_item_trait_object_weak: parse_quote!(
                std::sync::Weak<std::sync::RwLock<dyn auto_lsp::traits::ast_item::AstItem>>
            ),
            ast_item_builder_trait: parse_quote!(
                auto_lsp::traits::ast_item_builder::AstItemBuilder
            ),
            ast_item_builder_trait_object: parse_quote!(
                std::rc::Rc<
                    std::cell::RefCell<dyn auto_lsp::traits::ast_item_builder::AstItemBuilder>,
                >
            ),
            deferred_ast_item_builder: parse_quote!(
                auto_lsp::traits::ast_item_builder::DeferredAstItemBuilder
            ),
            semantic_tokens_builder: parse_quote!(
                auto_lsp::builders::semantic_tokens::SemanticTokensBuilder
            ),
            code_lens_trait: parse_quote!(auto_lsp::traits::ast_item::CodeLens),
            completion_items_trait: parse_quote!(auto_lsp::traits::ast_item::CompletionItems),
            document_symbols_trait: parse_quote!(auto_lsp::traits::ast_item::DocumentSymbols),
            hover_info_trait: parse_quote!(auto_lsp::traits::ast_item::HoverInfo),
            inlay_hints_trait: parse_quote!(auto_lsp::traits::ast_item::InlayHints),
            semantic_tokens_trait: parse_quote!(auto_lsp::traits::ast_item::SemanticTokens),
            scope_trait: parse_quote!(auto_lsp::traits::ast_item::Scope),
        }
    }
}
