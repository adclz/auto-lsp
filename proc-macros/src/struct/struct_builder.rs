#![allow(unused)]
use super::{feature_builder::Features, field_builder::{FieldBuilder, FieldType, Fields}};
use crate::{
    DarlingInput,
    PATHS,
};
use darling::{ast, util};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{Attribute, Path};

/// Builder for generating the AST symbol from a struct.
/// 
/// This is the core builder called by the `#[seq]` macro.
/// 
/// It generates:
///     - The implementation of all capabilitties and `AstSymbol` traits. 
///     - The builder struct (named `input_builder_name`) that is used to create the AST symbol.
pub struct StructBuilder<'a> {
    // Input data
    pub input_attr: &'a Vec<Attribute>,
    pub input_name: &'a Ident,
    pub query_name: &'a str,
    pub input_builder_name: &'a Ident,
    pub fields: &'a Fields,
    // Features
    pub features: Features<'a>,
}

impl<'a> StructBuilder<'a> {
    pub fn new(
        darling_input: &'a DarlingInput,
        input_attr: &'a Vec<Attribute>,
        input_name: &'a Ident,
        input_builder_name: &'a Ident,
        query_name: &'a str,
        fields: &'a Fields,
    ) -> Self {
        Self {
            input_name,
            input_attr,
            query_name,
            input_builder_name,
            fields,
            features: Features::new(&darling_input, &input_name, &fields),
        }
    }
}

impl<'a> ToTokens for StructBuilder<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // generate ast item

        let mut builder = FieldBuilder::default();

        /// Create the struct 
        self.struct_input(&mut builder);

        // Implement the AstSymbol trait
        self.impl_ast_symbol(&mut builder);

        // Implement core capabilities
        self.impl_locator(&mut builder);
        self.impl_parent(&mut builder);
        #[cfg(feature = "incremental")]
        self.impl_dynamic_swap(&mut builder);

        // Implement other features
        builder.add(self.features.to_token_stream());
        builder.stage();

        // Generate builder struct
        self.struct_input_builder(&mut builder);

        // Implement `Buildable` trait
        builder.add(quote! {
            fn get_url(&self) -> std::sync::Arc<auto_lsp::lsp_types::Url> {
                self.url.clone()
            }

            fn get_range(&self) -> std::ops::Range<usize>{
                self.range.clone()
            }

            fn get_query_index(&self) -> usize {
                self.query_index
            }
        });
        self.fn_new(&mut builder);
        self.fn_add(&mut builder);
        builder.stage_trait(&self.input_builder_name, &PATHS.symbol_builder_trait.path);

        // Implement `TryFromBuilder`
        self.impl_try_from(&mut builder);

        // Implement `Queryable`
        self.impl_queryable(&mut builder);

        tokens.extend(builder.to_token_stream());
    }
}

impl<'a> StructBuilder<'a> {
    fn struct_input(&self, builder: &mut FieldBuilder) {
        let symbol = &PATHS.symbol;
        let symbol_data = &PATHS.symbol_data;

        builder
            .add(quote! { _data: #symbol_data })
            .add_iter(&self.fields, |ty, _, name, field_type, _| match ty {
                FieldType::Normal => quote! {
                    pub #name: #symbol<#field_type>
                },
                FieldType::Vec => quote! {
                    pub #name: Vec<#symbol<#field_type>>
                },
                FieldType::Option => quote! {
                    pub #name: Option<#symbol<#field_type>>
                },
            })
            .stage_struct(&self.input_name);
    }

    fn impl_ast_symbol(&self, builder: &mut FieldBuilder) {
        let get_data = &PATHS.symbol_trait.get_data.sig;
        let get_mut_data = &PATHS.symbol_trait.get_mut_data.sig;

        builder
            .add(quote! { #get_data { &self._data } })
            .add(quote! { #get_mut_data { &mut self._data } })
            .stage_trait(&self.input_name, &PATHS.symbol_trait.path);
    }

    fn impl_locator(&self, builder: &mut FieldBuilder) {
        let symbol_trait = &PATHS.symbol_trait.path;
        builder
            .add_fn_iter(
                &self.fields,
                &PATHS.locator.descendant_at.sig,
                Some(quote! {
                    use #symbol_trait;
                }),
                |_, _, name, _, _| {
                    quote! {
                        if let Some(symbol) = self.#name.descendant_at(offset) {
                           return Some(symbol);
                        }
                    }
                },
                Some(quote! { None }),
            )
            .add_fn_iter(
                &self.fields,
                &PATHS.locator.descendant_at_and_collect.sig,
                Some(quote! {
                    use #symbol_trait;
                }),
                |_, _, name, _, _| {
                    quote! {
                        if let Some(symbol) = self.#name.descendant_at_and_collect(offset, collect_fn, collect) {
                           return Some(symbol);
                        }
                    }
                },
                Some(quote! { None }),
            )
            .add_fn_iter(&self.fields, &PATHS.locator.traverse_and_collect.sig, None, 
                |_, _, name, _, _| {
                    quote! {
                        self.#name.traverse_and_collect(collect_fn, collect);
                    }
                },
            None
            )
            .stage_trait(&self.input_name, &PATHS.locator.path);
    }

    fn impl_parent(&self, builder: &mut FieldBuilder) {
        builder
            .add_fn_iter(
                &self.fields,
                &PATHS.parent.inject_parent.sig,
                None,
                |_, _, name, _, _| {
                    quote! {
                        self.#name.inject_parent(parent.clone());
                    }
                },
                None,
            )
            .stage_trait(&self.input_name, &PATHS.parent.path);
    }

    fn impl_queryable(&self, builder: &mut FieldBuilder) {
        let queryable = &PATHS.queryable.path;
        let query_name = self.query_name;

        builder
            .add(quote! { const QUERY_NAMES: &'static [&'static str] = &[#query_name]; })
            .stage_trait(&self.input_builder_name, queryable);
    }

    #[cfg(feature = "incremental")]
    fn impl_dynamic_swap(&self, builder: &mut FieldBuilder) {
        let static_update_trait = &PATHS.static_swap.path;
        builder
            .add_fn_iter(
                &self.fields,
                &PATHS.dynamic_swap.adjust.sig,
                Some(quote! { use #static_update_trait; }),
                |_, _, name, _, _| {
                    quote! {
                        self.#name.adjust(edit, collect, workspace, document);
                    }
                },
                None
            )
            .add_fn_iter(
                &self.fields,
                &PATHS.dynamic_swap.swap.sig,
                Some(quote! { use #static_update_trait; }),
                |_, _, name, _, _| {
                    quote! {
                        self.#name.update(edit, collect, workspace, document)?;
                    }
                },
                Some(quote! { std::ops::ControlFlow::Continue(()) }),
            )
            .stage_trait(&self.input_name, &PATHS.dynamic_swap.path);
    }

    fn struct_input_builder(&self, builder: &mut FieldBuilder) {
        let maybe_pending_symbol = &PATHS.maybe_pending_symbol;
        let pending_symbol = &PATHS.pending_symbol;

        builder
            .add(quote! { url: std::sync::Arc<auto_lsp::lsp_types::Url> })
            .add(quote! { query_index: usize })
            .add(quote! { range: std::ops::Range<usize> })
            .add_iter(&self.fields, |ty, _, name, _, _| match ty {
                FieldType::Vec => quote! { #name: Vec<#pending_symbol> },
                _ => quote! { #name: #maybe_pending_symbol },
            })
            .stage_struct(&self.input_builder_name)
            .to_token_stream();
    }

    fn fn_new(&self, builder: &mut FieldBuilder) {
        let maybe_pending_symbol = &PATHS.maybe_pending_symbol;
        let sig = &PATHS.symbol_builder_trait.new.sig;

        let fields = FieldBuilder::default()
            .add_iter(&self.fields, |ty, _, name, _, _| match ty {
                FieldType::Vec => quote! { #name: vec![] },
                _ => quote! { #name: #maybe_pending_symbol::none() },
            })
            .stage_fields()
            .to_token_stream();

        builder.add(quote! {
          #sig {
            let range = capture.node.range();
            Some(Self {
                url,
                query_index: capture.index as usize,
                range: std::ops::Range {
                    start: range.start_byte,
                    end: range.end_byte,
                },
                #fields
            })
          }
        });
    }

    fn fn_add(&self, builder: &mut FieldBuilder) {
        let input_name = &self.input_name;
        let add_symbol_trait = &PATHS.add_symbol_trait;
        builder.add_fn_iter(
            &self.fields,
            &PATHS.symbol_builder_trait.add.sig,
            Some(quote! { use #add_symbol_trait; }),
            |_, _, name, field_type, builder| {
                quote! {
                    
                    if let Some(node) =  self.#name.add::<#builder>(capture, workspace, stringify!(#input_name), stringify!(#field_type))? {
                       return Ok(Some(node))
                    };
                }
            },
            Some(quote! { Ok(None) }),
        );
    }

    fn impl_try_from(&self, builder: &mut FieldBuilder) {
        let fields = self.fields.get_field_names();

        let input_name = self.input_name;
        let input_builder_name = &self.input_builder_name;

        let try_from_builder = &PATHS.try_from_builder;

        let symbol_data = &PATHS.symbol_data;
        let workspace = &PATHS.workspace;
        let try_downcast = &PATHS.try_downcast_trait;
        let finalize = &PATHS.finalize_trait;
        let symbol = &PATHS.symbol;

        let _builder = FieldBuilder::default()
            .add(quote! {
                use #try_downcast;
                use #finalize;
            })
            .add_iter(&self.fields,
                |ty, _, name, field_type, _| match ty  {
                FieldType::Normal  => quote! {
                    let #name = #symbol::new_and_check(builder
                        .#name
                        .as_ref()
                        .ok_or(auto_lsp::core::builder_error!(
                            auto_lsp,
                            builder_range,
                            format!(
                                "Syntax error: Missing {:?} for {:?}",
                                stringify!(#name), 
                                stringify!(#input_name),
                            )
                        ))?
                        .try_downcast(workspace, document, stringify!(#field_type), builder_range, stringify!(#input_name))?, workspace);
                },
                _=> quote! {
                        let #name = builder
                            .#name
                            .try_downcast(workspace, document, stringify!(#field_type), builder_range, stringify!(#input_name))?.finalize(workspace);
                    }
            })
            .stage()
            .to_token_stream();

        let builder_trait = &PATHS.symbol_builder_trait.path;

        builder.add(quote! {
            impl #try_from_builder<&#input_builder_name> for #input_name {
                type Error = auto_lsp::lsp_types::Diagnostic;

                fn try_from_builder(builder: &#input_builder_name, workspace: &mut #workspace, document: &auto_lsp::core::document::Document) -> Result<Self, Self::Error> {
                    use #builder_trait;
                    let builder_range = builder.get_lsp_range(document);

                    #_builder

                    Ok(#input_name {
                        _data: #symbol_data::new(builder.url.clone(), builder.range.clone()),
                        #(#fields),*
                    })
                }
            }
        });
        builder.stage();
    }
}
