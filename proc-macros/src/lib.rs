#![allow(deprecated)]

extern crate proc_macro;

use darling::FromDeriveInput;
use darling::{ast::NestedMeta, FromMeta};
use enum_builder::EnumBuilder;
use field_builder::extract_fields;
use meta::*;
use paths::*;
use proc_macro::TokenStream;
use quote::{format_ident, ToTokens};
use r#enum::*;
use r#struct::*;
use struct_builder::StructBuilder;
use syn::{parse_macro_input, DeriveInput};
use variant_builder::extract_variants;

mod r#enum;
mod meta;
mod paths;
mod r#struct;
mod utilities;

use std::cell::LazyCell;

/// Paths of every structs, enums, traits and functions from core crate
const PATHS: LazyCell<Paths> = LazyCell::new(|| Paths::default());

/// Procedural macro for generating an AstSymbol from a struct
///
/// ## Basic usage
/// ```ignore
/// #[seq(query_name = "query_name", kind(symbol()))]
/// struct MyStruct {}
/// ```
///
/// ## Attributes
///
/// - `query_name`: The name of the tree sitter query this struct will be associated with
/// - `kind`: The kind of struct to generate. Can be either `symbol` or `reference`
///
/// ### symbol
///
/// When the `kind` attribute is set to `symbol`, the generated struct will implement the `AstSymbol` trait,
/// therefore, all LSP traits are implemented by default but can be overriden by the user using the nested attributes.
///
/// All nested attributes are optional, when an attribute is set it offers the user the ability to override the default implementation,
/// either by providing a custom implementation of the trait (with `user`) or code_gen (with `codegen`).
/// ```ignore
/// // With `user`, default trait implementation is removed
///
/// #[seq(query_name = "query_name", kind(symbol(
///     lsp_document_symbols(user),
/// )))]
/// struct MyStruct {}
///
/// impl BuildDocumentSymbols for Module {
///    fn get_document_symbols(&self, doc: &Document) -> Option<VecOrSymbol> {
///        /* ... */
///    }
///}
///
/// // With `codegen`, the default implementation is replaced by the code_gen implementation
///
/// #[seq(query_name = "query_name2", kind(symbol(
/// lsp_document_symbols(
///    code_gen(
///        name = self::name,
///        kind = auto_lsp::lsp_types::SymbolKind::FUNCTION,
///    )
///),
/// )))]
/// struct MyStruct2 {}
///
/// ```
///
/// ### reference
///
/// When the `kind` attribute is set to `reference`, the generated struct implements `AstSymbol` as well, but
///
#[proc_macro_attribute]
pub fn seq(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse args

    let attr_meta = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => return e.into_compile_error().into(),
    };

    let attributes = match UserFeatures::from_list(&attr_meta) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };

    // Parse input

    let input: DeriveInput = syn::parse_macro_input!(input);

    let derive_input = match StructInput::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    if !derive_input.data.is_struct() {
        return syn::Error::new_spanned(input, "Expected a struct")
            .to_compile_error()
            .into();
    }

    let input_name = &input.ident;
    let input_builder_name = format_ident!("{}Builder", input_name);

    let fields = extract_fields(&derive_input.data);
    let query_name = attributes.query_name;

    let input_attr = input.attrs;
    let tokens = match attributes.kind {
        AstStructKind::Reference(accessor_features) => StructBuilder::new(
            &ReferenceOrSymbolFeatures::Reference(&accessor_features),
            &derive_input.data,
            &input_attr,
            &input_name,
            &input_builder_name,
            &query_name,
            &fields,
        )
        .to_token_stream(),
        AstStructKind::Symbol(symbol_features) => StructBuilder::new(
            &ReferenceOrSymbolFeatures::Symbol(&symbol_features),
            &derive_input.data,
            &input_attr,
            &input_name,
            &input_builder_name,
            &query_name,
            &fields,
        )
        .to_token_stream(),
    };

    TokenStream::from(tokens)
}

/// Procedural macro for generating an AstSymbol from an enum
///
/// ## Basic usage
/// ```ignore
/// #[choice]
/// enum MyEnum {
///     Variant1(Variant)
/// }
/// ```
///
/// `choice` does not have any attributes, it generates an enum that implements the `AstSymbol` trait.
///
/// However, every variant of the enum **must** implement the `AstSymbol` trait.
///
#[proc_macro_attribute]
pub fn choice(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let input_name = &input.ident;
    let input_builder_name = format_ident!("{}Builder", input_name);
    let fields = extract_variants(&input.data);
    let mut tokens = proc_macro2::TokenStream::new();

    EnumBuilder::new(&input_name, &input_builder_name, &fields).to_tokens(&mut tokens);
    tokens.into()
}
