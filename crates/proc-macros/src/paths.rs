/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/

#![allow(non_snake_case)]

use nested_struct::nested_struct;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Path};

// All paths from core crate
// Eventually, this will be generated by a proc-macro or a build script

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
        pub symbol_data: Path,
        pub pending_symbol: Path,
        pub maybe_pending_symbol: Path,
        pub parsers: Path,

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
            pub get_range: Method,
            pub get_query_index: Method,
        },

        pub add_symbol_trait: Path,
        pub try_downcast_trait: Path,
        pub finalize_trait: Path,

        pub lsp_code_lens: LspCodeLens {
            pub path: Path,
            pub build_code_lenses: Method
        },
        pub lsp_code_actions: LspCodeActions {
            pub path: Path,
            pub build_code_actions: Method
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
            pub build_triggered_completion_items: Method
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
        pub scope: Scope {
            pub path: Path,
            pub is_scope: Method
        },
        pub traverse: Traverse {
            pub path: Path,
            pub descendant_at: Method,
            pub descendant_at_and_collect: Method,
            pub traverse_and_collect: Method
        },
        pub parent: Parent {
            pub path: Path,
            pub inject_parent: Method
        },
        pub display: Display {
            pub path: Path,
            pub fmt: Method
        },
        pub indented_display: IndentedDisplay {
            pub path: Path,
            pub fmt_with_indent: Method,
        }
    }
);

impl Default for Paths {
    fn default() -> Self {
        Self {
            symbol: core_ast(parse_quote!(Symbol)),
            dyn_symbol: core_ast(parse_quote!(DynSymbol)),
            weak_symbol: core_ast(parse_quote!(WeakSymbol)),
            symbol_data: core_ast(parse_quote!(SymbolData)),
            pending_symbol: core_build(parse_quote!(PendingSymbol)),
            maybe_pending_symbol: core_build(parse_quote!(MaybePendingSymbol)),
            parsers: parse_quote!(auto_lsp::core::parsers::Parsers),

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
                    sig: quote! { #[inline] fn get_data(&self) -> &auto_lsp::core::ast::SymbolData },
                    variant: quote! { get_data() },
                },
                get_mut_data: Method {
                    sig: quote! { #[doc(hidden)] #[inline] fn get_mut_data(&mut self) -> &mut auto_lsp::core::ast::SymbolData },
                    variant: quote! { get_mut_data() },
                },
            },

            symbol_builder_trait: SymbolBuilderTrait {
                path: core_build(parse_quote!(Buildable)),
                new: Method {
                    sig: quote! { fn new(
                        query: &auto_lsp::tree_sitter::Query,
                        capture: &auto_lsp::tree_sitter::QueryCapture,
                    ) -> Option<Self> },
                    variant: quote! { new(query, capture) },
                },
                add: Method {
                    sig: quote! { fn add(
                        &mut self,
                        capture: &auto_lsp::tree_sitter::QueryCapture,
                        parsers: &'static auto_lsp::core::parsers::Parsers,
                        document: &auto_lsp::core::document::Document,
                    ) -> Result<Option<auto_lsp::core::build::PendingSymbol>, auto_lsp::core::errors::AstError> },
                    variant: quote! { add(capture, parsers, document) },
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
            add_symbol_trait: core_build(parse_quote!(AddSymbol)),
            try_downcast_trait: core_build(parse_quote!(TryDownCast)),
            finalize_trait: core_build(parse_quote!(Finalize)),

            lsp_document_symbols: LspDocumentSymbols {
                path: core_ast(parse_quote!(BuildDocumentSymbols)),
                build_document_symbols: Method {
                    sig: quote! { fn build_document_symbols(&self, doc: &auto_lsp::core::document::Document, builder: &mut auto_lsp::core::document_symbols_builder::DocumentSymbolsBuilder) -> auto_lsp::anyhow::Result<()> },
                    variant: quote! { build_document_symbols(doc, builder) },
                },
            },
            lsp_code_lens: LspCodeLens {
                path: core_ast(parse_quote!(BuildCodeLenses)),
                build_code_lenses: Method {
                    sig: quote! { fn build_code_lenses(&self, doc: &auto_lsp::core::document::Document, acc: &mut Vec<auto_lsp::lsp_types::CodeLens>) -> auto_lsp::anyhow::Result<()> },
                    variant: quote! { build_code_lenses(doc, acc) },
                },
            },
            lsp_code_actions: LspCodeActions {
                path: core_ast(parse_quote!(BuildCodeActions)),
                build_code_actions: Method {
                    sig: quote! { fn build_code_actions(&self, doc: &auto_lsp::core::document::Document, acc: &mut Vec<auto_lsp::lsp_types::CodeActionOrCommand>) -> auto_lsp::anyhow::Result<()>  },
                    variant: quote! { build_code_actions(doc, acc) },
                },
            },
            lsp_completion_items: LspCompletionItems {
                path: core_ast(parse_quote!(BuildCompletionItems)),
                build_completion_items: Method {
                    sig: quote! { fn build_completion_items(&self, doc: &auto_lsp::core::document::Document, acc: &mut Vec<auto_lsp::lsp_types::CompletionItem>) -> auto_lsp::anyhow::Result<()> },
                    variant: quote! { build_completion_items(doc, acc) },
                },
            },
            lsp_invoked_completion_items: LspInvokedCompletionItems {
                path: core_ast(parse_quote!(BuildTriggeredCompletionItems)),
                build_triggered_completion_items: Method {
                    sig: quote! { fn build_triggered_completion_items(&self, trigger: &str, doc: &auto_lsp::core::document::Document, acc: &mut Vec<auto_lsp::lsp_types::CompletionItem>)  -> auto_lsp::anyhow::Result<()>  },
                    variant: quote! { build_triggered_completion_items(trigger, doc, acc) },
                },
            },
            lsp_go_to_definition: LspGoToDefinition {
                path: core_ast(parse_quote!(GetGoToDefinition)),
                go_to_definition: Method {
                    sig: quote! { fn go_to_definition(&self, doc: &auto_lsp::core::document::Document) -> auto_lsp::anyhow::Result<Option<auto_lsp::lsp_types::GotoDefinitionResponse>> },
                    variant: quote! { go_to_definition(doc) },
                },
            },
            lsp_go_to_declaration: LspGoToDeclaration {
                path: core_ast(parse_quote!(GetGoToDeclaration)),
                go_to_declaration: Method {
                    sig: quote! { fn go_to_declaration(&self, doc: &auto_lsp::core::document::Document) -> auto_lsp::anyhow::Result<Option<auto_lsp::lsp_types::request::GotoDeclarationResponse>> },
                    variant: quote! { go_to_declaration(doc) },
                },
            },
            lsp_hover_info: LspHoverInfo {
                path: core_ast(parse_quote!(GetHover)),
                get_hover: Method {
                    sig: quote! { fn get_hover(&self, doc: &auto_lsp::core::document::Document) -> anyhow::Result<Option<auto_lsp::lsp_types::Hover>> },
                    variant: quote! { get_hover(doc) },
                },
            },
            lsp_inlay_hint: LspInlayHint {
                path: core_ast(parse_quote!(BuildInlayHints)),
                build_inlay_hints: Method {
                    sig: quote! { fn build_inlay_hints(&self, doc: &auto_lsp::core::document::Document, acc: &mut Vec<auto_lsp::lsp_types::InlayHint>) -> auto_lsp::anyhow::Result<()>  },
                    variant: quote! { build_inlay_hints(doc, acc) },
                },
            },
            lsp_semantic_token: LspSemanticToken {
                path: core_ast(parse_quote!(BuildSemanticTokens)),
                build_semantic_tokens: Method {
                    sig: quote! { fn build_semantic_tokens(&self, doc: &auto_lsp::core::document::Document, builder: &mut auto_lsp::core::semantic_tokens_builder::SemanticTokensBuilder) -> auto_lsp::anyhow::Result<()>  },
                    variant: quote! { build_semantic_tokens(doc, builder) },
                },
            },
            scope: Scope {
                path: core_ast(parse_quote!(Scope)),
                is_scope: Method {
                    sig: quote! { fn is_scope(&self) -> bool },
                    variant: quote! { is_scope() },
                },
            },
            traverse: Traverse {
                path: core_ast(parse_quote!(Traverse)),
                descendant_at: Method {
                    sig: quote! { fn descendant_at(&self, offset: usize) -> Option<auto_lsp::core::ast::DynSymbol> },
                    variant: quote! { descendant_at(offset) },
                },
                descendant_at_and_collect: Method {
                    sig: quote! { fn descendant_at_and_collect(&self, offset: usize, collect_fn: fn(auto_lsp::core::ast::DynSymbol) -> bool, collect: &mut Vec<auto_lsp::core::ast::DynSymbol>) -> Option<auto_lsp::core::ast::DynSymbol> },
                    variant: quote! { descendant_at_and_collect(offset, collect_fn, collect) },
                },
                traverse_and_collect: Method {
                    sig: quote! { fn traverse_and_collect(&self, collect_fn: fn(auto_lsp::core::ast::DynSymbol) -> bool, collect: &mut Vec<auto_lsp::core::ast::DynSymbol>) },
                    variant: quote! { traverse_and_collect(collect_fn, collect) },
                },
            },
            parent: Parent {
                path: core_build(parse_quote!(Parent)),
                inject_parent: Method {
                    sig: quote! { fn inject_parent(&mut self, parent: auto_lsp::core::ast::WeakSymbol) },
                    variant: quote! { inject_parent(parent) },
                },
            },
            display: Display {
                path: parse_quote!(std::fmt::Display),
                fmt: Method {
                    sig: quote! { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result },
                    variant: quote! { fmt(f) },
                },
            },
            indented_display: IndentedDisplay {
                path: core_ast(parse_quote!(IndentedDisplay)),
                fmt_with_indent: Method {
                    sig: quote! { fn fmt_with_indent(&self, f: &mut std::fmt::Formatter, indent: usize) -> std::fmt::Result },
                    variant: quote! { fmt_with_indent(f, indent) },
                },
            },
        }
    }
}
