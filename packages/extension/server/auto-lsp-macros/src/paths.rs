use proc_macro::TokenStream;
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

    pub symbol_trait: Path,
    pub symbol_builder_trait: Path,
    pub code_lens_trait: Path,
    pub completion_items_trait: Path,
    pub document_symbols_trait: Path,
    pub hover_info_trait: Path,
    pub inlay_hints_trait: Path,
    pub semantic_tokens_trait: Path,
    pub go_to_definition_trait: Path,
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
            symbol: parse_quote!(auto_lsp::symbol::Symbol),
            dyn_symbol: parse_quote!(auto_lsp::symbol::DynSymbol),
            weak_symbol: parse_quote!(auto_lsp::symbol::WeakSymbol),
            pending_symbol: parse_quote!(auto_lsp::pending_symbol::PendingSymbol),
            maybe_pending_symbol: parse_quote!(auto_lsp::pending_symbol::MaybePendingSymbol),

            // traits
            queryable: parse_quote!(auto_lsp::queryable::Queryable),
            locator: parse_quote!(auto_lsp::symbol::Locator),
            parent: parse_quote!(auto_lsp::symbol::Parent),
            check_duplicate: parse_quote!(auto_lsp::symbol::CheckDuplicate),
            symbol_trait: parse_quote!(auto_lsp::symbol::AstSymbol),
            symbol_builder_trait: parse_quote!(auto_lsp::pending_symbol::AstBuilder),
            code_lens_trait: parse_quote!(auto_lsp::symbol::CodeLens),
            completion_items_trait: parse_quote!(auto_lsp::symbol::CompletionItems),
            document_symbols_trait: parse_quote!(auto_lsp::symbol::DocumentSymbols),
            hover_info_trait: parse_quote!(auto_lsp::symbol::HoverInfo),
            inlay_hints_trait: parse_quote!(auto_lsp::symbol::InlayHints),
            semantic_tokens_trait: parse_quote!(auto_lsp::symbol::SemanticTokens),
            go_to_definition_trait: parse_quote!(auto_lsp::symbol::GoToDefinition),
            scope_trait: parse_quote!(auto_lsp::symbol::Scope),
            is_accessor_trait: parse_quote!(auto_lsp::symbol::IsAccessor),
            accessor_trait: parse_quote!(auto_lsp::symbol::Accessor),
            try_into_builder: parse_quote!(auto_lsp::convert::TryIntoBuilder),
            try_from_builder: parse_quote!(auto_lsp::convert::TryFromBuilder),
        }
    }
}
