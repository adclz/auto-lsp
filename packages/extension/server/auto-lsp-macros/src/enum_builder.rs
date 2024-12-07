use crate::utilities::extract_fields::{EnumFields, SignatureAndBody, VariantBuilder};
use crate::{BuildAstItem, BuildAstItemBuilder, Paths};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

pub struct EnumBuilder<'a> {
    pub paths: &'a Paths,
    pub fields: &'a EnumFields,
    pub input_name: &'a Ident,
    pub input_builder_name: &'a Ident,
}

impl<'a> EnumBuilder<'a> {
    pub fn new(
        input_name: &'a Ident,
        input_builder_name: &'a Ident,
        fields: &'a EnumFields,
        paths: &'a Paths,
    ) -> Self {
        Self {
            paths,
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
            .generate_duplicate()
            .generate_code_lens()
            .generate_completion_items()
            .generate_document_symbol()
            .generate_hover_info()
            .generate_inlay_hint()
            .generate_semantic_tokens()
            .generate_go_to_definition()
            .generate_parent()
            .generate_locator()
            .generate_scope()
            .generate_accessor()
            .to_token_stream();

        let pending_symbol = &self.paths.symbol_builder_trait;
        let dyn_symbol = &self.paths.dyn_symbol;

        let try_from_builder = &self.paths.try_from_builder;

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
        let pending_symbol: &syn::Path = &self.paths.pending_symbol;
        vec![quote! { pub unique_field: #pending_symbol }]
    }

    fn generate_builder_new(&self) -> TokenStream {
        let pending_symbol = &self.paths.pending_symbol;

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
        let maybe_pending_symbol = &self.paths.maybe_pending_symbol;

        quote! {
            fn query_binder(&self, url: std::sync::Arc<lsp_types::Url>, capture: &tree_sitter::QueryCapture, query: &tree_sitter::Query) -> #maybe_pending_symbol {
                self.unique_field.get_rc().borrow().query_binder(url, capture, query)
            }
        }
    }

    fn generate_add(&self) -> TokenStream {
        let pending_symbol = &self.paths.pending_symbol;

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

        let try_from_builder = &self.paths.try_from_builder;
        let try_into_builder = &self.paths.try_into_builder;

        let dyn_symbol = &self.paths.dyn_symbol;

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
                    // todo!
                    panic!("Enum variant is not implemented")
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
        let weak_symbol = &self.paths.weak_symbol;

        VariantBuilder::new(&self)
        .dispatch(
            &self.paths.symbol_trait,
            vec![SignatureAndBody::new(
                quote! { fn get_url(&self) -> std::sync::Arc<lsp_types::Url> },
                quote! { get_url() },
            ),
            SignatureAndBody::new(
                quote! { fn get_range(&self) -> tree_sitter::Range },
                quote! { get_range() },
            ),
            SignatureAndBody::new(
                quote! { fn get_parent(&self) -> Option<#weak_symbol> },
                quote! { get_parent() },
            ),
            SignatureAndBody::new(
                quote! { fn set_parent(&mut self, parent: #weak_symbol) },
                quote! { set_parent(parent) },
            ),
            SignatureAndBody::new(
                quote! { fn get_start_position(&self, doc: &lsp_textdocument::FullTextDocument) -> lsp_types::Position },
                quote! { get_start_position(doc) },
            ),
            SignatureAndBody::new(
                quote! { fn get_end_position(&self, doc: &lsp_textdocument::FullTextDocument) -> lsp_types::Position },
                quote! { get_end_position(doc) },
            ),],
        )
        .to_token_stream()
    }
}

impl<'a> VariantBuilder<'a> {
    fn generate_duplicate(&mut self) -> &mut Self {
        self.dispatch(
            &self.enum_builder.paths.check_duplicate,
            vec![SignatureAndBody::new(
                quote! { fn must_check(&self) -> bool },
                quote! { must_check() },
            ), SignatureAndBody::new(
                quote! { fn check(&self, doc: &lsp_textdocument::FullTextDocument, diagnostics: &mut Vec<lsp_types::Diagnostic>) },
                quote! { check(doc, diagnostics) },
            )],
        )
    }

    fn generate_locator(&mut self) -> &mut Self {
        let dyn_symbol = &self.enum_builder.paths.dyn_symbol;

        self.dispatch(
            &self.enum_builder.paths.locator,
            vec![SignatureAndBody::new(
                quote! { fn find_at_offset(&self, offset: usize) -> Option<#dyn_symbol> },
                quote! { find_at_offset(offset) },
            )],
        )
    }

    fn generate_parent(&mut self) -> &mut Self {
        let weak_symbol = &self.enum_builder.paths.weak_symbol;

        self.dispatch(
            &self.enum_builder.paths.parent,
            vec![SignatureAndBody::new(
                quote! { fn inject_parent(&mut self, parent: #weak_symbol) },
                quote! { inject_parent(parent) },
            )],
        )
    }

    fn generate_code_lens(&mut self) -> &mut Self {
        self.dispatch(
            &self.enum_builder.paths.code_lens_trait,
            vec![SignatureAndBody::new(
                quote! { fn build_code_lens(&self, acc: &mut Vec<lsp_types::CodeLens>) },
                quote! { build_code_lens(acc) },
            )],
        )
    }

    fn generate_completion_items(&mut self) -> &mut Self {
        self.dispatch(
            &self.enum_builder.paths.completion_items_trait,
            vec![SignatureAndBody::new(
                quote! { fn build_completion_items(&self, acc: &mut Vec<lsp_types::CompletionItem>, doc: &lsp_textdocument::FullTextDocument) },
                quote! { build_completion_items(acc, doc) },
            )],
        )
    }

    fn generate_document_symbol(&mut self) -> &mut Self {
        self.dispatch(
            &self.enum_builder.paths.document_symbols_trait,
            vec![SignatureAndBody::new(
                quote! { fn get_document_symbols(&self, doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::DocumentSymbol> },
                quote! { get_document_symbols(doc) },
            )],
        )
    }

    fn generate_hover_info(&mut self) -> &mut Self {
        self.dispatch(
            &self.enum_builder.paths.hover_info_trait,
            vec![SignatureAndBody::new(
                quote! { fn get_hover(&self, doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::Hover> },
                quote! { get_hover(doc) },
            )],
        )
    }

    fn generate_inlay_hint(&mut self) -> &mut Self {
        self.dispatch(
            &self.enum_builder.paths.inlay_hints_trait,
            vec![SignatureAndBody::new(
                quote! { fn build_inlay_hint(&self, doc: &lsp_textdocument::FullTextDocument, acc: &mut Vec<lsp_types::InlayHint>) },
                quote! { build_inlay_hint(doc, acc) },
            )],
        )
    }

    fn generate_semantic_tokens(&mut self) -> &mut Self {
        let semangic_tokens_builder = &self.enum_builder.paths.semantic_tokens_builder;

        self.dispatch(
            &self.enum_builder.paths.semantic_tokens_trait,
            vec![SignatureAndBody::new(
                quote! { fn build_semantic_tokens(&self, builder: &mut #semangic_tokens_builder) },
                quote! { build_semantic_tokens(builder) },
            )],
        )
    }

    fn generate_go_to_definition(&mut self) -> &mut Self {
        self.dispatch(
            &self.enum_builder.paths.go_to_definition_trait,
            vec![
                SignatureAndBody::new(
                    quote! { fn go_to_definition(&self, doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::GotoDefinitionResponse> },
                    quote! { go_to_definition(doc) },
                ),
            ],
        )
    }

    fn generate_scope(&mut self) -> &mut Self {
        self.dispatch(
            &self.enum_builder.paths.scope_trait,
            vec![
                SignatureAndBody::new(quote! { fn is_scope(&self) -> bool }, quote! { is_scope() }),
                SignatureAndBody::new(
                    quote! { fn get_scope_range(&self) -> Vec<[usize; 2]> },
                    quote! { get_scope_range() },
                ),
            ],
        )
    }

    fn generate_accessor(&mut self) -> &mut Self {
        let weak_symbol = &self.enum_builder.paths.weak_symbol;

        self.dispatch(
            &self.enum_builder.paths.is_accessor_trait,
            vec![
                SignatureAndBody::new(
                    quote! { fn is_accessor(&self) -> bool },
                    quote! { is_accessor() },
                ),
                SignatureAndBody::new(
                    quote! { fn set_accessor(&mut self, accessor: #weak_symbol) },
                    quote! { set_accessor(accessor) },
                ),
            ],
        );

        self
        .dispatch(&self.enum_builder.paths.accessor_trait, 
            vec![SignatureAndBody::new(
                quote! { fn find(&self, doc: &lsp_textdocument::FullTextDocument, ctx: &dyn auto_lsp::workspace::WorkspaceContext) -> Result<Option<#weak_symbol>, lsp_types::Diagnostic> },
                quote! { find(doc, ctx) },
            )]
        )
    }
}
