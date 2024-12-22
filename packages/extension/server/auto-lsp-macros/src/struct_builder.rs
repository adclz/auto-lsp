use crate::{
    utilities::extract_fields::{FieldBuilder, FieldBuilderType, StructFields},
    BuildAstItem, BuildAstItemBuilder, Features, ReferenceOrSymbolFeatures, StructHelpers, PATHS,
};
use darling::{ast, util};
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
    // Features
    pub features: Features<'a>,
}

impl<'a> StructBuilder<'a> {
    pub fn new(
        params: &'a ReferenceOrSymbolFeatures<'a>,
        helpers: &'a ast::Data<util::Ignored, StructHelpers>,
        input_attr: &'a Vec<Attribute>,
        input_name: &'a Ident,
        input_buider_name: &'a Ident,
        query_name: &'a str,
        fields: &'a StructFields,
    ) -> Self {
        Self {
            input_name,
            input_attr,
            query_name,
            input_buider_name,
            fields,
            features: Features::new(&params, &helpers, &input_name, &fields),
        }
    }
}

impl<'a> ToTokens for StructBuilder<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let input_name = &self.input_name;
        let input_attr = &self.input_attr;
        let query_name = self.query_name;

        let features = self.features.to_token_stream();

        // generate ast item
        let symbol_trait = &PATHS.symbol_trait;

        let fields = self.generate_fields();
        let methods = self.generate_symbol_methods();

        let locator = self.generate_locator_trait();

        let parent = self.generate_parent_trait();
        let queryable = self.generate_queryable();
        let dynamic_swap = self.generate_dynamic_swap();

        tokens.extend(quote! {
            #(#input_attr)*
            #[derive(Clone)]
            pub struct #input_name {
                #(#fields,)*
            }

            impl #input_name {
                pub const QUERY_NAMES: &[&str] = &[#query_name];
            }

            impl #symbol_trait for #input_name {
                #methods
            }

            #features
            #locator
            #parent
            #queryable
            #dynamic_swap
        });

        // Generate builder

        let input_builder_name = &self.input_buider_name;
        let pending_symbol = &PATHS.symbol_builder_trait;

        let builder_fields = self.generate_builder_fields();
        let new = self.generate_builder_new();
        let query_binder = self.generate_query_binder();
        let add = self.generate_add();
        let try_from = self.generate_try_from();

        let dyn_symbol = &PATHS.dyn_symbol;
        let try_from_builder = &PATHS.try_from_builder;

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

            impl #pending_symbol for #input_builder_name {
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
        let queryable = &PATHS.queryable;

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
        let locator = &PATHS.locator.path;
        let input_name = &self.input_name;
        let dyn_symbol = &PATHS.dyn_symbol;

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
        let parent = &PATHS.parent.path;
        let input_name = &self.input_name;
        let weak_symbol = &PATHS.weak_symbol;

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

    fn generate_dynamic_swap(&self) -> TokenStream {
        let input_name = &self.input_name;
        let dynamic_swap = &PATHS.dynamic_swap.path;

        let builder = FieldBuilder::new(&self.fields)
            .apply_all(|_, _, name, _, _| {
                quote! {
                    self.#name.to_swap(offset, builder_params)?;
                }
            })
            .to_token_stream();

        quote! {
            impl #dynamic_swap for #input_name {
                fn dyn_swap<'a>(
                    &mut self,
                    offset: usize,
                    builder_params: &'a mut BuilderParams,
                ) -> Result<(), Diagnostic> {
                    eprintln!("SWAP {:?}", stringify!(#input_name));
                    #builder
                    self.to_swap(offset, builder_params)
                }
            }
        }
    }
}

impl<'a> BuildAstItem for StructBuilder<'a> {
    fn generate_fields(&self) -> Vec<TokenStream> {
        let symbol = &PATHS.symbol;
        let symbol_data = &PATHS.symbol_data;

        let mut fields = vec![quote! { pub _data: #symbol_data }];

        let mut builder = FieldBuilder::new(&self.fields);

        builder.apply_all(|ty, _, name, field_type, _| match ty {
            FieldBuilderType::Normal => quote! {
                pub #name: #symbol<#field_type>
            },
            FieldBuilderType::Vec => quote! {
                pub #name: Vec<#symbol<#field_type>>
            },
            FieldBuilderType::Option => quote! {
                pub #name: Option<#symbol<#field_type>>
            },
        });

        fields.extend::<Vec<TokenStream>>(builder.into());
        fields
    }

    fn generate_symbol_methods(&self) -> TokenStream {
        let symbol_data = &PATHS.symbol_data;

        quote! {
            fn get_data(&self) -> &#symbol_data {
                &self._data
            }

            fn get_mut_data(&mut self) -> &mut #symbol_data {
                &mut self._data
            }
        }
    }
}

impl<'a> BuildAstItemBuilder for StructBuilder<'a> {
    fn generate_builder_fields(&self) -> Vec<TokenStream> {
        let maybe_pending_symbol = &PATHS.maybe_pending_symbol;
        let pending_symbol = &PATHS.pending_symbol;

        let mut builder = FieldBuilder::new(&self.fields);

        builder.apply_all(|ty, _, name, _, _| match ty {
            FieldBuilderType::Vec => quote! { #name: Vec<#pending_symbol> },
            _ => quote! { #name: #maybe_pending_symbol },
        });
        builder.into()
    }

    fn generate_builder_new(&self) -> TokenStream {
        let maybe_pending_symbol = &PATHS.maybe_pending_symbol;

        let fields = FieldBuilder::new(&self.fields)
            .apply_all(|ty, _, name, _, _| match ty {
                FieldBuilderType::Vec => quote! { #name: vec![], },
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
        let maybe_pending_symbol = &PATHS.maybe_pending_symbol;

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
        let pending_symbol = &PATHS.pending_symbol;

        let input_name = &self.input_name;
        let input_builder_name = &self.input_buider_name;

        let builder = FieldBuilder::new(&self.fields)
            .apply_all(|_, _, name, field_type, _| {
                quote! {
                    node = match self.#name.add::<#field_type>(query_name, node, range, stringify!(#input_name), stringify!(#field_type))? {
                        Some(a) => a,
                        None => return Ok(()),
                    };

                }
            })
            .into_token_stream();

        quote! {
            fn add(&mut self, query: &tree_sitter::Query, node: #pending_symbol, source_code: &[u8]) ->
                Result<(), lsp_types::Diagnostic> {

                let query_name = query.capture_names()[node.get_query_index()];
                let range = self.get_lsp_range();
                let mut node = node;

                #builder

                Err(auto_lsp::builder_error!(self.get_lsp_range(), format!("Invalid query {:?} in {:?}", query_name, stringify!(#input_name))))
            }
        }
    }

    fn generate_try_from(&self) -> TokenStream {
        let fields = self.fields.get_field_names();

        let input_name = self.input_name;
        let input_builder_name = &self.input_buider_name;

        let try_from_builder = &PATHS.try_from_builder;

        let symbol_data = &PATHS.symbol_data;
        let dyn_symbol = &PATHS.dyn_symbol;

        let builder = FieldBuilder::new(self.fields)
            .apply_all(|ty, _, name, field_type, _| match ty  {
                FieldBuilderType::Normal  => quote! {
                    let #name = Symbol::new_and_check(builder
                        .#name
                        .as_ref()
                        .ok_or(auto_lsp::builder_error!(
                            builder_range,
                            format!(
                                "Could not cast field {:?} in {:?}",
                                stringify!(#name), stringify!(#input_name)
                            )
                        ))?
                        .try_downcast(check, stringify!(#field_type), builder_range, stringify!(#input_name))?, check);
                },
                _=> quote! {
                        let #name = builder
                            .#name
                            .try_downcast(check, stringify!(#field_type), builder_range, stringify!(#input_name))?.finalize(check);
                    }
            }).to_token_stream();

        quote! {
            impl #try_from_builder<&#input_builder_name> for #input_name {
                type Error = lsp_types::Diagnostic;

                fn try_from_builder(builder: &#input_builder_name, check: &mut Vec<#dyn_symbol>) -> Result<Self, Self::Error> {
                    let builder_range = builder.get_lsp_range();

                    #builder

                    Ok(#input_name {
                        _data: #symbol_data::new(builder.url.clone(), builder.range.clone()),
                        #(#fields),*
                    })
                }
            }
        }
    }
}
