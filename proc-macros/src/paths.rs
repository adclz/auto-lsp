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
    pub referrers: Path,
    pub symbol_data: Path,
    pub pending_symbol: Path,
    pub maybe_pending_symbol: Path,
    pub builder_params: Path,

    // traits
    pub queryable: TraitInfo<
        Structx! {
                QUERY_NAMES: Structx! {
                    sig: TokenStream,
                },
        },
    >,
    pub check_queryable: TraitInfo<
        Structx! {
                CHECK: Structx! {
                    sig: TokenStream,
                },
        },
    >,
    pub symbol_trait: TraitInfo<
        Structx! {
            get_data: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
            get_mut_data: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
        },
    >,
    pub symbol_builder_trait: TraitInfo<
        Structx! {
            new: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
            add: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
            get_url: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
            get_range: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
            get_query_index: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
        },
    >,
    pub try_into_builder: Path,
    pub try_from_builder: Path,

    pub lsp_code_lens: TraitInfo<
        Structx! {
            build_code_lens: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            }
        },
    >,
    pub lsp_document_symbols: TraitInfo<
        Structx! {
            get_document_symbols: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            }
        },
    >,
    pub lsp_completion_items: TraitInfo<
        Structx! {
            build_completion_items: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            }
        },
    >,
    pub lsp_go_to_definition: TraitInfo<
        Structx! {
            go_to_definition: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            }
        },
    >,
    pub lsp_go_to_declaration: TraitInfo<
        Structx! {
            go_to_declaration: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            }
        },
    >,
    pub lsp_hover_info: TraitInfo<
        Structx! {
            get_hover: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            }
        },
    >,
    pub lsp_inlay_hint: TraitInfo<
        Structx! {
            build_inlay_hint: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            }
        },
    >,
    pub lsp_semantic_token: TraitInfo<
        Structx! {
            build_semantic_tokens: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            }
        },
    >,
    pub is_accessor: TraitInfo<
        Structx! {
            is_accessor: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
        },
    >,
    pub accessor: TraitInfo<
        Structx! {
            find: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
        },
    >,
    pub is_comment: TraitInfo<
        Structx! {
            is_comment: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
        },
    >,
    pub is_scope: TraitInfo<
        Structx! {
            is_scope: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            }
        },
    >,
    pub scope: TraitInfo<
        Structx! {
            get_scope_range: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
        },
    >,
    pub locator: TraitInfo<
        Structx! {
            find_at_offset: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
        },
    >,
    pub parent: TraitInfo<
        Structx! {
            inject_parent: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
        },
    >,
    pub is_check: TraitInfo<
        Structx! {
            must_check: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            }
        },
    >,
    pub check: TraitInfo<
        Structx! {
        check: Structx! {
            sig: TokenStream,
            variant: TokenStream,
        },
        },
    >,
    pub dynamic_swap: TraitInfo<
        Structx! {
            swap: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
        },
    >,
    pub edit_range: TraitInfo<
        Structx! {
            edit_range: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
        },
    >,
    pub collect_references: TraitInfo<
        Structx! {
            collect_references: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
        },
    >,
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            // new types idioms
            symbol: parse_quote!(auto_lsp::auto_lsp_core::symbol::Symbol),
            dyn_symbol: parse_quote!(auto_lsp::auto_lsp_core::symbol::DynSymbol),
            weak_symbol: parse_quote!(auto_lsp::auto_lsp_core::symbol::WeakSymbol),
            symbol_data: parse_quote!(auto_lsp::auto_lsp_core::symbol::AstSymbolData),
            referrers: parse_quote!(auto_lsp::auto_lsp_core::symbol::Referrers),
            pending_symbol: parse_quote!(auto_lsp::auto_lsp_core::pending_symbol::PendingSymbol),
            maybe_pending_symbol: parse_quote!(
                auto_lsp::auto_lsp_core::pending_symbol::MaybePendingSymbol
            ),
            builder_params: parse_quote!(auto_lsp::auto_lsp_core::builders::BuilderParams),

            // traits
            queryable: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::queryable::Queryable),
                methods: structx! {
                    QUERY_NAMES: structx! {
                        sig: quote! { const QUERY_NAMES: &'static [&'static str] },
                    },
                },
            },
            check_queryable: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::queryable::CheckQueryable),
                methods: structx! {
                    CHECK: structx! {
                        sig: quote! { const CHECK: () },
                    },
                },
            },
            symbol_trait: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::symbol::AstSymbol),
                methods: structx! {
                    get_data: structx! {
                        sig: quote! { fn get_data(&self) -> &auto_lsp::auto_lsp_core::symbol::AstSymbolData },
                        variant: quote! { get_data() },
                    },
                    get_mut_data: structx! {
                        sig: quote! { fn get_mut_data(&mut self) -> &mut auto_lsp::auto_lsp_core::symbol::AstSymbolData },
                        variant: quote! { get_mut_data() },
                    }
                },
            },
            symbol_builder_trait: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::pending_symbol::AstBuilder),
                methods: structx! {
                    new: structx! {
                        sig: quote! { fn new(
                            url: std::sync::Arc<auto_lsp::lsp_types::Url>,
                            query: &auto_lsp::tree_sitter::Query,
                            capture: &auto_lsp::tree_sitter::QueryCapture,
                        ) -> Option<Self> },
                        variant: quote! { new(url, query, capture) },
                    },
                    add: structx! {
                        sig: quote! { fn add(
                            &mut self,
                            capture: &auto_lsp::tree_sitter::QueryCapture,
                            params: &mut auto_lsp::auto_lsp_core::builders::BuilderParams,
                        ) -> Result<Option<auto_lsp::auto_lsp_core::pending_symbol::PendingSymbol>, auto_lsp::lsp_types::Diagnostic> },
                        variant: quote! { add(capture, params) },
                    },
                    get_url: structx! {
                        sig: quote! { fn get_url(&self) -> std::sync::Arc<auto_lsp::lsp_types::Url> },
                        variant: quote! { get_url() },
                    },
                    get_range: structx! {
                        sig: quote! { fn get_range(&self) -> std::ops::Range<usize> },
                        variant: quote! { get_range() },
                    },
                    get_query_index: structx! {
                        sig: quote! { fn get_query_index(&self) -> usize },
                        variant: quote! { get_query_index() },
                    },
                },
            },
            try_into_builder: parse_quote!(auto_lsp::auto_lsp_core::convert::TryIntoBuilder),
            try_from_builder: parse_quote!(auto_lsp::auto_lsp_core::convert::TryFromBuilder),

            lsp_code_lens: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::symbol::CodeLens),
                methods: structx! {
                    build_code_lens: structx! {
                        sig: quote! { fn build_code_lens(&self, acc: &mut Vec<auto_lsp::lsp_types::CodeLens>) },
                        variant: quote! { build_code_lens(acc) },
                    }
                },
            },
            lsp_completion_items: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::symbol::CompletionItems),
                methods: structx! {
                    build_completion_items: structx! {
                        sig: quote! { fn build_completion_items(&self, acc: &mut Vec<auto_lsp::lsp_types::CompletionItem>, doc: &auto_lsp::lsp_textdocument::FullTextDocument) },
                        variant: quote! { build_completion_items(acc, doc) },
                    }
                },
            },
            lsp_document_symbols: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::symbol::DocumentSymbols),
                methods: structx! {
                    get_document_symbols: structx! {
                        sig: quote! { fn get_document_symbols(&self, doc: &auto_lsp::lsp_textdocument::FullTextDocument) -> Option<auto_lsp::lsp_types::DocumentSymbol> },
                        variant: quote! { get_document_symbols(doc) },
                    }
                },
            },
            lsp_go_to_definition: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::symbol::GoToDefinition),
                methods: structx! {
                    go_to_definition: structx! {
                        sig: quote! { fn go_to_definition(&self, doc: &auto_lsp::lsp_textdocument::FullTextDocument) -> Option<auto_lsp::lsp_types::GotoDefinitionResponse> },
                        variant: quote! { go_to_definition(doc) },
                    }
                },
            },
            lsp_go_to_declaration: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::symbol::GoToDeclaration),
                methods: structx! {
                    go_to_declaration: structx! {
                        sig: quote! { fn go_to_declaration(&self, doc: &auto_lsp::lsp_textdocument::FullTextDocument) -> Option<auto_lsp::lsp_types::request::GotoDeclarationResponse> },
                        variant: quote! { go_to_declaration(doc) },
                    }
                },
            },
            lsp_hover_info: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::symbol::HoverInfo),
                methods: structx! {
                    get_hover: structx! {
                        sig: quote! { fn get_hover(&self, doc: &auto_lsp::lsp_textdocument::FullTextDocument) -> Option<auto_lsp::lsp_types::Hover> },
                        variant: quote! { get_hover(doc) },
                    }
                },
            },
            lsp_inlay_hint: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::symbol::InlayHints),
                methods: structx! {
                    build_inlay_hint: structx! {
                        sig: quote! { fn build_inlay_hint(&self, doc: &auto_lsp::lsp_textdocument::FullTextDocument, acc: &mut Vec<auto_lsp::lsp_types::InlayHint>) },
                        variant: quote! { build_inlay_hint(doc, acc) },
                    }
                },
            },
            lsp_semantic_token: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::symbol::SemanticTokens),
                methods: structx! {
                    build_semantic_tokens: structx! {
                        sig: quote! { fn build_semantic_tokens(&self, builder: &mut auto_lsp::auto_lsp_core::semantic_tokens::SemanticTokensBuilder) },
                        variant: quote! { build_semantic_tokens(builder) },
                    }
                },
            },
            is_accessor: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::symbol::IsAccessor),
                methods: structx! {
                    is_accessor: structx! {
                        sig: quote! { fn is_accessor(&self) -> bool},
                        variant: quote! { is_accessor() },
                    },
                },
            },
            accessor: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::symbol::Accessor),
                methods: structx! {
                    find: structx! {
                        sig: quote! { fn find(&self, doc: &auto_lsp::lsp_textdocument::FullTextDocument) -> Result<Option<auto_lsp::auto_lsp_core::symbol::DynSymbol>, auto_lsp::lsp_types::Diagnostic> },
                        variant: quote! { find(doc) },
                    },
                },
            },
            is_comment: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::symbol::IsComment),
                methods: structx! {
                    is_comment: structx! {
                        sig: quote! { fn is_comment(&self) -> bool },
                        variant: quote! { is_comment() },
                    },
                },
            },
            is_scope: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::symbol::IsScope),
                methods: structx! {
                    is_scope: structx! {
                        sig: quote! { fn is_scope(&self) -> bool },
                        variant: quote! { is_scope() },
                    },
                },
            },
            scope: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::symbol::Scope),
                methods: structx! {
                    get_scope_range: structx! {
                        sig: quote! { fn get_scope_range(&self) -> Vec<[usize; 2]> },
                        variant: quote! { get_scope_range() },
                    },
                },
            },
            locator: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::symbol::Locator),
                methods: structx! {
                    find_at_offset: structx! {
                        sig: quote! { fn find_at_offset(&self, offset: usize) -> Option<auto_lsp::auto_lsp_core::symbol::DynSymbol> },
                        variant: quote! { find_at_offset(offset) },
                    },
                },
            },
            parent: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::symbol::Parent),
                methods: structx! {
                    inject_parent: structx! {
                        sig: quote! { fn inject_parent(&mut self, parent: auto_lsp::auto_lsp_core::symbol::WeakSymbol) },
                        variant: quote! { inject_parent(parent) },
                    },
                },
            },
            is_check: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::symbol::IsCheck),
                methods: structx! {
                    must_check: structx! {
                        sig: quote! { fn must_check(&self) -> bool },
                        variant: quote! { must_check() },
                    },
                },
            },
            check: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::symbol::Check),
                methods: structx! {
                    check: structx! {
                        sig: quote! { fn check(&self, doc: &auto_lsp::lsp_textdocument::FullTextDocument, diagnostics: &mut Vec<auto_lsp::lsp_types::Diagnostic>) -> Result<(), ()> },
                        variant: quote! { check(doc, diagnostics) },
                    },
                },
            },
            dynamic_swap: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::symbol::DynamicSwap),
                methods: structx! {
                    swap: structx! {
                        sig: quote! { fn dyn_swap<'a>(
                            &mut self,
                            start: usize,
                            offset: isize,
                            builder_params: &'a mut auto_lsp::auto_lsp_core::builders::BuilderParams,
                        ) -> std::ops::ControlFlow<Result<usize, auto_lsp::lsp_types::Diagnostic>, ()> },
                        variant: quote! { dyn_swap(start, offset, builder_params) },
                    },
                },
            },
            edit_range: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::symbol::EditRange),
                methods: structx! {
                    edit_range: structx! {
                        sig: quote! { fn edit_range(&self, start: usize, offset: isize) },
                        variant: quote! { edit_range(start, offset) },
                    },
                },
            },
            collect_references: TraitInfo {
                path: parse_quote!(auto_lsp::auto_lsp_core::symbol::CollectReferences),
                methods: structx! {
                    collect_references: structx! {
                        sig: quote! { fn collect_references(&self, builder_params: & mut auto_lsp::auto_lsp_core::builders::BuilderParams) },
                        variant: quote! { collect_references(builder_params) },
                    },
                },
            },
        }
    }
}
