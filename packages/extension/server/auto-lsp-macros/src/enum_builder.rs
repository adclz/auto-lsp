use crate::utilities::extract_fields::{EnumFields, VariantBuilder};
use crate::{BuildAstItem, BuildAstItemBuilder, PATHS};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

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
        let name = self.input_name;
        let input_builder_name = &self.input_builder_name;

        let builder_fields = self.generate_builder_fields();
        let builder_new = self.generate_builder_new();
        let query_binder = self.generate_query_binder();
        let add = self.generate_add();
        let try_from = self.generate_try_from();

        let fields = self.generate_fields();
        let symbol_methods = self.generate_symbol_methods();

        let builder = VariantBuilder::new(&self)
            .generate_check()
            .generate_code_lens()
            .generate_completion_items()
            .generate_document_symbol()
            .generate_hover_info()
            .generate_inlay_hint()
            .generate_semantic_tokens()
            .generate_go_to_definition()
            .generate_go_to_declaration()
            .generate_parent()
            .generate_locator()
            .generate_scope()
            .generate_accessor()
            .to_token_stream();

        let pending_symbol = &PATHS.symbol_builder_trait;
        let dyn_symbol = &PATHS.dyn_symbol;

        let try_from_builder = &PATHS.try_from_builder;

        let into = quote! {
            fn try_to_dyn_symbol(&self, check: &mut Vec<#dyn_symbol>) -> Result<#dyn_symbol, lsp_types::Diagnostic> {
                use #try_from_builder;

                let item = #name::try_from_builder(self, check)?;
                Ok(#dyn_symbol::new(item))
            }
        };

        tokens.extend(quote! {
            pub enum #name {
                #(
                    #fields
                )*
            }

            #symbol_methods

            pub struct #input_builder_name {
                #(
                    #builder_fields
                )*
            }

            impl #pending_symbol for #input_builder_name {
                #builder_new
                #query_binder
                #add
                #into

                fn get_url(&self) -> std::sync::Arc<lsp_types::Url> {
                    self.unique_field.get_rc().borrow().get_url()
                }

                fn get_range(&self) -> tree_sitter::Range {
                    self.unique_field.get_rc().borrow().get_range()
                }

                fn get_query_index(&self) -> usize {
                    self.unique_field.get_rc().borrow().get_query_index()
                }
            }

            #try_from
            #builder
        });
    }
}

impl<'a> BuildAstItemBuilder for EnumBuilder<'a> {
    fn generate_builder_fields(&self) -> Vec<TokenStream> {
        let pending_symbol: &syn::Path = &PATHS.pending_symbol;
        vec![quote! { pub unique_field: #pending_symbol }]
    }

    fn generate_builder_new(&self) -> TokenStream {
        let pending_symbol = &PATHS.pending_symbol;

        let variant_types_names = &self.fields.variant_types_names;
        let variant_builder_names = &self.fields.variant_builder_names;

        quote! {
            fn new(url: std::sync::Arc<lsp_types::Url>, query: &tree_sitter::Query, query_index: usize, range: tree_sitter::Range, start_position: tree_sitter::Point, end_position: tree_sitter::Point) -> Option<Self> {
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
        }
    }

    fn generate_query_binder(&self) -> TokenStream {
        let maybe_pending_symbol = &PATHS.maybe_pending_symbol;

        quote! {
            fn query_binder(&self, url: std::sync::Arc<lsp_types::Url>, capture: &tree_sitter::QueryCapture, query: &tree_sitter::Query) -> #maybe_pending_symbol {
                self.unique_field.get_rc().borrow().query_binder(url, capture, query)
            }
        }
    }

    fn generate_add(&self) -> TokenStream {
        let pending_symbol = &PATHS.pending_symbol;

        quote! {
            fn add(&mut self, query: &tree_sitter::Query, node: #pending_symbol, source_code: &[u8]) ->
                Result<(), lsp_types::Diagnostic> {
                    self.unique_field.get_rc().borrow_mut().add(query, node, source_code)
            }
        }
    }

    fn generate_try_from(&self) -> TokenStream {
        let name = self.input_name;
        let input_builder_name = &self.input_builder_name;

        let variant_names = &self.fields.variant_names;
        let variant_builder_names = &self.fields.variant_builder_names;

        let try_from_builder = &PATHS.try_from_builder;
        let try_into_builder = &PATHS.try_into_builder;

        let dyn_symbol = &PATHS.dyn_symbol;

        quote! {
            impl #try_from_builder<&#input_builder_name> for #name {
                type Error = lsp_types::Diagnostic;

                fn try_from_builder(builder: &#input_builder_name, check: &mut Vec<#dyn_symbol>) -> Result<Self, Self::Error> {
                    use #try_into_builder;

                    #(
                        if let Some(variant) = builder.unique_field.get_rc().borrow().downcast_ref::<#variant_builder_names>() {
                            return Ok(Self::#variant_names(variant.try_into_builder(check)?));
                        };
                    )*
                    Err(auto_lsp::builder_error!(
                        builder.unique_field.get_rc().borrow().get_lsp_range(),
                        format!("Failed to downcast builder to enum: {}", stringify!(#name))
                    ))
                }
            }
        }
    }
}

impl<'a> BuildAstItem for EnumBuilder<'a> {
    fn generate_fields(&self) -> Vec<TokenStream> {
        let variant_names = &self.fields.variant_names;
        let variant_types_names = &self.fields.variant_types_names;

        vec![quote! {
            #(
                #variant_names(#variant_types_names)
            ),*
        }]
    }

    fn generate_symbol_methods(&self) -> TokenStream {
        let symbol_data = &PATHS.symbol_data;

        VariantBuilder::new(&self)
            .dispatch(
                &PATHS.symbol_trait,
                vec![
                    (
                        &quote! { fn get_data(&self) -> &#symbol_data },
                        &quote! { get_data() },
                    ),
                    (
                        &quote! { fn get_mut_data(&mut self) -> &mut #symbol_data },
                        &quote! { get_mut_data() },
                    ),
                ],
            )
            .to_token_stream()
    }
}

impl<'a> VariantBuilder<'a> {
    fn generate_check(&mut self) -> &mut Self {
        self.dispatch(
            &PATHS.check.path,
            vec![
                (
                    &PATHS.check.methods.must_check.sig,
                    &PATHS.check.methods.must_check.variant,
                ),
                (
                    &PATHS.check.methods.check.sig,
                    &PATHS.check.methods.check.variant,
                ),
            ],
        )
    }

    fn generate_locator(&mut self) -> &mut Self {
        self.dispatch(
            &PATHS.locator.path,
            vec![(
                &PATHS.locator.methods.find_at_offset.sig,
                &PATHS.locator.methods.find_at_offset.variant,
            )],
        )
    }

    fn generate_parent(&mut self) -> &mut Self {
        self.dispatch(
            &PATHS.parent.path,
            vec![(
                &PATHS.parent.methods.inject_parent.sig,
                &PATHS.parent.methods.inject_parent.variant,
            )],
        )
    }

    fn generate_code_lens(&mut self) -> &mut Self {
        self.dispatch(
            &PATHS.lsp_code_lens.path,
            vec![(
                &PATHS.lsp_code_lens.methods.build_code_lens.sig,
                &PATHS.lsp_code_lens.methods.build_code_lens.variant,
            )],
        )
    }

    fn generate_completion_items(&mut self) -> &mut Self {
        self.dispatch(
            &PATHS.lsp_completion_items.path,
            vec![(
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
            )],
        )
    }

    fn generate_document_symbol(&mut self) -> &mut Self {
        self.dispatch(
            &PATHS.lsp_document_symbols.path,
            vec![(
                &PATHS.lsp_document_symbols.methods.get_document_symbols.sig,
                &PATHS
                    .lsp_document_symbols
                    .methods
                    .get_document_symbols
                    .variant,
            )],
        )
    }

    fn generate_hover_info(&mut self) -> &mut Self {
        self.dispatch(
            &PATHS.lsp_hover_info.path,
            vec![(
                &PATHS.lsp_hover_info.methods.get_hover.sig,
                &PATHS.lsp_hover_info.methods.get_hover.variant,
            )],
        )
    }

    fn generate_inlay_hint(&mut self) -> &mut Self {
        self.dispatch(
            &PATHS.lsp_inlay_hint.path,
            vec![(
                &PATHS.lsp_inlay_hint.methods.build_inlay_hint.sig,
                &PATHS.lsp_inlay_hint.methods.build_inlay_hint.variant,
            )],
        )
    }

    fn generate_semantic_tokens(&mut self) -> &mut Self {
        self.dispatch(
            &PATHS.lsp_semantic_token.path,
            vec![(
                &PATHS.lsp_semantic_token.methods.build_semantic_tokens.sig,
                &PATHS
                    .lsp_semantic_token
                    .methods
                    .build_semantic_tokens
                    .variant,
            )],
        )
    }

    fn generate_go_to_definition(&mut self) -> &mut Self {
        self.dispatch(
            &PATHS.lsp_go_to_definition.path,
            vec![(
                &PATHS.lsp_go_to_definition.methods.go_to_definition.sig,
                &PATHS.lsp_go_to_definition.methods.go_to_definition.variant,
            )],
        )
    }

    fn generate_go_to_declaration(&mut self) -> &mut Self {
        self.dispatch(
            &PATHS.lsp_go_to_declaration.path,
            vec![(
                &PATHS.lsp_go_to_declaration.methods.go_to_declaration.sig,
                &PATHS
                    .lsp_go_to_declaration
                    .methods
                    .go_to_declaration
                    .variant,
            )],
        )
    }

    fn generate_scope(&mut self) -> &mut Self {
        self.dispatch(
            &PATHS.scope.path,
            vec![
                (
                    &PATHS.scope.methods.is_scope.sig,
                    &PATHS.scope.methods.is_scope.variant,
                ),
                (
                    &PATHS.scope.methods.get_scope_range.sig,
                    &PATHS.scope.methods.get_scope_range.variant,
                ),
            ],
        )
    }

    fn generate_accessor(&mut self) -> &mut Self {
        self.dispatch(
            &PATHS.is_accessor.path,
            vec![
                (
                    &PATHS.is_accessor.methods.is_accessor.sig,
                    &PATHS.is_accessor.methods.is_accessor.variant,
                ),
                (
                    &PATHS.is_accessor.methods.set_accessor.sig,
                    &PATHS.is_accessor.methods.set_accessor.variant,
                ),
            ],
        );

        self.dispatch(
            &PATHS.accessor.path,
            vec![(
                &PATHS.accessor.methods.find.sig,
                &PATHS.accessor.methods.find.variant,
            )],
        )
    }
}
