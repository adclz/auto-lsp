use crate::PATHS;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

use super::variant_builder::{VariantBuilder, Variants};

pub struct EnumBuilder<'a> {
    pub fields: &'a Variants,
    pub input_name: &'a Ident,
    pub input_builder_name: &'a Ident,
}

impl<'a> EnumBuilder<'a> {
    pub fn new(input_name: &'a Ident, input_builder_name: &'a Ident, fields: &'a Variants) -> Self {
        Self {
            fields,
            input_name,
            input_builder_name,
        }
    }
}

impl<'a> ToTokens for EnumBuilder<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut builder = VariantBuilder::default();

        self.enum_input(&mut builder);

        self.impl_ast_symbol(&mut builder);
        self.impl_locator(&mut builder);
        #[cfg(feature = "incremental")]
        self.impl_dynamic_swap(&mut builder);
        self.impl_queryable(&mut builder);
        self.impl_parent(&mut builder);
        self.impl_scope(&mut builder);
        self.impl_comment(&mut builder);

        self.impl_check(&mut builder);
        self.impl_reference(&mut builder);
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
            fn get_url(&self) -> std::sync::Arc<auto_lsp::lsp_types::Url> {
                self.unique_field.get_rc().borrow().get_url()
            }

            fn get_range(&self) -> std::ops::Range<usize> {
                self.unique_field.get_rc().borrow().get_range()
            }

            fn get_query_index(&self) -> usize {
                self.unique_field.get_rc().borrow().get_query_index()
            }
        });
        self.fn_new(&mut builder);
        self.fn_add(&mut builder);
        builder.stage_trait(&self.input_builder_name, &PATHS.symbol_builder_trait.path);

        self.impl_try_from(&mut builder);

        tokens.extend(builder.to_token_stream());
    }
}

impl<'a> EnumBuilder<'a> {
    fn enum_input(&self, builder: &mut VariantBuilder) {
        builder
            .add_iter(&self.fields, |name, _type, _builder| {
                quote! { #name(#_type) }
            })
            .stage_enum(&self.input_name);
    }

    fn impl_ast_symbol(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.symbol_trait.get_data.sig,
                &PATHS.symbol_trait.get_data.variant,
            )
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.symbol_trait.get_mut_data.sig,
                &PATHS.symbol_trait.get_mut_data.variant,
            )
            .stage_trait(&self.input_name, &PATHS.symbol_trait.path);
    }

    fn impl_check(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.is_check.must_check.sig,
                &PATHS.is_check.must_check.variant,
            )
            .stage_trait(&self.input_name, &PATHS.is_check.path);

        builder
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.check.check.sig,
                &PATHS.check.check.variant,
            )
            .stage_trait(&self.input_name, &PATHS.check.path);
    }

    fn impl_reference(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.is_reference.is_reference.sig,
                &PATHS.is_reference.is_reference.variant,
            )
            .stage_trait(&self.input_name, &PATHS.is_reference.path)
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.reference.find.sig,
                &PATHS.reference.find.variant,
            )
            .stage_trait(&self.input_name, &PATHS.reference.path);
    }

    fn impl_locator(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.locator.descendant_at.sig,
                &PATHS.locator.descendant_at.variant,
            )
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.locator.descendant_at_and_collect.sig,
                &PATHS.locator.descendant_at_and_collect.variant,
            )
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.locator.traverse_and_collect.sig,
                &PATHS.locator.traverse_and_collect.variant,
            )
            .stage_trait(&self.input_name, &PATHS.locator.path);
    }

    #[cfg(feature = "incremental")]
    fn impl_dynamic_swap(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.dynamic_swap.adjust.sig,
                &PATHS.dynamic_swap.adjust.variant,
            )
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.dynamic_swap.swap.sig,
                &PATHS.dynamic_swap.swap.variant,
            )
            .stage_trait(&self.input_name, &PATHS.dynamic_swap.path);
    }

    fn impl_queryable(&self, builder: &mut VariantBuilder) {
        let queryable = &PATHS.queryable.path;

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
            .stage_trait(&self.input_builder_name, queryable);
    }

    fn impl_parent(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.parent.inject_parent.sig,
                &PATHS.parent.inject_parent.variant,
            )
            .stage_trait(&self.input_name, &PATHS.parent.path);
    }

    fn impl_scope(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.scope.is_scope.sig,
                &PATHS.scope.is_scope.variant,
            )
            .stage_trait(&self.input_name, &PATHS.scope.path);
    }

    fn impl_comment(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.is_comment.is_comment.sig,
                &PATHS.is_comment.is_comment.variant,
            )
            .stage_trait(&self.input_name, &PATHS.is_comment.path);
    }

    fn impl_code_lens(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.lsp_code_lens.build_code_lens.sig,
                &PATHS.lsp_code_lens.build_code_lens.variant,
            )
            .stage_trait(&self.input_name, &PATHS.lsp_code_lens.path);
    }

    fn impl_completion_items(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.lsp_completion_items.build_completion_items.sig,
                &PATHS.lsp_completion_items.build_completion_items.variant,
            )
            .stage_trait(&self.input_name, &PATHS.lsp_completion_items.path);
    }

    fn impl_invoked_completion_items(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                &self.fields,
                &PATHS
                    .lsp_invoked_completion_items
                    .build_invoked_completion_items
                    .sig,
                &PATHS
                    .lsp_invoked_completion_items
                    .build_invoked_completion_items
                    .variant,
            )
            .stage_trait(&self.input_name, &PATHS.lsp_invoked_completion_items.path);
    }

    fn impl_document_symbol(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.lsp_document_symbols.build_document_symbols.sig,
                &PATHS.lsp_document_symbols.build_document_symbols.variant,
            )
            .stage_trait(&self.input_name, &PATHS.lsp_document_symbols.path);
    }

    fn impl_hover_info(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.lsp_hover_info.get_hover.sig,
                &PATHS.lsp_hover_info.get_hover.variant,
            )
            .stage_trait(&self.input_name, &PATHS.lsp_hover_info.path);
    }

    fn impl_inlay_hint(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.lsp_inlay_hint.build_inlay_hints.sig,
                &PATHS.lsp_inlay_hint.build_inlay_hints.variant,
            )
            .stage_trait(&self.input_name, &PATHS.lsp_inlay_hint.path);
    }

    fn impl_semantic_tokens(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.lsp_semantic_token.build_semantic_tokens.sig,
                &PATHS.lsp_semantic_token.build_semantic_tokens.variant,
            )
            .stage_trait(&self.input_name, &PATHS.lsp_semantic_token.path);
    }

    fn impl_go_to_definition(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.lsp_go_to_definition.go_to_definition.sig,
                &PATHS.lsp_go_to_definition.go_to_definition.variant,
            )
            .stage_trait(&self.input_name, &PATHS.lsp_go_to_definition.path);
    }

    fn impl_go_to_declaration(&self, builder: &mut VariantBuilder) {
        builder
            .add_pattern_match_iter(
                &self.fields,
                &PATHS.lsp_go_to_declaration.go_to_declaration.sig,
                &PATHS.lsp_go_to_declaration.go_to_declaration.variant,
            )
            .stage_trait(&self.input_name, &PATHS.lsp_go_to_declaration.path);
    }

    fn struct_input_builder(&self, builder: &mut VariantBuilder) {
        let pending_symbol: &syn::Path = &PATHS.pending_symbol;
        builder
            .add(quote! { pub unique_field: #pending_symbol })
            .stage_struct(&self.input_builder_name);
    }

    fn fn_new(&self, builder: &mut VariantBuilder) {
        let queryable = &PATHS.queryable.path;
        let pending_symbol = &PATHS.pending_symbol;

        let variant_builder_names = &self.fields.variant_builder_names;

        let sig = &PATHS.symbol_builder_trait.new.sig;
        let variant = &PATHS.symbol_builder_trait.new.variant;

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
        let sig = &PATHS.symbol_builder_trait.add.sig;
        let variant = &PATHS.symbol_builder_trait.add.variant;

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
        let variant_builder_names = &self.fields.variant_builder_names;

        let try_from_builder = &PATHS.try_from_builder;
        let try_into_builder = &PATHS.try_into_builder;

        let workspace = &PATHS.workspace;

        builder.add(quote! {
            impl #try_from_builder<&#input_builder_name> for #name {
                type Error = auto_lsp::lsp_types::Diagnostic;

                fn try_from_builder(builder: &#input_builder_name, workspace: &mut #workspace, document: &auto_lsp::core::document::Document) -> Result<Self, Self::Error> {
                    use #try_into_builder;

                    #(
                        if let Some(variant) = builder.unique_field.get_rc().borrow().downcast_ref::<#variant_builder_names>() {
                            return Ok(Self::#variant_names(variant.try_into_builder(workspace, document)?));
                        };
                    )*
                    Err(auto_lsp::core::builder_error!(
                        auto_lsp,
                        builder.unique_field.get_rc().borrow().get_lsp_range(document),
                        format!("Failed to downcast builder to enum: {}", stringify!(#name))
                    ))
                }
            }
        });
        builder.stage();
    }
}
