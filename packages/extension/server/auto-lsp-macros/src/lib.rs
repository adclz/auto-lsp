#![allow(deprecated)]

extern crate proc_macro;

use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};

mod enum_builder;
mod feature_builder;
mod features;
mod meta;
mod paths;
mod struct_builder;
mod utilities;

use enum_builder::*;
use feature_builder::*;
use paths::*;
use struct_builder::*;

use crate::meta::*;
use crate::utilities::extract_fields::{match_enum_fields, match_struct_fields};

use std::cell::LazyCell;

trait BuildAstItem {
    fn generate_fields(&self) -> Vec<proc_macro2::TokenStream>;
    fn generate_ast_item_methods(&self) -> proc_macro2::TokenStream;
}

trait BuildAstItemBuilder {
    fn generate_builder_fields(&self) -> Vec<proc_macro2::TokenStream>;
    fn generate_builder_new(&self) -> proc_macro2::TokenStream;
    fn generate_query_binder(&self) -> proc_macro2::TokenStream;
    fn generate_add(&self) -> proc_macro2::TokenStream;
    fn generate_try_from(&self) -> proc_macro2::TokenStream;
}

const PATHS: LazyCell<Paths> = LazyCell::new(|| Paths::default());

#[proc_macro_attribute]
pub fn ast_struct(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => return e.into_compile_error().into(),
    };

    let args = match AstStruct::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };

    let input = parse_macro_input!(input as DeriveInput);

    let input_name = &input.ident;
    let input_builder_name = format_ident!("{}Builder", input_name);

    let fields = match_struct_fields(&input.data);
    let query_name = args.query_name;
    let mut tokens = proc_macro2::TokenStream::new();

    match args.kind {
        AstStructKind::Accessor => StructBuilder::new(
            None,
            &input_name,
            &input_builder_name,
            &query_name,
            &fields,
            &*PATHS,
            true,
        )
        .to_tokens(&mut tokens),
        AstStructKind::Symbol(symbol_features) => StructBuilder::new(
            Some(&symbol_features),
            &input_name,
            &input_builder_name,
            &query_name,
            &fields,
            &*PATHS,
            false,
        )
        .to_tokens(&mut tokens),
    };

    tokens.into()
}

#[proc_macro_attribute]
pub fn ast_enum(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let input_name = &input.ident;
    let input_builder_name = format_ident!("{}Builder", input_name);
    let fields = match_enum_fields(&input.data);
    let mut tokens = proc_macro2::TokenStream::new();

    EnumBuilder::new(&input_name, &input_builder_name, &fields, &*PATHS).to_tokens(&mut tokens);
    tokens.into()
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
            pub fn query_binder(url: Arc<lsp_types::Url>, capture: &tree_sitter::QueryCapture, query: &tree_sitter::Query) -> Option<std::rc::Rc<std::cell::RefCell<dyn auto_lsp::traits::ast_item_builder::AstItemBuilder>>> {
                let query_name = query.capture_names()[capture.index as usize];
                #(
                    if let true = #variant_names::QUERY_NAMES.contains(&query_name)  {
                            return Some(std::rc::Rc::new(std::cell::RefCell::new(#variant_builder_names::new(
                                url,
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
                            let item: #variant_types_names = b.clone().try_into()?;
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

#[proc_macro_derive(KeySet, attributes(key))]
pub fn derive_helper_attr(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let struct_name = &input.ident;

    // Ensure the input is a struct
    let data_struct = match input.data {
        syn::Data::Struct(ref data) => data,
        _ => {
            return syn::Error::new_spanned(input, "Expected a struct")
                .to_compile_error()
                .into()
        }
    };

    // Find the field with the 'key' attribute
    let key_field = data_struct
        .fields
        .iter()
        .find(|field| field.attrs.iter().any(|attr| attr.path().is_ident("key")));

    let key_field = match key_field {
        Some(field) => field,
        None => {
            return syn::Error::new_spanned(&input.ident, "Expected a field with #[key] attribute")
                .to_compile_error()
                .into()
        }
    };

    // Get the field name
    let key_field_ident = match &key_field.ident {
        Some(ident) => ident,
        None => {
            return syn::Error::new_spanned(
                &key_field,
                "Expected a named field with #[key] attribute",
            )
            .to_compile_error()
            .into()
        }
    };

    let builder = format_ident!("{}Builder", struct_name);

    // Generate the implementation
    let expanded = quote! {
        impl auto_lsp::traits::key::Key for #builder {
            fn get_key<'a>(&self, source_code: &'a [u8]) -> &'a str {
                self.#key_field_ident.as_ref().expect(&format!("Key {} is not present on {}", stringify!(#key_field_ident), stringify!(#builder))).borrow().get_text(source_code)
            }
        }
    };

    TokenStream::from(expanded)
}
