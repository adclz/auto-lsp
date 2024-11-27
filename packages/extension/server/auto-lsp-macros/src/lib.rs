#![allow(deprecated)]

extern crate proc_macro;

use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DataStruct, DeriveInput};

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

    let input_attr = input.attrs;
    match args.kind {
        AstStructKind::Accessor => StructBuilder::new(
            None,
            &input_attr,
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
            &input_attr,
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

    let builder = format_ident!("{}Builder", struct_name);

    TokenStream::from(match get_key_helper(&data_struct) {
        None => quote! {
            impl auto_lsp::traits::key::Key for #builder {
                fn get_key<'a>(&self, source_code: &'a [u8]) -> &'a str {
                    self.get_text(source_code)
                }
            }
        },
        Some(key_field_ident) => quote! {
            impl auto_lsp::traits::key::Key for #builder {
                fn get_key<'a>(&self, source_code: &'a [u8]) -> &'a str {
                    self.#key_field_ident.as_ref().expect(&format!("Key {} is not present on {}", stringify!(#key_field_ident), stringify!(#builder))).get_rc().borrow().get_text(source_code)
                }
            }
        },
    })
}

fn get_key_helper<'a>(data_struct: &'a DataStruct) -> Option<&'a syn::Ident> {
    // Find the field with the 'key' attribute
    let key_field = data_struct
        .fields
        .iter()
        .find(|field| field.attrs.iter().any(|attr| attr.path().is_ident("key")));

    let key_field = match key_field {
        Some(field) => field,
        None => return None,
    };

    // Get the field name
    match &key_field.ident {
        Some(ident) => Some(ident),
        None => return None,
    }
}
