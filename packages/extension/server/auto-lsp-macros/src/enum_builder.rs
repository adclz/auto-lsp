use crate::utilities::extract_fields::EnumFields;
use crate::PATHS;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Ident, Path};

pub struct EnumBuilder<'a> {
    pub fields: &'a EnumFields,
    pub input_name: &'a Ident,
    pub input_builder_name: &'a Ident,
}

impl<'a> EnumBuilder<'a> {
    pub fn new(
        input_name: &'a Ident,
        input_builder_name: &'a Ident,
        fields: &'a EnumFields,
    ) -> Self {
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
        self.impl_dynamic_swap(&mut builder);
        self.impl_edit_range(&mut builder);
        self.impl_queryable(&mut builder);
        self.impl_parent(&mut builder);
        self.impl_scope(&mut builder);
        self.impl_comment(&mut builder);

        self.impl_check(&mut builder);
        self.impl_accessor(&mut builder);
        self.impl_code_lens(&mut builder);
        self.impl_completion_items(&mut builder);
        self.impl_document_symbol(&mut builder);
        self.impl_hover_info(&mut builder);
        self.impl_inlay_hint(&mut builder);
        self.impl_semantic_tokens(&mut builder);
        self.impl_go_to_definition(&mut builder);
        self.impl_go_to_declaration(&mut builder);

        // Generate builder

        self.struct_input_builder(&mut builder);

        builder.add(quote! {
            fn get_url(&self) -> std::sync::Arc<lsp_types::Url> {
                self.unique_field.get_rc().borrow().get_url()
            }

            fn get_range(&self) -> std::ops::Range<usize> {
                self.unique_field.get_rc().borrow().get_range()
            }

            fn get_query_index(&self) -> usize {
                self.unique_field.get_rc().borrow().get_query_index()
            }
        });
        self.fn_try_to_dyn_symbol(&mut builder);
        self.fn_new(&mut builder);
        self.fn_query_binder(&mut builder);
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
            .add_default_iter(
                &self.fields,
                &PATHS.symbol_trait.methods.get_data.sig,
                &PATHS.symbol_trait.methods.get_data.variant,
            )
            .add_default_iter(
                &self.fields,
                &PATHS.symbol_trait.methods.get_mut_data.sig,
                &PATHS.symbol_trait.methods.get_mut_data.variant,
            )
            .stage_trait(&self.input_name, &PATHS.symbol_trait.path);
    }

    fn impl_check(&self, builder: &mut VariantBuilder) {
        builder
            .add_default_iter(
                &self.fields,
                &PATHS.is_check.methods.must_check.sig,
                &PATHS.is_check.methods.must_check.variant,
            )
            .stage_trait(&self.input_name, &PATHS.is_check.path);

        builder
            .add_default_iter(
                &self.fields,
                &PATHS.check.methods.check.sig,
                &PATHS.check.methods.check.variant,
            )
            .stage_trait(&self.input_name, &PATHS.check.path);
    }

    fn impl_accessor(&self, builder: &mut VariantBuilder) {
        builder
            .add_default_iter(
                &self.fields,
                &PATHS.is_accessor.methods.is_accessor.sig,
                &PATHS.is_accessor.methods.is_accessor.variant,
            )
            .add_default_iter(
                &self.fields,
                &PATHS.is_accessor.methods.set_accessor.sig,
                &PATHS.is_accessor.methods.set_accessor.variant,
            )
            .add_default_iter(
                &self.fields,
                &PATHS.is_accessor.methods.reset_accessor.sig,
                &PATHS.is_accessor.methods.reset_accessor.variant,
            )
            .add_default_iter(
                &self.fields,
                &PATHS.is_accessor.methods.get_accessor.sig,
                &PATHS.is_accessor.methods.get_accessor.variant,
            )
            .stage_trait(&self.input_name, &PATHS.is_accessor.path)
            .add_default_iter(
                &self.fields,
                &PATHS.accessor.methods.find.sig,
                &PATHS.accessor.methods.find.variant,
            )
            .stage_trait(&self.input_name, &PATHS.accessor.path);
    }

    fn impl_locator(&self, builder: &mut VariantBuilder) {
        builder
            .add_default_iter(
                &self.fields,
                &PATHS.locator.methods.find_at_offset.sig,
                &PATHS.locator.methods.find_at_offset.variant,
            )
            .stage_trait(&self.input_name, &PATHS.locator.path);
    }

    fn impl_dynamic_swap(&self, builder: &mut VariantBuilder) {
        builder
            .add_default_iter(
                &self.fields,
                &PATHS.dynamic_swap.methods.swap.sig,
                &PATHS.dynamic_swap.methods.swap.variant,
            )
            .stage_trait(&self.input_name, &PATHS.dynamic_swap.path);
    }

    fn impl_edit_range(&self, builder: &mut VariantBuilder) {
        builder
            .add_default_iter(
                &self.fields,
                &PATHS.edit_range.methods.edit_range.sig,
                &PATHS.edit_range.methods.edit_range.variant,
            )
            .stage_trait(&self.input_name, &PATHS.edit_range.path);
    }

    fn impl_queryable(&self, builder: &mut VariantBuilder) {
        let queryable = &PATHS.queryable.path;
        let check_queryable = &PATHS.check_queryable.path;

        let concat: Vec<_> = self
            .fields
            .variant_types_names
            .iter()
            .map(|name| quote! { #name::QUERY_NAMES })
            .collect();

        builder
            .add(quote! { const QUERY_NAMES: &'static [&'static str] = {
                    use #queryable;
                    constcat::concat_slices!([&'static str]: #(#concat),*)
                };
            })
            .stage_trait(&self.input_name, queryable);

        builder
            .add(quote! { const QUERY_NAMES: &'static [&'static str] = {
                    use #queryable;
                    constcat::concat_slices!([&'static str]: #(#concat),*)
                };
            })
            .stage_trait(&self.input_builder_name, queryable);

        let names = self
            .fields
            .variant_names
            .iter()
            .map(|name| quote! { stringify!(#name) })
            .collect::<Vec<_>>();

        let names = quote! { &[#(#names),*] };

        let input_name = self.input_name;

        builder
            .add(quote! { const CHECK: () = {
                use #queryable;
                use #check_queryable;
                let queries = constcat::concat_slices!([&str]: #(#concat),*);
                auto_lsp::queryable::check_conflicts(stringify!(#input_name), #names, queries);
            }; })
            .stage_trait(&self.input_name, check_queryable);

        builder
            .add(quote! { const _: () = <#input_name as  #check_queryable>::CHECK; })
            .stage();
    }

    fn impl_parent(&self, builder: &mut VariantBuilder) {
        builder
            .add_default_iter(
                &self.fields,
                &PATHS.parent.methods.inject_parent.sig,
                &PATHS.parent.methods.inject_parent.variant,
            )
            .stage_trait(&self.input_name, &PATHS.parent.path);
    }

    fn impl_scope(&self, builder: &mut VariantBuilder) {
        builder
            .add_default_iter(
                &self.fields,
                &PATHS.is_scope.methods.is_scope.sig,
                &PATHS.is_scope.methods.is_scope.variant,
            )
            .stage_trait(&self.input_name, &PATHS.is_scope.path);

        builder
            .add_default_iter(
                &self.fields,
                &PATHS.scope.methods.get_scope_range.sig,
                &PATHS.scope.methods.get_scope_range.variant,
            )
            .stage_trait(&self.input_name, &PATHS.scope.path);
    }

    fn impl_comment(&self, builder: &mut VariantBuilder) {
        builder
            .add_default_iter(
                &self.fields,
                &PATHS.is_comment.methods.is_comment.sig,
                &PATHS.is_comment.methods.is_comment.variant,
            )
            .stage_trait(&self.input_name, &PATHS.is_comment.path);
    }

    fn impl_code_lens(&self, builder: &mut VariantBuilder) {
        builder
            .add_default_iter(
                &self.fields,
                &PATHS.lsp_code_lens.methods.build_code_lens.sig,
                &PATHS.lsp_code_lens.methods.build_code_lens.variant,
            )
            .stage_trait(&self.input_name, &PATHS.lsp_code_lens.path);
    }

    fn impl_completion_items(&self, builder: &mut VariantBuilder) {
        builder
            .add_default_iter(
                &self.fields,
                &PATHS
                    .lsp_completion_items
                    .methods
                    .build_completion_items
                    .sig,
                &PATHS
                    .lsp_completion_items
                    .methods
                    .build_completion_items
                    .variant,
            )
            .stage_trait(&self.input_name, &PATHS.lsp_completion_items.path);
    }

    fn impl_document_symbol(&self, builder: &mut VariantBuilder) {
        builder
            .add_default_iter(
                &self.fields,
                &PATHS.lsp_document_symbols.methods.get_document_symbols.sig,
                &PATHS
                    .lsp_document_symbols
                    .methods
                    .get_document_symbols
                    .variant,
            )
            .stage_trait(&self.input_name, &PATHS.lsp_document_symbols.path);
    }

    fn impl_hover_info(&self, builder: &mut VariantBuilder) {
        builder
            .add_default_iter(
                &self.fields,
                &PATHS.lsp_hover_info.methods.get_hover.sig,
                &PATHS.lsp_hover_info.methods.get_hover.variant,
            )
            .stage_trait(&self.input_name, &PATHS.lsp_hover_info.path);
    }

    fn impl_inlay_hint(&self, builder: &mut VariantBuilder) {
        builder
            .add_default_iter(
                &self.fields,
                &PATHS.lsp_inlay_hint.methods.build_inlay_hint.sig,
                &PATHS.lsp_inlay_hint.methods.build_inlay_hint.variant,
            )
            .stage_trait(&self.input_name, &PATHS.lsp_inlay_hint.path);
    }

    fn impl_semantic_tokens(&self, builder: &mut VariantBuilder) {
        builder
            .add_default_iter(
                &self.fields,
                &PATHS.lsp_semantic_token.methods.build_semantic_tokens.sig,
                &PATHS
                    .lsp_semantic_token
                    .methods
                    .build_semantic_tokens
                    .variant,
            )
            .stage_trait(&self.input_name, &PATHS.lsp_semantic_token.path);
    }

    fn impl_go_to_definition(&self, builder: &mut VariantBuilder) {
        builder
            .add_default_iter(
                &self.fields,
                &PATHS.lsp_go_to_definition.methods.go_to_definition.sig,
                &PATHS.lsp_go_to_definition.methods.go_to_definition.variant,
            )
            .stage_trait(&self.input_name, &PATHS.lsp_go_to_definition.path);
    }

    fn impl_go_to_declaration(&self, builder: &mut VariantBuilder) {
        builder
            .add_default_iter(
                &self.fields,
                &PATHS.lsp_go_to_declaration.methods.go_to_declaration.sig,
                &PATHS
                    .lsp_go_to_declaration
                    .methods
                    .go_to_declaration
                    .variant,
            )
            .stage_trait(&self.input_name, &PATHS.lsp_go_to_declaration.path);
    }

    fn struct_input_builder(&self, builder: &mut VariantBuilder) {
        let pending_symbol: &syn::Path = &PATHS.pending_symbol;
        builder
            .add(quote! { pub unique_field: #pending_symbol })
            .stage_struct(&self.input_builder_name);
    }

    fn fn_try_to_dyn_symbol(&self, builder: &mut VariantBuilder) {
        let dyn_symbol = &PATHS.dyn_symbol;
        let sig = &PATHS.symbol_builder_trait.methods.try_to_dyn_symbol.sig;
        let input_name = &self.input_name;
        let try_from_builder = &PATHS.try_from_builder;

        builder.add(quote! {
            #sig {
                use #try_from_builder;

                let item = #input_name::try_from_builder(self, check)?;
                Ok(#dyn_symbol::new(item))
            }
        });
    }

    fn fn_new(&self, builder: &mut VariantBuilder) {
        let queryable = &PATHS.queryable.path;
        let pending_symbol = &PATHS.pending_symbol;

        let variant_types_names = &self.fields.variant_types_names;
        let variant_builder_names = &self.fields.variant_builder_names;

        builder.add(quote! {
            fn new(url: std::sync::Arc<lsp_types::Url>, query: &tree_sitter::Query, query_index: usize, range: tree_sitter::Range, start_position: tree_sitter::Point, end_position: tree_sitter::Point) -> Option<Self> {
                use #queryable;
                let query_name = query.capture_names()[query_index as usize];
                #(
                    if #variant_types_names::QUERY_NAMES.contains(&query_name) {
                        match #variant_builder_names::new(url, query, query_index, range, start_position, end_position) {
                            Some(builder) => return Some(Self {
                                unique_field: #pending_symbol::new(builder),
                            }),
                            None => return None,
                        }
                    };
                )*
                None
            }
        });
    }

    fn fn_query_binder(&self, builder: &mut VariantBuilder) {
        let maybe_pending_symbol = &PATHS.maybe_pending_symbol;

        builder.add(quote! {
            fn query_binder(&self, url: std::sync::Arc<lsp_types::Url>, capture: &tree_sitter::QueryCapture, query: &tree_sitter::Query) -> #maybe_pending_symbol {
                self.unique_field.get_rc().borrow().query_binder(url, capture, query)
            }
        });
    }

    fn fn_add(&self, builder: &mut VariantBuilder) {
        let pending_symbol = &PATHS.pending_symbol;
        let params = &PATHS.builder_params;

        builder.add(quote! {
            fn add(&mut self, query: &tree_sitter::Query, node: #pending_symbol, source_code: &[u8], params: &mut #params) ->
                Result<(), lsp_types::Diagnostic> {
                    self.unique_field.get_rc().borrow_mut().add(query, node, source_code, params)
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

        let params = &PATHS.builder_params;

        builder.add(quote! {
            impl #try_from_builder<&#input_builder_name> for #name {
                type Error = lsp_types::Diagnostic;

                fn try_from_builder(builder: &#input_builder_name, params: &mut #params) -> Result<Self, Self::Error> {
                    use #try_into_builder;

                    #(
                        if let Some(variant) = builder.unique_field.get_rc().borrow().downcast_ref::<#variant_builder_names>() {
                            return Ok(Self::#variant_names(variant.try_into_builder(params)?));
                        };
                    )*
                    Err(auto_lsp::builder_error!(
                        builder.unique_field.get_rc().borrow().get_lsp_range(params.doc),
                        format!("Failed to downcast builder to enum: {}", stringify!(#name))
                    ))
                }
            }
        });
        builder.stage();
    }
}

#[derive(Default)]
pub struct VariantBuilder {
    staged: Vec<TokenStream>,
    unstaged: Vec<TokenStream>,
}

impl VariantBuilder {
    pub fn add(&mut self, field: TokenStream) -> &mut Self {
        self.unstaged.push(field);
        self
    }

    pub fn add_iter<F>(&mut self, variants: &EnumFields, body: F) -> &mut Self
    where
        F: Fn(&Ident, &Ident, &Ident) -> TokenStream,
    {
        let variants = variants
            .variant_names
            .iter()
            .zip(variants.variant_types_names.iter())
            .zip(variants.variant_builder_names.iter())
            .map(|((name, _type), builder)| body(name, _type, builder))
            .collect::<Vec<_>>();

        self.unstaged.extend(variants);
        self
    }

    pub fn add_default_iter(
        &mut self,
        variants: &EnumFields,
        sig_path: &TokenStream,
        default: &TokenStream,
    ) -> &mut Self {
        let variants = variants
            .variant_names
            .iter()
            .map(|name| {
                quote! {
                    Self::#name(inner) => inner.#default,
                }
            })
            .collect::<Vec<_>>();

        self.unstaged.push(quote! {
            #sig_path {
                match self {
                    #(#variants)*
                }
            }
        });
        self
    }

    pub fn add_fn_iter<F>(
        &mut self,
        variants: &EnumFields,
        sig_path: &TokenStream,
        before: Option<TokenStream>,
        body: F,
        after: Option<TokenStream>,
    ) -> &mut Self
    where
        F: Fn(&Ident, &Ident, &Ident) -> TokenStream,
    {
        let variants = variants
            .variant_names
            .iter()
            .zip(variants.variant_types_names.iter())
            .zip(variants.variant_builder_names.iter())
            .map(|((name, _type), builder)| {
                let body = body(name, _type, builder);
                quote! {
                    Self::#name(inner) => inner.#body,
                }
            })
            .collect::<Vec<_>>();

        let mut result = TokenStream::default();
        if let Some(before) = before {
            result.extend(before);
        }

        result.extend(variants);

        if let Some(after) = after {
            result.extend(after);
        }

        self.unstaged.push(quote! {
            #sig_path {
                match self {
                    #result
                }
            }
        });
        self
    }

    fn drain(&mut self) -> Vec<TokenStream> {
        std::mem::take(&mut self.unstaged)
    }

    pub fn stage(&mut self) -> &mut Self {
        let drain = self.drain();
        self.staged.extend(drain);
        self
    }

    pub fn stage_trait(&mut self, input_name: &Ident, trait_path: &Path) -> &mut Self {
        let drain = self.drain();
        let result = quote! {
            impl #trait_path for #input_name {
                #(#drain)*
            }
        };
        self.staged.push(result);
        self
    }

    pub fn stage_struct(&mut self, input_name: &Ident) -> &mut Self {
        let drain = self.drain();
        let result = quote! {
            #[derive(Clone)]
            pub struct #input_name {
                #(#drain,)*
            }
        };
        self.staged.push(result);
        self
    }

    pub fn stage_enum(&mut self, input_name: &Ident) -> &mut Self {
        let drain = self.drain();
        let result = quote! {
            #[derive(Clone)]
            pub enum #input_name {
                #(#drain,)*
            }
        };
        self.staged.push(result);
        self
    }
}

impl ToTokens for VariantBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.staged.clone());
    }
}

impl<'a> From<VariantBuilder> for Vec<TokenStream> {
    fn from(builder: VariantBuilder) -> Self {
        builder.staged
    }
}
