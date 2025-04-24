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

use crate::Paths;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

use super::variant_builder::{VariantBuilder, Variants};

pub struct EnumBuilder<'a> {
    pub paths: &'a Paths,
    pub fields: &'a Variants,
    pub input_name: &'a Ident,
    pub input_builder_name: &'a Ident,
}

impl<'a> EnumBuilder<'a> {
    pub fn new(
        paths: &'a Paths,
        input_name: &'a Ident,
        input_builder_name: &'a Ident,
        fields: &'a Variants,
    ) -> Self {
        Self {
            paths,
            fields,
            input_name,
            input_builder_name,
        }
    }
}

impl ToTokens for EnumBuilder<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut builder = VariantBuilder::default();

        self.enum_input(&mut builder);

        self.impl_ast_symbol(&mut builder);
        self.impl_traverse(&mut builder);
        self.impl_indented_display(&mut builder);
        self.impl_queryable(&mut builder);
        self.impl_scope(&mut builder);

        self.impl_code_actions(&mut builder);
        self.impl_code_lens(&mut builder);
        self.impl_completion_items(&mut builder);
        self.impl_invoked_completion_items(&mut builder);
        self.impl_document_symbol(&mut builder);
        self.impl_hover_info(&mut builder);
        self.impl_inlay_hint(&mut builder);
        self.impl_semantic_tokens(&mut builder);
        self.impl_go_to_definition(&mut builder);
        self.impl_go_to_declaration(&mut builder);

        // Generate builder

        self.struct_input_builder(&mut builder);

        builder.add(quote! {
            fn get_range(&self) -> std::ops::Range<usize> {
                self.unique_field.get_rc().borrow().get_range()
            }

            fn get_query_index(&self) -> usize {
                self.unique_field.get_rc().borrow().get_query_index()
            }
        });
        self.fn_new(&mut builder);
        self.fn_add(&mut builder);
        builder.stage_trait(
            self.input_builder_name,
            &self.paths.symbol_builder_trait.path,
        );

        self.impl_try_from(&mut builder);

        tokens.extend(builder.to_token_stream());
    }
}

impl EnumBuilder<'_> {
    fn enum_input(&self, builder: &mut VariantBuilder) {
        builder
            .add_iter(self.fields, |name, _type, _builder| {
                quote! { #name(#_type) }
            })
            .stage_enum(self.input_name);
    }

    fn impl_ast_symbol(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                self.fields,
                &self.paths.symbol_trait.get_data.sig,
                &self.paths.symbol_trait.get_data.variant,
            )
            .stage_trait(self.input_name, &self.paths.symbol_trait.path);
    }

    fn impl_traverse(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                self.fields,
                &self.paths.traverse.descendant_at.sig,
                &self.paths.traverse.descendant_at.variant,
            )
            .add_pattern_match_iter(
                self.fields,
                &self.paths.traverse.descendant_at_and_collect.sig,
                &self.paths.traverse.descendant_at_and_collect.variant,
            )
            .add_pattern_match_iter(
                self.fields,
                &self.paths.traverse.traverse_and_collect.sig,
                &self.paths.traverse.traverse_and_collect.variant,
            )
            .stage_trait(self.input_name, &self.paths.traverse.path);
    }

    fn impl_indented_display(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                self.fields,
                &self.paths.display.fmt.sig,
                &self.paths.display.fmt.variant,
            )
            .stage_trait(self.input_name, &self.paths.display.path)
            .add_pattern_match_iter(
                self.fields,
                &self.paths.indented_display.fmt_with_indent.sig,
                &self.paths.indented_display.fmt_with_indent.variant,
            )
            .stage_trait(self.input_name, &self.paths.indented_display.path);
    }

    fn impl_queryable(&self, builder: &mut VariantBuilder) {
        let queryable = &self.paths.queryable.path;

        let concat: Vec<_> = self
            .fields
            .variant_builder_names
            .iter()
            .map(|name| quote! { #name::QUERY_NAMES })
            .collect();

        builder
            .add(quote! { const QUERY_NAMES: &'static [&'static str] = {
                    use #queryable;
                    auto_lsp::constcat::concat_slices!([&'static str]: #(#concat),*)
                };
            })
            .stage_trait(self.input_builder_name, queryable);
    }

    fn impl_scope(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                self.fields,
                &self.paths.scope.is_scope.sig,
                &self.paths.scope.is_scope.variant,
            )
            .stage_trait(self.input_name, &self.paths.scope.path);
    }

    fn impl_code_actions(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                self.fields,
                &self.paths.lsp_code_actions.build_code_actions.sig,
                &self.paths.lsp_code_actions.build_code_actions.variant,
            )
            .stage_trait(self.input_name, &self.paths.lsp_code_actions.path);
    }

    fn impl_code_lens(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                self.fields,
                &self.paths.lsp_code_lens.build_code_lenses.sig,
                &self.paths.lsp_code_lens.build_code_lenses.variant,
            )
            .stage_trait(self.input_name, &self.paths.lsp_code_lens.path);
    }

    fn impl_completion_items(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                self.fields,
                &self.paths.lsp_completion_items.build_completion_items.sig,
                &self
                    .paths
                    .lsp_completion_items
                    .build_completion_items
                    .variant,
            )
            .stage_trait(self.input_name, &self.paths.lsp_completion_items.path);
    }

    fn impl_invoked_completion_items(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                self.fields,
                &self
                    .paths
                    .lsp_invoked_completion_items
                    .build_triggered_completion_items
                    .sig,
                &self
                    .paths
                    .lsp_invoked_completion_items
                    .build_triggered_completion_items
                    .variant,
            )
            .stage_trait(
                self.input_name,
                &self.paths.lsp_invoked_completion_items.path,
            );
    }

    fn impl_document_symbol(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                self.fields,
                &self.paths.lsp_document_symbols.build_document_symbols.sig,
                &self
                    .paths
                    .lsp_document_symbols
                    .build_document_symbols
                    .variant,
            )
            .stage_trait(self.input_name, &self.paths.lsp_document_symbols.path);
    }

    fn impl_hover_info(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                self.fields,
                &self.paths.lsp_hover_info.get_hover.sig,
                &self.paths.lsp_hover_info.get_hover.variant,
            )
            .stage_trait(self.input_name, &self.paths.lsp_hover_info.path);
    }

    fn impl_inlay_hint(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                self.fields,
                &self.paths.lsp_inlay_hint.build_inlay_hints.sig,
                &self.paths.lsp_inlay_hint.build_inlay_hints.variant,
            )
            .stage_trait(self.input_name, &self.paths.lsp_inlay_hint.path);
    }

    fn impl_semantic_tokens(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                self.fields,
                &self.paths.lsp_semantic_token.build_semantic_tokens.sig,
                &self.paths.lsp_semantic_token.build_semantic_tokens.variant,
            )
            .stage_trait(self.input_name, &self.paths.lsp_semantic_token.path);
    }

    fn impl_go_to_definition(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                self.fields,
                &self.paths.lsp_go_to_definition.go_to_definition.sig,
                &self.paths.lsp_go_to_definition.go_to_definition.variant,
            )
            .stage_trait(self.input_name, &self.paths.lsp_go_to_definition.path);
    }

    fn impl_go_to_declaration(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                self.fields,
                &self.paths.lsp_go_to_declaration.go_to_declaration.sig,
                &self.paths.lsp_go_to_declaration.go_to_declaration.variant,
            )
            .stage_trait(self.input_name, &self.paths.lsp_go_to_declaration.path);
    }

    fn struct_input_builder(&self, builder: &mut VariantBuilder) {
        let pending_symbol: &syn::Path = &self.paths.pending_symbol;
        builder
            .add(quote! { pub unique_field: #pending_symbol })
            .stage_struct(self.input_builder_name);
    }

    fn fn_new(&self, builder: &mut VariantBuilder) {
        let queryable = &self.paths.queryable.path;
        let pending_symbol = &self.paths.pending_symbol;

        let variant_builder_names = &self.fields.variant_builder_names;

        let sig = &self.paths.symbol_builder_trait.new.sig;
        let variant = &self.paths.symbol_builder_trait.new.variant;

        builder.add(quote! {
            #sig {
                use #queryable;
                let query_name = query.capture_names()[capture.index as usize];
                #(
                    if #variant_builder_names::QUERY_NAMES.contains(&query_name) {
                        return #variant_builder_names::#variant.and_then(|b| {
                            Some(Self {
                                unique_field: #pending_symbol::new(b),
                            })
                        });
                    };
                )*
                None
            }
        });
    }

    fn fn_add(&self, builder: &mut VariantBuilder) {
        let sig = &self.paths.symbol_builder_trait.add.sig;
        let variant = &self.paths.symbol_builder_trait.add.variant;

        builder.add(quote! {
            #sig {
                self.unique_field.get_rc().borrow_mut().#variant
            }
        });
    }

    fn impl_try_from(&self, builder: &mut VariantBuilder) {
        let name = self.input_name;
        let input_builder_name = &self.input_builder_name;

        let variant_names = &self.fields.variant_names;
        let variant_types = &self.fields.variant_types_names;
        let variant_builder_names = &self.fields.variant_builder_names;

        let parsers = &self.paths.parsers;

        builder.add(quote! {
            impl TryFrom<(
                &#input_builder_name,
                &auto_lsp::core::document::Document,
                &'static #parsers,
                &mut auto_lsp::id_arena::Arena<std::sync::Arc<dyn auto_lsp::core::ast::AstSymbol>>,
            )> for #name {
                type Error = auto_lsp::core::errors::AstError;

                fn try_from(
                    (builder, document, parsers, arena): (
                        &#input_builder_name,
                        &auto_lsp::core::document::Document,
                        &'static #parsers,
                        &mut auto_lsp::id_arena::Arena<std::sync::Arc<dyn auto_lsp::core::ast::AstSymbol>>,
                    )
                ) -> Result<Self, Self::Error> {
                    #(
                        if let Some(variant) = builder.unique_field.get_rc().borrow().downcast_ref::<#variant_builder_names>() {
                            return Ok(Self::#variant_names(#variant_types::try_from((variant, document, parsers, arena))?));
                        };
                    )*
                    Err(auto_lsp::core::errors::AstError::UnknownSymbol {
                        range: builder.unique_field.get_rc().borrow().get_range(),
                        symbol: parsers.core.capture_names()[builder.unique_field.get_query_index() as usize],
                        parent_name: stringify!(#name)
                    })
                }
            }
        });
        builder.stage();
    }
}
