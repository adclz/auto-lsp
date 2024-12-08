use proc_macro2::TokenStream;
use quote::quote;
use structx::*;
use syn::{parse_quote, Path};

pub struct TraitInfo<T> {
    pub path: Path,
    pub methods: T,
}

pub struct Paths {
    pub lsp_code_lens: TraitInfo<
        Structx! {
            build_code_lens: Structx! {
                sig: TokenStream,
                variant: TokenStream,
                default: TokenStream
            }
        },
    >,
    pub lsp_document_symbols: TraitInfo<
        Structx! {
            get_document_symbols: Structx! {
                sig: TokenStream,
                variant: TokenStream,
                default: TokenStream
            }
        },
    >,
    pub lsp_completion_items: TraitInfo<
        Structx! {
            build_completion_items: Structx! {
                sig: TokenStream,
                variant: TokenStream,
                default: TokenStream
            }
        },
    >,
    pub lsp_go_to_definition: TraitInfo<
        Structx! {
            go_to_definition: Structx! {
                sig: TokenStream,
                variant: TokenStream,
                default: TokenStream
            }
        },
    >,
    pub lsp_hover_info: TraitInfo<
        Structx! {
            get_hover: Structx! {
                sig: TokenStream,
                variant: TokenStream,
                default: TokenStream
            }
        },
    >,
    pub lsp_inlay_hint: TraitInfo<
        Structx! {
            build_inlay_hint: Structx! {
                sig: TokenStream,
                variant: TokenStream,
                default: TokenStream
            }
        },
    >,
    pub lsp_semantic_token: TraitInfo<
        Structx! {
            build_semantic_tokens: Structx! {
                sig: TokenStream,
                variant: TokenStream,
                default: TokenStream
            }
        },
    >,
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
    pub scope_trait: Path,
    pub is_accessor_trait: Path,
    pub accessor_trait: Path,
    pub try_into_builder: Path,
    pub try_from_builder: Path,
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            lsp_code_lens: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::CodeLens),
                methods: structx! {
                    build_code_lens: structx! {
                        sig: quote! { fn build_code_lens(&self, acc: &mut Vec<lsp_types::CodeLens>) },
                        variant: quote! { build_code_lens(acc) },
                        default: quote! { }
                    }
                },
            },
            lsp_completion_items: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::CompletionItems),
                methods: structx! {
                    build_completion_items: structx! {
                        sig: quote! { fn build_completion_items(&self, acc: &mut Vec<lsp_types::CompletionItem>, doc: &lsp_textdocument::FullTextDocument) },
                        variant: quote! { build_completion_items(acc, doc) },
                        default: quote! { }
                    }
                },
            },
            lsp_document_symbols: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::DocumentSymbols),
                methods: structx! {
                    get_document_symbols: structx! {
                        sig: quote! { fn get_document_symbols(&self, doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::DocumentSymbol> },
                        variant: quote! { get_document_symbols(doc) },
                        default: quote! { None }
                    }
                },
            },
            lsp_go_to_definition: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::GoToDefinition),
                methods: structx! {
                    go_to_definition: structx! {
                        sig: quote! { fn go_to_definition(&self, doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::GotoDefinitionResponse> },
                        variant: quote! { go_to_definition(doc) },
                        default: quote! { None }
                    }
                },
            },
            lsp_hover_info: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::HoverInfo),
                methods: structx! {
                    get_hover: structx! {
                        sig: quote! { fn get_hover(&self, doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::Hover> },
                        variant: quote! { get_hover(doc) },
                        default: quote! { None }
                    }
                },
            },
            lsp_inlay_hint: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::InlayHints),
                methods: structx! {
                    build_inlay_hint: structx! {
                        sig: quote! { fn build_inlay_hint(&self, doc: &lsp_textdocument::FullTextDocument, acc: &mut Vec<lsp_types::InlayHint>) },
                        variant: quote! { build_inlay_hint(doc, acc) },
                        default: quote! { }
                    }
                },
            },
            lsp_semantic_token: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::SemanticTokens),
                methods: structx! {
                    build_semantic_tokens: structx! {
                        sig: quote! { fn build_semantic_tokens(&self, builder: &mut auto_lsp::semantic_tokens::SemanticTokensBuilder) },
                        variant: quote! { build_semantic_tokens(builder) },
                        default: quote! { }
                    }
                },
            },
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
            scope_trait: parse_quote!(auto_lsp::symbol::Scope),
            is_accessor_trait: parse_quote!(auto_lsp::symbol::IsAccessor),
            accessor_trait: parse_quote!(auto_lsp::symbol::Accessor),
            try_into_builder: parse_quote!(auto_lsp::convert::TryIntoBuilder),
            try_from_builder: parse_quote!(auto_lsp::convert::TryFromBuilder),
        }
    }
}
