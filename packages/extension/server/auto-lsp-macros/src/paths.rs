use syn::{parse_quote, Path};

pub struct Paths {
    pub semantic_tokens_builder: Path,
    // new types idioms
    pub symbol: Path,
    pub dyn_symbol: Path,
    pub weak_symbol: Path,
    pub pending_symbol: Path,
    pub maybe_pending_symbol: Path,

    // traits
    pub queryable: Path,
    pub locator: Path,
    pub parent: Path,
    pub check_duplicate: Path,

    pub ast_item_trait: Path,
    pub ast_item_builder_trait: Path,
    pub code_lens_trait: Path,
    pub completion_items_trait: Path,
    pub document_symbols_trait: Path,
    pub hover_info_trait: Path,
    pub inlay_hints_trait: Path,
    pub semantic_tokens_trait: Path,
    pub scope_trait: Path,
    pub is_accessor_trait: Path,
    pub accessor_trait: Path,
    pub try_into_builder: Path,
    pub try_from_builder: Path,
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            semantic_tokens_builder: parse_quote!(auto_lsp::semantic_tokens::SemanticTokensBuilder),

            // new types idioms
            symbol: parse_quote!(auto_lsp::ast_item::Symbol),
            dyn_symbol: parse_quote!(auto_lsp::ast_item::DynSymbol),
            weak_symbol: parse_quote!(auto_lsp::ast_item::WeakSymbol),
            pending_symbol: parse_quote!(auto_lsp::ast_item_builder::PendingSymbol),
            maybe_pending_symbol: parse_quote!(auto_lsp::ast_item_builder::MaybePendingSymbol),

            // traits
            queryable: parse_quote!(auto_lsp::queryable::Queryable),
            locator: parse_quote!(auto_lsp::ast_item::Locator),
            parent: parse_quote!(auto_lsp::ast_item::Parent),
            check_duplicate: parse_quote!(auto_lsp::ast_item::CheckDuplicate),
            ast_item_trait: parse_quote!(auto_lsp::ast_item::AstItem),
            ast_item_builder_trait: parse_quote!(auto_lsp::ast_item_builder::AstItemBuilder),
            code_lens_trait: parse_quote!(auto_lsp::ast_item::CodeLens),
            completion_items_trait: parse_quote!(auto_lsp::ast_item::CompletionItems),
            document_symbols_trait: parse_quote!(auto_lsp::ast_item::DocumentSymbols),
            hover_info_trait: parse_quote!(auto_lsp::ast_item::HoverInfo),
            inlay_hints_trait: parse_quote!(auto_lsp::ast_item::InlayHints),
            semantic_tokens_trait: parse_quote!(auto_lsp::ast_item::SemanticTokens),
            scope_trait: parse_quote!(auto_lsp::ast_item::Scope),
            is_accessor_trait: parse_quote!(auto_lsp::ast_item::IsAccessor),
            accessor_trait: parse_quote!(auto_lsp::ast_item::Accessor),
            try_into_builder: parse_quote!(auto_lsp::convert::TryIntoBuilder),
            try_from_builder: parse_quote!(auto_lsp::convert::TryFromBuilder),
        }
    }
}
