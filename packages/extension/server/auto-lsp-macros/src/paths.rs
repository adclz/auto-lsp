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
            query_binder: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
            add: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
            try_to_dyn_symbol: Structx! {
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
            set_accessor: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
            get_accessor: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
            reset_accessor: Structx! {
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
    pub scope: TraitInfo<
        Structx! {
            is_scope: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
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
    pub check: TraitInfo<
        Structx! {
            must_check: Structx! {
                sig: TokenStream,
                variant: TokenStream,
            },
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
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            // new types idioms
            symbol: parse_quote!(auto_lsp::symbol::Symbol),
            dyn_symbol: parse_quote!(auto_lsp::symbol::DynSymbol),
            weak_symbol: parse_quote!(auto_lsp::symbol::WeakSymbol),
            symbol_data: parse_quote!(auto_lsp::symbol::AstSymbolData),
            referrers: parse_quote!(auto_lsp::symbol::Referrers),
            pending_symbol: parse_quote!(auto_lsp::pending_symbol::PendingSymbol),
            maybe_pending_symbol: parse_quote!(auto_lsp::pending_symbol::MaybePendingSymbol),
            builder_params: parse_quote!(auto_lsp::builders::BuilderParams),

            // traits
            queryable: TraitInfo {
                path: parse_quote!(auto_lsp::queryable::Queryable),
                methods: structx! {
                    QUERY_NAMES: structx! {
                        sig: quote! { const QUERY_NAMES: &'static [&'static str] },
                    },
                },
            },
            check_queryable: TraitInfo {
                path: parse_quote!(auto_lsp::queryable::CheckQueryable),
                methods: structx! {
                    CHECK: structx! {
                        sig: quote! { const CHECK: () },
                    },
                },
            },
            symbol_trait: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::AstSymbol),
                methods: structx! {
                    get_data: structx! {
                        sig: quote! { fn get_data(&self) -> &auto_lsp::symbol::AstSymbolData },
                        variant: quote! { get_data() },
                    },
                    get_mut_data: structx! {
                        sig: quote! { fn get_mut_data(&mut self) -> &mut auto_lsp::symbol::AstSymbolData },
                        variant: quote! { get_mut_data() },
                    }
                },
            },
            symbol_builder_trait: TraitInfo {
                path: parse_quote!(auto_lsp::pending_symbol::AstBuilder),
                methods: structx! {
                    new: structx! {
                        sig: quote! { fn new(
                            url: std::sync::Arc<lsp_types::Url>,
                            _query: &tree_sitter::Query,
                            query_index: usize,
                            range: tree_sitter::Range,
                            start_position: tree_sitter::Point,
                            end_position: tree_sitter::Point,
                        ) -> Option<Self> },
                        variant: quote! { new(url, _query, qury_index, range, start_position, end_position) },
                    },
                    query_binder: structx! {
                        sig: quote! { fn query_binder(
                            &self,
                            url: std::sync::Arc<lsp_types::Url>,
                            capture: &tree_sitter::QueryCapture,
                            query: &tree_sitter::Query,
                        ) -> auto_lsp::pending_symbol::MaybePendingSymbol },
                        variant: quote! { get_data(url, capture, query) },
                    },
                    add: structx! {
                        sig: quote! { fn add(
                            &mut self,
                            query: &tree_sitter::Query,
                            node: auto_lsp::pending_symbol::PendingSymbol,
                            source_code: &[u8],
                            params: &mut auto_lsp::builders::BuilderParams,
                        ) -> Result<(), lsp_types::Diagnostic> },
                        variant: quote! { add(query, node, source_code, params) },
                    },
                    try_to_dyn_symbol: structx! {
                        sig: quote! { fn try_to_dyn_symbol(
                            &self,
                            check: &mut auto_lsp::builders:: BuilderParams,
                        ) -> Result<auto_lsp::symbol::DynSymbol, lsp_types::Diagnostic>
                        },
                        variant: quote! { try_to_dyn_symbol(check) }
                    },
                    get_url: structx! {
                        sig: quote! { fn get_url(&self) -> std::sync::Arc<lsp_types::Url> },
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
            try_into_builder: parse_quote!(auto_lsp::convert::TryIntoBuilder),
            try_from_builder: parse_quote!(auto_lsp::convert::TryFromBuilder),

            lsp_code_lens: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::CodeLens),
                methods: structx! {
                    build_code_lens: structx! {
                        sig: quote! { fn build_code_lens(&self, acc: &mut Vec<lsp_types::CodeLens>) },
                        variant: quote! { build_code_lens(acc) },
                    }
                },
            },
            lsp_completion_items: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::CompletionItems),
                methods: structx! {
                    build_completion_items: structx! {
                        sig: quote! { fn build_completion_items(&self, acc: &mut Vec<lsp_types::CompletionItem>, doc: &lsp_textdocument::FullTextDocument) },
                        variant: quote! { build_completion_items(acc, doc) },
                    }
                },
            },
            lsp_document_symbols: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::DocumentSymbols),
                methods: structx! {
                    get_document_symbols: structx! {
                        sig: quote! { fn get_document_symbols(&self, doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::DocumentSymbol> },
                        variant: quote! { get_document_symbols(doc) },
                    }
                },
            },
            lsp_go_to_definition: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::GoToDefinition),
                methods: structx! {
                    go_to_definition: structx! {
                        sig: quote! { fn go_to_definition(&self, doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::GotoDefinitionResponse> },
                        variant: quote! { go_to_definition(doc) },
                    }
                },
            },
            lsp_go_to_declaration: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::GoToDeclaration),
                methods: structx! {
                    go_to_declaration: structx! {
                        sig: quote! { fn go_to_declaration(&self, doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::request::GotoDeclarationResponse> },
                        variant: quote! { go_to_declaration(doc) },
                    }
                },
            },
            lsp_hover_info: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::HoverInfo),
                methods: structx! {
                    get_hover: structx! {
                        sig: quote! { fn get_hover(&self, doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::Hover> },
                        variant: quote! { get_hover(doc) },
                    }
                },
            },
            lsp_inlay_hint: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::InlayHints),
                methods: structx! {
                    build_inlay_hint: structx! {
                        sig: quote! { fn build_inlay_hint(&self, doc: &lsp_textdocument::FullTextDocument, acc: &mut Vec<lsp_types::InlayHint>) },
                        variant: quote! { build_inlay_hint(doc, acc) },
                    }
                },
            },
            lsp_semantic_token: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::SemanticTokens),
                methods: structx! {
                    build_semantic_tokens: structx! {
                        sig: quote! { fn build_semantic_tokens(&self, builder: &mut auto_lsp::semantic_tokens::SemanticTokensBuilder) },
                        variant: quote! { build_semantic_tokens(builder) },
                    }
                },
            },
            is_accessor: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::IsAccessor),
                methods: structx! {
                    is_accessor: structx! {
                        sig: quote! { fn is_accessor(&self) -> bool},
                        variant: quote! { is_accessor() },
                    },
                    set_accessor: structx! {
                        sig: quote! { fn set_accessor(&mut self, accessor: auto_lsp::symbol::WeakSymbol) },
                        variant: quote! { set_accessor(accessor) },
                    },
                    get_accessor: structx! {
                        sig: quote! { fn get_accessor(&self) -> Option<&auto_lsp::symbol::WeakSymbol>  },
                        variant: quote! { get_accessor() },
                    },
                    reset_accessor: structx! {
                        sig: quote! { fn reset_accessor(&mut self) },
                        variant: quote! { reset_accessor() },
                    },
                },
            },
            accessor: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::Accessor),
                methods: structx! {
                    find: structx! {
                        sig: quote! { fn find(&self, doc: &lsp_textdocument::FullTextDocument, ctx: &dyn auto_lsp::workspace::WorkspaceContext) -> Result<Option<auto_lsp::symbol::DynSymbol>, lsp_types::Diagnostic> },
                        variant: quote! { find(doc, ctx) },
                    },
                },
            },
            scope: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::Scope),
                methods: structx! {
                    is_scope: structx! {
                        sig: quote! { fn is_scope(&self) -> bool },
                        variant: quote! { is_scope() },
                    },
                    get_scope_range: structx! {
                        sig: quote! { fn get_scope_range(&self) -> Vec<[usize; 2]> },
                        variant: quote! { get_scope_range() },
                    },
                },
            },
            locator: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::Locator),
                methods: structx! {
                    find_at_offset: structx! {
                        sig: quote! { fn find_at_offset(&self, offset: usize) -> Option<auto_lsp::symbol::DynSymbol> },
                        variant: quote! { find_at_offset(offset) },
                    },
                },
            },
            parent: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::Parent),
                methods: structx! {
                    inject_parent: structx! {
                        sig: quote! { fn inject_parent(&mut self, parent: auto_lsp::symbol::WeakSymbol) },
                        variant: quote! { inject_parent(parent) },
                    },
                },
            },
            check: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::Check),
                methods: structx! {
                    must_check: structx! {
                        sig: quote! { fn must_check(&self) -> bool },
                        variant: quote! { must_check() },
                    },
                    check: structx! {
                        sig: quote! { fn check(&self, doc: &lsp_textdocument::FullTextDocument, diagnostics: &mut Vec<lsp_types::Diagnostic>) },
                        variant: quote! { check(doc, diagnostics) },
                    },
                },
            },
            dynamic_swap: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::DynamicSwap),
                methods: structx! {
                    swap: structx! {
                        sig: quote! { fn dyn_swap<'a>(
                            &mut self,
                            offset: usize,
                            builder_params: &'a mut auto_lsp::builders::BuilderParams,
                        ) -> Result<(), lsp_types::Diagnostic> },
                        variant: quote! { dyn_swap(offset, builder_params) },
                    },
                },
            },
            edit_range: TraitInfo {
                path: parse_quote!(auto_lsp::symbol::EditRange),
                methods: structx! {
                    edit_range: structx! {
                        sig: quote! { fn edit_range(&mut self, start: usize, offset: isize) },
                        variant: quote! { edit_range(start, offset) },
                    },
                },
            },
        }
    }
}
