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
use super::{
    feature_builder::Features,
    field_builder::{FieldBuilder, FieldType, Fields},
};
use crate::{DarlingInput, Paths};
use darling::{ast, util};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_quote, Attribute, Path};

/// Builder for generating the AST symbol from a struct.
///
/// This is the core builder called by the `#[seq]` macro.
///
/// It generates:
///     - The implementation of all capabilitties and `AstSymbol` traits.
///     - The builder struct (named `input_builder_name`) that is used to create the AST symbol.
pub struct StructBuilder<'a> {
    // Paths
    pub paths: &'a Paths,
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
        paths: &'a Paths,
        darling_input: &'a DarlingInput,
        input_attr: &'a Vec<Attribute>,
        input_name: &'a Ident,
        input_builder_name: &'a Ident,
        query_name: &'a str,
        fields: &'a Fields,
    ) -> Self {
        Self {
            paths,
            input_name,
            input_attr,
            query_name,
            input_builder_name,
            fields,
            features: Features::new(paths, darling_input, input_name, fields),
        }
    }
}

impl ToTokens for StructBuilder<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // generate ast item

        let mut builder = FieldBuilder::default();

        /// Create the struct
        self.struct_input(&mut builder);

        // Implement the AstSymbol trait
        self.impl_ast_symbol(&mut builder);

        // Implement core capabilities
        self.impl_traverse(&mut builder);
        self.impl_indented_display(&mut builder);

        // Implement other features
        builder.add(self.features.to_token_stream());
        builder.stage();

        // Generate builder struct
        self.struct_input_builder(&mut builder);

        // Implement `Buildable` trait
        builder.add(quote! {
            fn get_range(&self) -> std::ops::Range<usize>{
                self.range.clone()
            }

            fn get_query_index(&self) -> usize {
                self.query_index
            }

            fn get_id(&self) -> usize {
                self.id
            }
        });
        self.fn_new(&mut builder);
        self.fn_add(&mut builder);
        builder.stage_trait(
            self.input_builder_name,
            &self.paths.symbol_builder_trait.path,
        );

        // Implement `TryFromBuilder`
        self.impl_try_from(&mut builder);

        // Implement `Queryable`
        self.impl_queryable(&mut builder);

        tokens.extend(builder.to_token_stream());
    }
}

impl StructBuilder<'_> {
    fn struct_input(&self, builder: &mut FieldBuilder) {
        let symbol_data = &self.paths.symbol_data;

        builder
            .add(quote! { _data: #symbol_data })
            .add_iter(self.fields, |ty, _, name, field_type, _| match ty {
                FieldType::Normal => quote! {
                    pub #name: std::sync::Arc<#field_type>
                },
                FieldType::Vec => quote! {
                    pub #name: Vec<std::sync::Arc<#field_type>>
                },
                FieldType::Option => quote! {
                    pub #name: Option<std::sync::Arc<#field_type>>
                },
            })
            .stage_struct(self.input_name);
    }

    fn impl_ast_symbol(&self, builder: &mut FieldBuilder) {
        let get_data = &self.paths.symbol_trait.get_data.sig;
        let get_mut_data = &self.paths.symbol_trait.get_mut_data.sig;

        builder
            .add(quote! { #get_data { &self._data } })
            .add(quote! { #get_mut_data { &mut self._data } })
            .stage_trait(self.input_name, &self.paths.symbol_trait.path);
    }

    fn impl_traverse(&self, builder: &mut FieldBuilder) {
        let symbol_trait = &self.paths.symbol_trait.path;
        builder
            .add_fn_iter(
                self.fields,
                &self.paths.traverse.descendant_at.sig,
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
                self.fields,
                &self.paths.traverse.descendant_at_and_collect.sig,
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
            .add_fn_iter(self.fields, &self.paths.traverse.traverse_and_collect.sig, None,
                |_, _, name, _, _| {
                    quote! {
                        self.#name.traverse_and_collect(collect_fn, collect);
                    }
                },
            None
            )
            .stage_trait(self.input_name, &self.paths.traverse.path);
    }

    fn impl_queryable(&self, builder: &mut FieldBuilder) {
        let queryable = &self.paths.queryable.path;
        let query_name = self.query_name;

        builder
            .add(quote! { const QUERY_NAMES: &'static [&'static str] = &[#query_name]; })
            .stage_trait(self.input_builder_name, queryable);
    }

    fn impl_indented_display(&self, builder: &mut FieldBuilder) {
        let input_name = &self.input_name;
        let indented_display = &self.paths.indented_display.path;
        builder
            .add(quote! {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    use #indented_display;
                    self.fmt_with_indent(f, 0)
                }
            })
            .stage_trait(self.input_name, &self.paths.display.path)
            .add_fn_iter(
                self.fields,
                &self.paths.indented_display.fmt_with_indent.sig,
                Some(quote! {
                    use #indented_display;
                    writeln!(f, "{}{:?}", " ".repeat(indent + 2), stringify!(#input_name))?;
                }),
                |kind, _, name, type_, _| {
                     match kind {
                        FieldType::Vec =>
                            quote! {
                                writeln!(f, "{}{}: Vec<{:?}>[{}]", " ".repeat(indent + 4), stringify!(#name), stringify!(#type_), self.#name.len())?;
                                self.#name.fmt_with_indent(f, indent + 4)?;
                             },
                        FieldType::Option =>
                            quote! {
                                writeln!(f, "{}{}: Option<{:?}>[{}]", " ".repeat(indent + 4), stringify!(#name), stringify!(#type_), self.#name.is_some())?;
                                self.#name.fmt_with_indent(f, indent + 4)?;
                         },
                        _ =>
                            quote! {
                                writeln!(f, "{}{}: <{:?}>", " ".repeat(indent + 4), stringify!(#name), stringify!(#type_))?;
                                self.#name.fmt_with_indent(f, indent + 4)?;
                             }
                    }
                },
                Some(quote! { Ok(()) })
            )
            .stage_trait(self.input_name, &self.paths.indented_display.path);
    }

    fn struct_input_builder(&self, builder: &mut FieldBuilder) {
        let maybe_pending_symbol = &self.paths.maybe_pending_symbol;
        let pending_symbol = &self.paths.pending_symbol;

        builder
            .add(quote! { query_index: usize })
            .add(quote! { id: usize })
            .add(quote! { range: std::ops::Range<usize> })
            .add_iter(self.fields, |ty, _, name, _, _| match ty {
                FieldType::Vec => quote! { #name: Vec<#pending_symbol> },
                _ => quote! { #name: #maybe_pending_symbol },
            })
            .stage_struct(self.input_builder_name)
            .to_token_stream();
    }

    fn fn_new(&self, builder: &mut FieldBuilder) {
        let maybe_pending_symbol = &self.paths.maybe_pending_symbol;
        let sig = &self.paths.symbol_builder_trait.new.sig;

        let fields = FieldBuilder::default()
            .add_iter(self.fields, |ty, _, name, _, _| match ty {
                FieldType::Vec => quote! { #name: vec![] },
                _ => quote! { #name: #maybe_pending_symbol::none() },
            })
            .stage_fields()
            .to_token_stream();

        builder.add(quote! {
          #sig {
            let range = capture.node.range();
            Some(Self {
                query_index: capture.index as usize,
                id: capture.node.id(),
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
        let add_symbol_trait = &self.paths.add_symbol_trait;
        builder.add_fn_iter(
            self.fields,
            &self.paths.symbol_builder_trait.add.sig,
            Some(quote! { use #add_symbol_trait; }),
            |_, _, name, field_type, builder| {
                quote! {

                    if let Some(node) =  self.#name.add::<#builder>(capture, parsers)? {
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

        let symbol_data = &self.paths.symbol_data;
        let try_downcast = &self.paths.try_downcast_trait;
        let builder_trait = &self.paths.symbol_builder_trait.path;
        let parsers = &self.paths.parsers;

        let _builder = FieldBuilder::default()
            .add(quote! {
                use #try_downcast;
                use #builder_trait;

                let parent_id = Some(builder.get_id());
            })
            .add_iter(self.fields, |ty, _, name, field_type, _| match ty {
                FieldType::Normal => quote! {
                    let #name = builder
                        .#name
                        .try_downcast(&parent_id, document, parsers, id_map, all_nodes)?
                        .ok_or(auto_lsp::core::errors::AstError::MissingSymbol {
                            range: builder_range.clone(),
                            symbol: stringify!(#name),
                            parent_name: stringify!(#input_name),
                        })?;
                },
                _ => quote! {
                    let #name = builder
                        .#name
                        .try_downcast(&parent_id, document, parsers, id_map, all_nodes)?;
                },
            })
            .stage()
            .to_token_stream();

        builder.add(quote! {
            impl TryFrom<(
                &#input_builder_name,
                &Option<usize>,
                &auto_lsp::core::document::Document,
                &'static #parsers,
                &std::collections::HashMap<usize, usize>,
                &mut Vec<std::sync::Arc<dyn auto_lsp::core::ast::AstSymbol>>,
            )> for #input_name {
                type Error = auto_lsp::core::errors::AstError;

                fn try_from(
                    (builder, parent_id, document, parsers, id_map, all_nodes): (
                        &#input_builder_name,
                        &Option<usize>,
                        &auto_lsp::core::document::Document,
                        &'static #parsers,
                        &std::collections::HashMap<usize, usize>,
                        &mut Vec<std::sync::Arc<dyn auto_lsp::core::ast::AstSymbol>>,
                    )
                ) -> Result<Self, Self::Error> {
                    let builder_range = builder.get_range();

                    #_builder

                    Ok(#input_name {
                        _data: #symbol_data::new(builder_range),
                        #(#fields),*
                    })
                }
            }
        });
        builder.stage();
    }
}
