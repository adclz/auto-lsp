use crate::{
    utilities::extract_fields::{FieldBuilder, FieldBuilderType, FieldInfoExtract, StructFields},
    BuildAstItem, BuildAstItemBuilder, Features, FeaturesCodeGen, Paths, SymbolFeatures, ToCodeGen,
};
use auto_lsp::traits::queryable;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::Attribute;

pub struct StructBuilder<'a> {
    // Input data
    pub input_attr: &'a Vec<Attribute>,
    pub input_name: &'a Ident,
    pub query_name: &'a str,
    pub input_buider_name: &'a Ident,
    pub fields: &'a StructFields,
    pub is_accessor: bool,
    // Paths
    pub paths: &'a Paths,
    // Features
    pub features: Features<'a>,
}

impl<'a> StructBuilder<'a> {
    pub fn new(
        params: Option<&'a SymbolFeatures>,
        input_attr: &'a Vec<Attribute>,
        input_name: &'a Ident,
        input_buider_name: &'a Ident,
        query_name: &'a str,
        fields: &'a StructFields,
        paths: &'a Paths,
        is_accessor: bool,
    ) -> Self {
        Self {
            input_name,
            input_attr,
            query_name,
            input_buider_name,
            fields,
            is_accessor,
            paths,
            features: Features::new(params, is_accessor, input_name, paths, fields),
        }
    }
}

impl<'a> ToTokens for StructBuilder<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let input_name = &self.input_name;
        let input_attr = &self.input_attr;
        let query_name = self.query_name;

        // Generate features
        let mut code_gen = FeaturesCodeGen::default();
        self.features.to_code_gen(&mut code_gen);

        let input_fields = code_gen.input.fields;
        let features_impl = code_gen.input.impl_base;
        let features_impl_ast = code_gen.input.impl_ast_item;
        let features_others_impl = code_gen.input.other_impl;

        // generate ast item
        let ast_item_trait = &self.paths.ast_item_trait;

        let fields = self.generate_fields();
        let methods = self.generate_ast_item_methods();

        let locator = self.generate_locator_trait();
        let parent = self.generate_parent_trait();
        let queryable = self.generate_queryable();

        tokens.extend(quote! {
            #(#input_attr)*
            #[derive(Clone)]
            pub struct #input_name {
                #(#fields,)*
                #(#input_fields),*
            }

            impl #input_name {
                pub const QUERY_NAMES: &[&str] = &[#query_name];
                #(#features_impl)*
            }

            impl #ast_item_trait for #input_name {
                #methods
                #(#features_impl_ast)*
            }

            #(#features_others_impl)*
            #locator
            #parent
            #queryable
        });

        // Generate builder

        let input_builder_name = &self.input_buider_name;
        let ast_item_builder = &self.paths.ast_item_builder_trait;

        let builder_fields = self.generate_builder_fields();
        let new = self.generate_builder_new();
        let query_binder = self.generate_query_binder();
        let add = self.generate_add();
        let try_from = self.generate_try_from();

        let dyn_symbol = &self.paths.dyn_symbol;
        let try_from_builder = &self.paths.try_from_builder;

        let into = quote! {
            fn try_to_dyn_symbol(&self, check: &mut Vec<#dyn_symbol>) -> Result<#dyn_symbol, lsp_types::Diagnostic> {
                use #try_from_builder;

                let item = #input_name::try_from_builder(self, check)?;
                Ok(#dyn_symbol::new(item))
            }
        };

        tokens.extend(quote! {
            #[derive(Clone, Debug)]
            pub struct #input_builder_name {
                url: std::sync::Arc<lsp_types::Url>,
                query_index: usize,
                range: tree_sitter::Range,
                start_position: tree_sitter::Point,
                end_position: tree_sitter::Point,
                #(#builder_fields),*
            }

            impl #ast_item_builder for #input_builder_name {
                #new
                #query_binder
                #add
                #into

                fn get_url(&self) -> std::sync::Arc<lsp_types::Url> {
                    self.url.clone()
                }

                fn get_range(&self) -> tree_sitter::Range {
                    self.range
                }

                fn get_query_index(&self) -> usize {
                    self.query_index
                }
            }

            #try_from
        });
    }
}

impl<'a> StructBuilder<'a> {
    fn generate_queryable(&self) -> TokenStream {
        let query_name = self.query_name;
        let input_name = &self.input_name;
        let input_builder_name = &self.input_buider_name;
        let queryable = &self.paths.queryable;

        quote! {
            impl #queryable for #input_name {
                const QUERY_NAMES: &'static [&'static str] = &[#query_name];
            }

            impl #queryable for #input_builder_name {
                const QUERY_NAMES: &'static [&'static str] = &[#query_name];
            }
        }
    }
}

impl<'a> StructBuilder<'a> {
    fn generate_locator_trait(&self) -> TokenStream {
        let locator = &self.paths.locator;
        let input_name = &self.input_name;
        let dyn_symbol = &self.paths.dyn_symbol;

        let builder = FieldBuilder::new(&self.fields)
            .apply_all(|_, _, name, _, _| {
                quote! {
                    if let Some(symbol) = self.#name.find_at_offset(offset) {
                       return Some(symbol);
                    }
                }
            })
            .to_token_stream();

        quote! {
            impl #locator for #input_name {
                fn find_at_offset(&self, offset: usize) -> Option<#dyn_symbol> {
                    if (!self.is_inside_offset(offset)) {
                        return None;
                    }

                    #builder

                    None
                }
            }
        }
    }
}

impl<'a> StructBuilder<'a> {
    fn generate_parent_trait(&self) -> TokenStream {
        let parent = &self.paths.parent;
        let input_name = &self.input_name;
        let weak_symbol = &self.paths.weak_symbol;

        let builder = FieldBuilder::new(&self.fields)
            .apply_all(|_, _, name, _, _| {
                quote! {
                    self.#name.inject_parent(parent.clone());
                }
            })
            .to_token_stream();

        quote! {
            impl #parent for #input_name {
                fn inject_parent(&mut self, parent: #weak_symbol) {
                    #builder
                }
            }
        }
    }
}

impl<'a> BuildAstItem for StructBuilder<'a> {
    fn generate_fields(&self) -> Vec<TokenStream> {
        let symbol = &self.paths.symbol;
        let weak_symbol = &self.paths.weak_symbol;

        let mut fields = vec![
            quote! { pub url: std::sync::Arc<lsp_types::Url> },
            quote! { pub parent: Option<#weak_symbol> },
            quote! { pub range: tree_sitter::Range },
            quote! { pub start_position: tree_sitter::Point },
            quote! { pub end_position: tree_sitter::Point },
        ];

        if self.is_accessor {
            fields.push(quote! { pub accessor: Option<#weak_symbol> });
        }

        let mut builder = FieldBuilder::new(&self.fields);

        builder.apply_all(|ty, attributes, name, field_type, _| match ty {
            FieldBuilderType::Normal => quote! {
                #(#attributes)*
                pub #name: #symbol<#field_type>
            },
            FieldBuilderType::Vec => quote! {
                #(#attributes)*
                pub #name: Vec<#symbol<#field_type>>
            },
            FieldBuilderType::Option => quote! {
                #(#attributes)*
                pub #name: Option<#symbol<#field_type>>
            },
            FieldBuilderType::HashMap => quote! {
                #(#attributes)*
                pub #name: HashMap<String, #symbol<#field_type>>
            },
        });

        fields.extend::<Vec<TokenStream>>(builder.into());
        fields
    }

    fn generate_ast_item_methods(&self) -> TokenStream {
        let weak_symbol = &self.paths.weak_symbol;

        quote! {
            fn get_url(&self) -> std::sync::Arc<lsp_types::Url> {
                self.url.clone()
            }

            fn get_range(&self) -> tree_sitter::Range {
                self.range
            }

            fn get_parent(&self) -> Option<#weak_symbol> {
                self.parent.as_ref().map(|p| p.clone())
            }

            fn set_parent(&mut self, parent: #weak_symbol) {
                self.parent = Some(parent);
            }
        }
    }
}

impl<'a> BuildAstItemBuilder for StructBuilder<'a> {
    fn generate_builder_fields(&self) -> Vec<TokenStream> {
        let maybe_pending_symbol = &self.paths.maybe_pending_symbol;
        let pending_symbol = &self.paths.pending_symbol;

        let mut builder = FieldBuilder::new(&self.fields);

        builder.apply_all(|ty, _, name, _, _| match ty {
            FieldBuilderType::Vec => quote! { #name: Vec<#pending_symbol> },
            FieldBuilderType::HashMap => quote! { #name: HashMap<String, #pending_symbol> },
            _ => quote! { #name: #maybe_pending_symbol },
        });
        builder.into()
    }

    fn generate_builder_new(&self) -> TokenStream {
        let maybe_pending_symbol = &self.paths.maybe_pending_symbol;

        let fields = FieldBuilder::new(&self.fields)
            .apply_all(|ty, _, name, _, _| match ty {
                FieldBuilderType::Vec => quote! { #name: vec![], },
                FieldBuilderType::HashMap => quote! { #name: std::collections::HashMap::new(), },
                _ => quote! { #name: #maybe_pending_symbol::none(), },
            })
            .to_token_stream();

        quote! {
            fn new(url: std::sync::Arc<lsp_types::Url>, _query: &tree_sitter::Query, query_index: usize, range: tree_sitter::Range, start_position: tree_sitter::Point, end_position: tree_sitter::Point) -> Option<Self> {
                Some(Self {
                    url,
                    query_index,
                    range,
                    start_position,
                    end_position,
                    #fields
                })
            }
        }
    }

    fn generate_query_binder(&self) -> TokenStream {
        let maybe_pending_symbol = &self.paths.maybe_pending_symbol;

        let fields_types = self.fields.get_field_types();
        let fields_builder = self.fields.get_field_builder_names();

        let query_binder = quote! {
            let query_name = query.capture_names()[capture.index as usize];
            #(
                if #fields_types::QUERY_NAMES.contains(&query_name)  {
                    match #fields_builder::new(url, query, capture.index as usize, capture.node.range(), capture.node.start_position(), capture.node.end_position()) {
                        Some(builder) => return #maybe_pending_symbol::new(builder),
                        None => return #maybe_pending_symbol::none()
                    }
                };
            )*
            #maybe_pending_symbol::none()
        };

        quote! {
            fn query_binder(&self, url: std::sync::Arc<lsp_types::Url>, capture: &tree_sitter::QueryCapture, query: &tree_sitter::Query) -> #maybe_pending_symbol {
                #query_binder
            }
        }
    }

    fn generate_add(&self) -> TokenStream {
        let pending_symbol = &self.paths.pending_symbol;

        let deferred_closure = &self.paths.deferred_closure;

        let input_builder_name = &self.input_buider_name;

        let builder = FieldBuilder::new(&self.fields)
            .apply_all(|_, _, name, field_type, _| {
                quote! {
                    node = match self.#name.add::<#field_type>(query_name, node, range, "", "")? {
                        Some(a) => a,
                        None => return Ok(None),
                    };

                }
            })
            .into_token_stream();

        quote! {
            fn add(&mut self, query: &tree_sitter::Query, node: #pending_symbol, source_code: &[u8]) ->
                Result<Option<#deferred_closure>, lsp_types::Diagnostic> {

                let query_name = query.capture_names()[node.get_query_index()];
                let range = self.get_lsp_range();
                let mut node = node;

                #builder

                Err(auto_lsp::builder_error!(self.get_lsp_range(), format!("Invalid field {:?} in {:?}", query_name, stringify!(#input_builder_name))))
            }
        }
    }

    fn generate_try_from(&self) -> TokenStream {
        let fields = self.fields.get_field_names();

        let name = self.input_name;
        let input_builder_name = &self.input_buider_name;

        let try_from_builder = &self.paths.try_from_builder;

        let dyn_symbol = &self.paths.dyn_symbol;

        let builder = FieldBuilder::new(self.fields)
            .apply_all(|ty, _, name, _, _| match ty  {
                FieldBuilderType::Normal  => quote! {
                    let #name = builder
                        .#name
                        .try_downcast(check, stringify!(#name), builder_range, stringify!(#input_builder_name))?
                        //todo ! remove expect with builder error
                        .expect("");
                },
                _=> quote! {
                        let #name = builder
                            .#name
                            .try_downcast(check, stringify!(#name), builder_range, stringify!(#input_builder_name))?;
                    }
            }).to_token_stream();

        let init_accessor = if self.is_accessor {
            quote! { accessor: None, }
        } else {
            quote! {}
        };

        quote! {
            impl #try_from_builder<&#input_builder_name> for #name {
                type Error = lsp_types::Diagnostic;

                fn try_from_builder(builder: &#input_builder_name, check: &mut Vec<#dyn_symbol>) -> Result<Self, Self::Error> {
                    let builder_range = builder.get_lsp_range();

                    #builder

                    Ok(#name {
                        #init_accessor
                        url: builder.url.clone(),
                        range: builder.range.clone(),
                        start_position: builder.start_position.clone(),
                        end_position: builder.end_position.clone(),
                        parent: None,
                        #(#fields),*
                    })
                }
            }
        }
    }
}
