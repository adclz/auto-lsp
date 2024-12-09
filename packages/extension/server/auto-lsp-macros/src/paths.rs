use proc_macro2::TokenStream;
use quote::quote;
use structx::*;
use syn::{parse_quote, Path};

pub struct TraitInfo<T> {
    pub path: Path,
    pub methods: T,
}

pub struct Paths {
    // new types idioms
    pub symbol: Path,
    pub dyn_symbol: Path,
    pub weak_symbol: Path,
    pub pending_symbol: Path,
    pub maybe_pending_symbol: Path,

    // traits
    pub queryable: Path,
    pub symbol_trait: Path,
    pub symbol_builder_trait: Path,
    pub try_into_builder: Path,
    pub try_from_builder: Path,

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
    pub is_accessor: TraitInfo<
        Structx! {
            is_accessor: Structx! {
                sig: TokenStream,
                variant: TokenStream,
                default: TokenStream
            },
            set_accessor: Structx! {
                sig: TokenStream,
                variant: TokenStream,
                default: TokenStream
            },
        },
    >,
    pub accessor: TraitInfo<
        Structx! {
            find: Structx! {
                sig: TokenStream,
                variant: TokenStream,
                default: TokenStream
            },
        },
    >,
    pub scope: TraitInfo<
        Structx! {
            is_scope: Structx! {
                sig: TokenStream,
                variant: TokenStream,
                default: TokenStream
            },
            get_scope_range: Structx! {
                sig: TokenStream,
                variant: TokenStream,
                default: TokenStream
            },
        },
    >,
    pub locator: TraitInfo<
        Structx! {
            find_at_offset: Structx! {
                sig: TokenStream,
                variant: TokenStream,
                default: TokenStream
            },
        },
    >,
    pub parent: TraitInfo<
        Structx! {
            inject_parent: Structx! {
                sig: TokenStream,
                variant: TokenStream,
                default: TokenStream
            },
        },
    >,
    pub check_duplicate: TraitInfo<
        Structx! {
            must_check: Structx! {
                sig: TokenStream,
                variant: TokenStream,
                default: TokenStream
            },
            check: Structx! {
                sig: TokenStream,
                variant: TokenStream,
                default: TokenStream
            },
        },
    >,
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            // new types idioms
            symbol: parse_quote!(auto_lsp::symbol::Symbol),
            dyn_symbol: parse_quote!(auto_lsp::symbol::DynSymbol),
            weak_symbol: parse_quote!(auto_lsp::symbol::WeakSymbol),
            pending_symbol: parse_quote!(auto_lsp::pending_symbol::PendingSymbol),
            maybe_pending_symbol: parse_quote!(auto_lsp::pending_symbol::MaybePendingSymbol),

            // traits
            queryable: parse_quote!(auto_lsp::queryable::Queryable),
            symbol_trait: parse_quote!(auto_lsp::symbol::AstSymbol),
            symbol_builder_trait: parse_quote!(auto_lsp::pending_symbol::AstBuilder),
            try_into_builder: parse_quote!(auto_lsp::convert::TryIntoBuilder),
            try_from_builder: parse_quote!(auto_lsp::convert::TryFromBuilder),

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
            is_accessor: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::IsAccessor),
                methods: structx! {
                    is_accessor: structx! {
                        sig: quote! { fn is_accessor(&self) -> bool},
                        variant: quote! { is_accessor() },
                        default: quote! { false }
                    },
                    set_accessor: structx! {
                        sig: quote! { fn set_accessor(&mut self, accessor: auto_lsp::symbol::WeakSymbol) },
                        variant: quote! { set_accessor(accessor) },
                        default: quote! { }
                    },
                },
            },
            accessor: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::Accessor),
                methods: structx! {
                    find: structx! {
                        sig: quote! { fn find(&self, doc: &lsp_textdocument::FullTextDocument, ctx: &dyn auto_lsp::workspace::WorkspaceContext) -> Result<Option<auto_lsp::symbol::WeakSymbol>, lsp_types::Diagnostic> },
                        variant: quote! { find(doc, ctx) },
                        default: quote! { Ok(None) }
                    },
                },
            },
            scope: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::Scope),
                methods: structx! {
                    is_scope: structx! {
                        sig: quote! { fn is_scope(&self) -> bool },
                        variant: quote! { is_scope() },
                        default: quote! { false }
                    },
                    get_scope_range: structx! {
                        sig: quote! { fn get_scope_range(&self) -> Vec<[usize; 2]> },
                        variant: quote! { get_scope_range() },
                        default: quote! { Vec::new() }
                    },
                },
            },
            locator: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::Locator),
                methods: structx! {
                    find_at_offset: structx! {
                        sig: quote! { fn find_at_offset(&self, offset: usize) -> Option<auto_lsp::symbol::DynSymbol> },
                        variant: quote! { find_at_offset(offset) },
                        default: quote! { None }
                    },
                },
            },
            parent: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::Parent),
                methods: structx! {
                    inject_parent: structx! {
                        sig: quote! { fn inject_parent(&mut self, parent: auto_lsp::symbol::WeakSymbol) },
                        variant: quote! { inject_parent(parent) },
                        default: quote! { }
                    },
                },
            },
            check_duplicate: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::CheckDuplicate),
                methods: structx! {
                    must_check: structx! {
                        sig: quote! { fn must_check(&self) -> bool },
                        variant: quote! { must_check() },
                        default: quote! { false }
                    },
                    check: structx! {
                        sig: quote! { fn check(&self, doc: &lsp_textdocument::FullTextDocument, diagnostics: &mut Vec<lsp_types::Diagnostic>) },
                        variant: quote! { check(doc, diagnostics) },
                        default: quote! { }
                    },
                },
            },
        }
    }
}
