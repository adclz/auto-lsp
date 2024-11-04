#![allow(deprecated)]

extern crate proc_macro;

use darling::{
    ast::{Data, NestedMeta},
    FromAttributes, FromDeriveInput, FromField, FromMeta,
};
use features::{
    lsp_code_lens::generate_code_lens_feature,
    lsp_completion_item::generate_completion_item_feature,
    lsp_inlay_hint::generate_inlay_hint_feature,
};
use proc_macro::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{meta::ParseNestedMeta, parse::Parser, ItemEnum, Macro, Type, TypeMacro, TypePath};
use syn::{parse_macro_input, token::Group, DeriveInput};
use syn::{Error, Expr, Field, Fields, FieldsNamed, Lit, LitStr, Path};

use traits::ast_item::for_enum::generate_enum_ast_item;
use utilities::{
    extract_fields::match_enum_fields,
    filter::{get_raw_type_name, is_hashmap, is_option, is_vec},
};

mod features;
mod meta;
mod traits;
mod utilities;
use crate::features::borrowable::*;
use crate::features::lsp_document_symbol::*;
use crate::features::lsp_hover_info::*;
use crate::features::lsp_semantic_token::*;
use crate::meta::*;
use crate::traits::ast_builder::{
    for_enum::generate_enum_builder_item, for_struct::generate_struct_builder_item,
};
use crate::traits::ast_item::for_struct::generate_struct_ast_item;
use crate::utilities::extract_fields::match_fields;

struct FeaturesCodeGen {
    fields: Option<Vec<proc_macro2::TokenStream>>, // Fields
    impl_base: Option<proc_macro2::TokenStream>,   // Impl <>
    impl_ast_item: Option<proc_macro2::TokenStream>, // Impl AstItem for <>
}

#[proc_macro_attribute]
pub fn ast_struct(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => return e.into_compile_error().into(),
    };

    let _args = match SymbolArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };

    let mut input = parse_macro_input!(input as DeriveInput);
    let input_name = input.ident.clone();

    let fields_sort = match_fields(&input.data);

    let mut code_gen_fields: Vec<proc_macro2::TokenStream> = vec![];
    let mut code_gen_impl: Vec<proc_macro2::TokenStream> = vec![];
    let mut code_gen_impl_ast_item: Vec<proc_macro2::TokenStream> = vec![];

    // Add AstItem trait implementation
    let code_gen = generate_struct_ast_item(_args.query_name.as_str(), &fields_sort);

    // Add builder item
    let builder_item = generate_struct_builder_item(input_name.to_string().as_str(), &fields_sort);

    code_gen_fields.append(&mut code_gen.fields.unwrap());
    code_gen_impl.push(code_gen.impl_base.unwrap());
    code_gen_impl_ast_item.push(code_gen.impl_ast_item.unwrap());

    if let Some(features) = _args.features {
        generate_document_symbol_feature(
            &features,
            &mut code_gen_impl,
            &mut code_gen_impl_ast_item,
        );
        generate_hover_info_feature(&features, &mut code_gen_impl, &mut code_gen_impl_ast_item);
        generate_semantic_token_feature(
            &features,
            &mut code_gen_impl,
            &mut code_gen_impl_ast_item,
            &fields_sort,
        );
        generate_inlay_hint_feature(
            &features,
            &mut code_gen_impl,
            &mut code_gen_impl_ast_item,
            &fields_sort,
        );
        generate_code_lens_feature(
            &features,
            &mut code_gen_impl,
            &mut code_gen_impl_ast_item,
            &fields_sort,
        );
        generate_completion_item_feature(
            &features,
            &mut code_gen_impl,
            &mut code_gen_impl_ast_item,
        );
        generate_borrowable_feature(&features, &mut code_gen_impl, &mut code_gen_impl_ast_item);
    }

    // Fields cannot be generated from the quote! macro, so we need to manually add them
    match &mut input.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    // Transform each field's type to Arc<RwLock<OriginalType>>
                    for field in fields.named.iter_mut() {
                        let raw_type_name = format_ident!("{}", get_raw_type_name(&field.ty));
                        let name = field.ident.clone();

                        *field =
                            if let true = is_vec(&field.ty) {
                                syn::Field::parse_named
                                    .parse2(quote! { #name: Vec<Arc<RwLock<#raw_type_name>>> })
                                    .unwrap()
                            } else if let true = is_option(&field.ty) {
                                syn::Field::parse_named
                                    .parse2(quote! { #name: Option<Arc<RwLock<#raw_type_name>>> })
                                    .unwrap()
                            } else if let true = is_hashmap(&field.ty) {
                                syn::Field::parse_named
                            .parse2(quote! { #name: HashMap<String, Arc<RwLock<#raw_type_name>>> })
                            .unwrap()
                            } else {
                                syn::Field::parse_named
                                    .parse2(quote! { #name: Arc<RwLock<#raw_type_name>> })
                                    .unwrap()
                            };
                    }

                    for field in code_gen_fields {
                        fields
                            .named
                            .push(syn::Field::parse_named.parse2(field).unwrap());
                    }
                    fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote! { parent: Option<Arc<RwLock<dyn AstItem>>> })
                            .unwrap(),
                    );
                }
                _ => (),
            }
        }
        _ => panic!("This proc macro only works with struct"),
    };

    TokenStream::from(quote! {
        #[derive(Clone)]
        #input

        impl #input_name {
            #(#code_gen_impl)*
        }

        impl AstItem for #input_name {
            #(#code_gen_impl_ast_item)*
        }

        #builder_item
    })
}

#[proc_macro_attribute]
pub fn ast_enum(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as syn::Item);

    let enum_item = match &input {
        syn::Item::Enum(item_enum) => item_enum,
        _ => {
            return syn::Error::new_spanned(input, "This macro only works with enums")
                .to_compile_error()
                .into();
        }
    };

    let enum_name = enum_item.ident.clone();

    let fields_sort = match_enum_fields(&enum_item);

    let mut code_gen_fields: Vec<proc_macro2::TokenStream> = vec![];
    let mut code_gen_impl: Vec<proc_macro2::TokenStream> = vec![];
    let mut code_gen_impl_ast_item: Vec<proc_macro2::TokenStream> = vec![];

    // Add AstItem trait implementation
    let code_gen = generate_enum_ast_item(&fields_sort);

    code_gen_fields.append(&mut code_gen.fields.unwrap());
    code_gen_impl.push(code_gen.impl_base.unwrap());
    code_gen_impl_ast_item.push(code_gen.impl_ast_item.unwrap());

    // Add builder item
    let builder_item = generate_enum_builder_item(enum_name.to_string().as_str(), &fields_sort);

    TokenStream::from(quote! {
        #[derive(Clone)]
        #input

        impl #enum_name {
            #(#code_gen_impl)*
        }

        impl AstItem for #enum_name {
            #(#code_gen_impl_ast_item)*
        }

        #builder_item
    })
}

#[proc_macro_attribute]
pub fn ast(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as syn::Item);

    // Ensure the input is an enum
    let mut enum_item = match input {
        syn::Item::Enum(item_enum) => item_enum,
        _ => {
            return syn::Error::new_spanned(input, "This macro only works with enums")
                .to_compile_error()
                .into();
        }
    };

    // Generate a custom function for each enum variant
    let enum_name = &enum_item.ident.clone();

    let variant_names: Vec<_> = enum_item
        .variants
        .iter()
        .map(|variant| variant.ident.clone())
        .collect();

    for variant in &mut enum_item.variants {
        let variant_name = &variant.ident;

        *variant = syn::parse_quote! {
            #variant_name(Arc<RwLock<#variant_name>>)
        };
    }

    let variant_types_names: Vec<_> = variant_names
        .iter()
        .map(|field_type| format_ident!("{}", field_type))
        .collect();

    let variant_builder_names: Vec<_> = variant_names
        .iter()
        .map(|field_type| format_ident!("{}Builder", field_type))
        .collect();

    // Generate the output tokens
    let expanded = quote! {
        #[derive(Clone)]
        #enum_item

        impl #enum_name {
            pub fn query_binder(capture: &tree_sitter::QueryCapture, query: &tree_sitter::Query) -> Option<std::rc::Rc<std::cell::RefCell<dyn auto_lsp::traits::ast_item_builder::AstItemBuilder>>> {
                let query_name = query.capture_names()[capture.index as usize];
                #(
                    if let true = #variant_names::QUERY_NAMES.contains(&query_name)  {
                            return Some(std::rc::Rc::new(std::cell::RefCell::new(#variant_builder_names::new(
                                &query,
                                capture.index as usize,
                                capture.node.range(),
                                capture.node.start_position(),
                                capture.node.end_position(),
                            ))))
                    };
                )*
                None
            }

            pub fn builder_binder(roots: Vec<std::rc::Rc<std::cell::RefCell<dyn AstItemBuilder>>>) -> Vec<Result<std::sync::Arc<std::sync::RwLock<dyn AstItem>>, lsp_types::Diagnostic>> {
                roots
                .into_iter()
                .map(|builder| {
                    #(
                        // note: this part could be improved by moving the inner value from Rc
                        // but: https://stackoverflow.com/questions/41618100/rctrait-to-optiont
                        if let Some(b) = builder.borrow().downcast_ref::<#variant_builder_names>() {
                            let item: #variant_types_names = b.clone().try_into().expect("Incomplete builder");
                            let item: Arc<RwLock<dyn AstItem>> = Arc::new(RwLock::new(item));
                            Ok(item)
                        }
                    )else *
                    else {
                        panic!("Unknown builder type")
                    }
                }).collect()
            }
         }
    };

    // Return the generated tokens
    TokenStream::from(expanded)
}
