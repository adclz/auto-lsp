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

#![allow(unused)]
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

use super::field_builder::Fields;
use crate::{DarlingInput, Paths};
pub struct Features<'a> {
    paths: &'a Paths,
    darling_input: &'a DarlingInput,
    input_name: &'a Ident,
    fields: &'a Fields,
}

impl<'a> Features<'a> {
    pub fn new(
        paths: &'a Paths,
        darling_input: &'a DarlingInput,
        input_name: &'a Ident,
        fields: &'a Fields,
    ) -> Self {
        Self {
            paths,
            darling_input,
            input_name,
            fields,
        }
    }
}

impl ToTokens for Features<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let input_name = self.input_name;

        if !self.darling_input.declaration.is_present() {
            let declaration = &self.paths.lsp_go_to_declaration.path;
            tokens.extend(quote! {
                impl #declaration for #input_name {}
            });
        }

        if !self.darling_input.definition.is_present() {
            let definition = &self.paths.lsp_go_to_definition.path;
            tokens.extend(quote! {
                impl #definition for #input_name {}
            });
        }

        if !self.darling_input.hover.is_present() {
            let hover = &self.paths.lsp_hover_info.path;
            tokens.extend(quote! {
                impl #hover for #input_name {}
            });
        }

        if !self.darling_input.document_symbols.is_present() {
            let document_symbols = &self.paths.lsp_document_symbols.path;
            tokens.extend(quote! {
                impl #document_symbols for #input_name {}
            });
        }

        if !self.darling_input.code_actions.is_present() {
            let lsp_code_actions: &_ = &self.paths.lsp_code_actions.path;
            tokens.extend(quote! {
                impl #lsp_code_actions for #input_name {}
            });
        }

        if !self.darling_input.code_lenses.is_present() {
            let lsp_code_lens: &_ = &self.paths.lsp_code_lens.path;
            tokens.extend(quote! {
                impl #lsp_code_lens for #input_name {}
            });
        }

        if !self.darling_input.completions.is_present() {
            let lsp_completion_items = &self.paths.lsp_completion_items.path;
            tokens.extend(quote! {
                impl #lsp_completion_items for #input_name {}
            });
        }

        if !self.darling_input.triggered_completions.is_present() {
            let triggered_completions = &self.paths.lsp_invoked_completion_items.path;
            tokens.extend(quote! {
                impl #triggered_completions for #input_name {}
            });
        }

        if !self.darling_input.inlay_hints.is_present() {
            let inlay_hints = &self.paths.lsp_inlay_hint.path;
            tokens.extend(quote! {
                impl #inlay_hints for #input_name {}
            });
        }

        if !self.darling_input.semantic_tokens.is_present() {
            let semantic_tokens = &self.paths.lsp_semantic_token.path;
            tokens.extend(quote! {
                impl #semantic_tokens for #input_name {}
            });
        }

        // Speciald

        if !self.darling_input.scope.is_present() {
            let is_scope_path = &self.paths.scope.path;

            tokens.extend(quote! {
                impl #is_scope_path for #input_name {}
            });
        } else {
            let is_scope_path = &self.paths.scope.path;
            let is_scope_sig = &self.paths.scope.is_scope.sig;

            tokens.extend(quote! {
                impl #is_scope_path for #input_name {
                    #is_scope_sig {
                        true
                    }
                }
            });
        }
    }
}
