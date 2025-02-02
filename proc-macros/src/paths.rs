#![allow(unused)]
#![allow(non_snake_case)]
use std::sync::LazyLock;

use nested_struct::nested_struct;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Path};

/// path to core_ast module
pub fn core_ast(path: Path) -> Path {
    parse_quote!(auto_lsp::core::ast::#path)
}

/// path to core_build module
pub fn core_build(path: Path) -> Path {
    parse_quote!(auto_lsp::core::build::#path)
}

pub struct Method {
    pub sig: TokenStream,
    pub variant: TokenStream,
}

nested_struct!(
    pub struct Paths {
        pub symbol: Path,
        pub dyn_symbol: Path,
        pub weak_symbol: Path,
        pub referrers: Path,
        pub symbol_data: Path,
        pub vec_or_symbol: Path,
        pub pending_symbol: Path,
        pub maybe_pending_symbol: Path,
        pub workspace: Path,

        pub queryable: Queryable {
            pub path: Path,
            pub QUERY_NAMES: Method
        },

        pub symbol_trait: SymbolTrait {
            pub path: Path,
            pub get_data: Method,
            pub get_mut_data: Method,
        },

        pub symbol_builder_trait: SymbolBuilderTrait {
            pub path: Path,
            pub new: Method,
            pub add: Method,
            pub get_url: Method,
            pub get_range: Method,
            pub get_query_index: Method,
        },

        pub try_into_builder: Path,
        pub try_from_builder: Path,

        pub add_symbol_trait: Path,
        pub try_downcast_trait: Path,
        pub finalize_trait: Path,

        pub lsp_code_lens: LspCodeLens {
            pub path: Path,
            pub build_code_lens: Method
        },
        pub lsp_document_symbols: LspDocumentSymbols {
            pub path: Path,
            pub build_document_symbols: Method
        },
        pub lsp_completion_items: LspCompletionItems {
            pub path: Path,
            pub build_completion_items: Method
        },
        pub lsp_invoked_completion_items: LspInvokedCompletionItems {
            pub path: Path,
            pub build_invoked_completion_items: Method
        },
        pub lsp_go_to_definition: LspGoToDefinition {
            pub path: Path,
            pub go_to_definition: Method
        },
        pub lsp_go_to_declaration: LspGoToDeclaration {
            pub path: Path,
            pub go_to_declaration: Method
        },
        pub lsp_hover_info: LspHoverInfo {
            pub path: Path,
            pub get_hover: Method
        },
        pub lsp_inlay_hint: LspInlayHint {
            pub path: Path,
            pub build_inlay_hints: Method
        },
        pub lsp_semantic_token: LspSemanticToken {
            pub path: Path,
            pub build_semantic_tokens: Method
        },
        pub is_reference: IsReference {
            pub path: Path,
            pub is_reference: Method
        },
        pub reference: Reference {
            pub path: Path,
            pub find: Method
        },
        pub is_comment: Comment {
            pub path: Path,
            pub is_comment: Method
        },
        pub is_scope: IsScope {
            pub path: Path,
            pub is_scope: Method
        },
        pub scope: Scope {
            pub path: Path,
            pub get_scope_range: Method
        },
        pub locator: Locator {
            pub path: Path,
            pub find_at_offset: Method
        },
        pub parent: Parent {
            pub path: Path,
            pub inject_parent: Method
        },
        pub is_check: IsCheck {
            pub path: Path,
            pub must_check: Method
        },
        pub check: Check {
            pub path: Path,
            pub check: Method
        },
        pub dynamic_swap: DynamicSwap {
            pub path: Path,
            pub swap: Method
        },
        pub static_swap: StaticSwap {
            pub path: Path,
            pub swap: Method
        },
        pub edit_range: EditRange {
            pub path: Path,
            pub edit_range: Method
        },
        pub collect_references: CollectReferences {
            pub path: Path,
            pub collect_references: Method
        },
    }
);

impl Default for Paths {
    fn default() -> Self {
        Self {
            symbol: core_ast(parse_quote!(Symbol)),
            dyn_symbol: core_ast(parse_quote!(DynSymbol)),
            weak_symbol: core_ast(parse_quote!(WeakSymbol)),
            symbol_data: core_ast(parse_quote!(SymbolData)),
            vec_or_symbol: core_ast(parse_quote!(VecOrSymbol)),
            referrers: core_ast(parse_quote!(Referrers)),
            pending_symbol: core_build(parse_quote!(PendingSymbol)),
            maybe_pending_symbol: core_build(parse_quote!(MaybePendingSymbol)),
            workspace: parse_quote!(auto_lsp::core::workspace::Workspace),

            queryable: Queryable {
                path: core_build(parse_quote!(Queryable)),
                QUERY_NAMES: Method {
                    sig: quote! { const QUERY_NAMES: &'static [&'static str] },
                    variant: quote! { QUERY_NAMES },
                },
            },

            symbol_trait: SymbolTrait {
                path: core_ast(parse_quote!(AstSymbol)),
                get_data: Method {
                    sig: quote! { fn get_data(&self) -> &auto_lsp::core::ast::SymbolData },
                    variant: quote! { get_data() },
                },
                get_mut_data: Method {
                    sig: quote! { fn get_mut_data(&mut self) -> &mut auto_lsp::core::ast::SymbolData },
                    variant: quote! { get_mut_data() },
                },
            },

            symbol_builder_trait: SymbolBuilderTrait {
                path: core_build(parse_quote!(Buildable)),
                new: Method {
                    sig: quote! { fn new(
                        url: std::sync::Arc<auto_lsp::lsp_types::Url>,
                        query: &auto_lsp::tree_sitter::Query,
                        capture: &auto_lsp::tree_sitter::QueryCapture,
                    ) -> Option<Self> },
                    variant: quote! { new(url, query, capture) },
                },
                add: Method {
                    sig: quote! { fn add(
                        &mut self,
                        capture: &auto_lsp::tree_sitter::QueryCapture,
                        workspace: &mut auto_lsp::core::workspace::Workspace,
                        document: &auto_lsp::core::document::Document,
                    ) -> Result<Option<auto_lsp::core::build::PendingSymbol>, auto_lsp::lsp_types::Diagnostic> },
                    variant: quote! { add(capture, workspace, document) },
                },
                get_url: Method {
                    sig: quote! { fn get_url(&self) -> std::sync::Arc<auto_lsp::lsp_types::Url> },
                    variant: quote! { get_url() },
                },
                get_range: Method {
                    sig: quote! { fn get_range(&self) -> std::ops::Range<usize> },
                    variant: quote! { get_range() },
                },
                get_query_index: Method {
                    sig: quote! { fn get_query_index(&self) -> usize },
                    variant: quote! { get_query_index() },
                },
            },
            try_from_builder: core_build(parse_quote!(TryFromBuilder)),
            try_into_builder: core_build(parse_quote!(TryIntoBuilder)),
            add_symbol_trait: core_build(parse_quote!(AddSymbol)),
            try_downcast_trait: core_build(parse_quote!(TryDownCast)),
            finalize_trait: core_build(parse_quote!(Finalize)),

            lsp_document_symbols: LspDocumentSymbols {
                path: core_ast(parse_quote!(BuildDocumentSymbols)),
                build_document_symbols: Method {
                    sig: quote! { fn build_document_symbols(&self, doc: &auto_lsp::core::document::Document, builder: &mut auto_lsp::core::document_symbols_builder::DocumentSymbolsBuilder) },
                    variant: quote! { build_document_symbols(doc, builder) },
                },
            },
            lsp_code_lens: LspCodeLens {
                path: core_ast(parse_quote!(BuildCodeLens)),
                build_code_lens: Method {
                    sig: quote! { fn build_code_lens(&self, doc: &auto_lsp::core::document::Document, acc: &mut Vec<auto_lsp::lsp_types::CodeLens>) },
                    variant: quote! { build_code_lens(doc, acc) },
                },
            },
            lsp_completion_items: LspCompletionItems {
                path: core_ast(parse_quote!(BuildCompletionItems)),
                build_completion_items: Method {
                    sig: quote! { fn build_completion_items(&self, doc: &auto_lsp::core::document::Document, acc: &mut Vec<auto_lsp::lsp_types::CompletionItem>)},
                    variant: quote! { build_completion_items(doc, acc) },
                },
            },
            lsp_invoked_completion_items: LspInvokedCompletionItems {
                path: core_ast(parse_quote!(BuildInvokedCompletionItems)),
                build_invoked_completion_items: Method {
                    sig: quote! { fn build_invoked_completion_items(&self, trigger: &str, doc: &auto_lsp::core::document::Document, acc: &mut Vec<auto_lsp::lsp_types::CompletionItem>) },
                    variant: quote! { build_invoked_completion_items(trigger, doc, acc) },
                },
            },
            lsp_go_to_definition: LspGoToDefinition {
                path: core_ast(parse_quote!(GetGoToDefinition)),
                go_to_definition: Method {
                    sig: quote! { fn go_to_definition(&self, doc: &auto_lsp::core::document::Document) -> Option<auto_lsp::lsp_types::GotoDefinitionResponse> },
                    variant: quote! { go_to_definition(doc) },
                },
            },
            lsp_go_to_declaration: LspGoToDeclaration {
                path: core_ast(parse_quote!(GetGoToDeclaration)),
                go_to_declaration: Method {
                    sig: quote! { fn go_to_declaration(&self, doc: &auto_lsp::core::document::Document) -> Option<auto_lsp::lsp_types::request::GotoDeclarationResponse> },
                    variant: quote! { go_to_declaration(doc) },
                },
            },
            lsp_hover_info: LspHoverInfo {
                path: core_ast(parse_quote!(GetHover)),
                get_hover: Method {
                    sig: quote! { fn get_hover(&self, doc: &auto_lsp::core::document::Document) -> Option<auto_lsp::lsp_types::Hover> },
                    variant: quote! { get_hover(doc) },
                },
            },
            lsp_inlay_hint: LspInlayHint {
                path: core_ast(parse_quote!(BuildInlayHints)),
                build_inlay_hints: Method {
                    sig: quote! { fn build_inlay_hints(&self, doc: &auto_lsp::core::document::Document, acc: &mut Vec<auto_lsp::lsp_types::InlayHint>) },
                    variant: quote! { build_inlay_hints(doc, acc) },
                },
            },
            lsp_semantic_token: LspSemanticToken {
                path: core_ast(parse_quote!(BuildSemanticTokens)),
                build_semantic_tokens: Method {
                    sig: quote! { fn build_semantic_tokens(&self, doc: &auto_lsp::core::document::Document, builder: &mut auto_lsp::core::semantic_tokens_builder::SemanticTokensBuilder) },
                    variant: quote! { build_semantic_tokens(doc, builder) },
                },
            },
            is_reference: IsReference {
                path: core_ast(parse_quote!(IsReference)),
                is_reference: Method {
                    sig: quote! { fn is_reference(&self) -> bool},
                    variant: quote! { is_reference() },
                },
            },
            reference: Reference {
                path: core_ast(parse_quote!(Reference)),
                find: Method {
                    sig: quote! { fn find(&self, doc: &auto_lsp::core::document::Document) -> Result<Option<auto_lsp::core::ast::DynSymbol>, auto_lsp::lsp_types::Diagnostic> },
                    variant: quote! { find(doc) },
                },
            },
            is_comment: Comment {
                path: core_ast(parse_quote!(Comment)),
                is_comment: Method {
                    sig: quote! { fn is_comment(&self) -> bool },
                    variant: quote! { is_comment() },
                },
            },
            is_scope: IsScope {
                path: core_ast(parse_quote!(IsScope)),
                is_scope: Method {
                    sig: quote! { fn is_scope(&self) -> bool },
                    variant: quote! { is_scope() },
                },
            },
            scope: Scope {
                path: core_ast(parse_quote!(Scope)),
                get_scope_range: Method {
                    sig: quote! { fn get_scope_range(&self) -> Vec<[usize; 2]> },
                    variant: quote! { get_scope_range() },
                },
            },
            locator: Locator {
                path: core_ast(parse_quote!(Locator)),
                find_at_offset: Method {
                    sig: quote! { fn find_at_offset(&self, offset: usize) -> Option<auto_lsp::core::ast::DynSymbol> },
                    variant: quote! { find_at_offset(offset) },
                },
            },
            parent: Parent {
                path: core_ast(parse_quote!(Parent)),
                inject_parent: Method {
                    sig: quote! { fn inject_parent(&mut self, parent: auto_lsp::core::ast::WeakSymbol) },
                    variant: quote! { inject_parent(parent) },
                },
            },
            is_check: IsCheck {
                path: core_ast(parse_quote!(IsCheck)),
                must_check: Method {
                    sig: quote! { fn must_check(&self) -> bool },
                    variant: quote! { must_check() },
                },
            },
            check: Check {
                path: core_ast(parse_quote!(Check)),
                check: Method {
                    sig: quote! { fn check(&self, doc: &auto_lsp::core::document::Document, diagnostics: &mut Vec<auto_lsp::lsp_types::Diagnostic>) -> Result<(), ()> },
                    variant: quote! { check(doc, diagnostics) },
                },
            },
            dynamic_swap: DynamicSwap {
                path: core_ast(parse_quote!(UpdateDynamic)),
                swap: Method {
                    sig: quote! { fn update(
                        &mut self,
                        range: &std::ops::Range<usize>,
                        parent_check: Option<auto_lsp::core::ast::WeakSymbol>,
                        workspace: &mut auto_lsp::core::workspace::Workspace,
                        document: &auto_lsp::core::document::Document,
                    ) -> std::ops::ControlFlow<Result<(), auto_lsp::lsp_types::Diagnostic>, ()> },
                    variant: quote! { update(&range, parent_check, workspace, document) },
                },
            },
            static_swap: StaticSwap {
                path: core_ast(parse_quote!(UpdateStatic)),
                swap: Method {
                    sig: quote! { fn update(
                        &mut self,
                        range: &std::ops::Range<usize>,
                        parent_check: Option<auto_lsp::core::ast::WeakSymbol>,
                        workspace: &mut auto_lsp::core::workspace::Workspace,
                        document: &auto_lsp::core::document::Document,
                    ) -> std::ops::ControlFlow<Result<(), auto_lsp::lsp_types::Diagnostic>, ()> },
                    variant: quote! { update(&range, parent_check, workspace, document) },
                },
            },
            edit_range: EditRange {
                path: core_ast(parse_quote!(UpdateRange)),
                edit_range: Method {
                    sig: quote! { fn edit_range(&self, start: usize, offset: isize) },
                    variant: quote! { edit_range(start, offset) },
                },
            },
            collect_references: CollectReferences {
                path: core_ast(parse_quote!(CollectReferences)),
                collect_references: Method {
                    sig: quote! { fn collect_references(&self, workspace: &mut auto_lsp::core::workspace::Workspace) },
                    variant: quote! { collect_references(workspace) },
                },
            },
        }
    }
}
