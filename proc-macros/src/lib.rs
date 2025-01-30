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

/// A procedural macro for generating an AST symbol from a struct.
///
/// ## Basic usage
///
/// ```ignore
/// #[seq(query = "query_name"]
/// struct MyStruct {}
/// ```
///
/// ## Attributes
///
/// - `query_name`: The name of the Tree-sitter query associated with this struct.
/// - `kind`: Specifies the type of symbol to generate, which can be either `symbol` or `reference`.
///
/// ### symbol
///
/// When the `kind` attribute is set to `symbol`, the generated symbol will implement the `AstSymbol` trait.
///
/// As a result, all capabilities traits are implemented by default, but users can override them using nested attributes.
///
/// All nested attributes are optional. If an attribute is set, it allows the user to override the default implementation,
/// either by providing a custom implementation of the trait (using `user`) or replacing the default implementation with code generation (using code_gen when available).
///
/// ```ignore
/// // When using `user`, the default trait implementation is removed.
///
/// #[seq(query = "query_name", document_symbols]
/// struct MyStruct {}
///
/// impl BuildDocumentSymbols for Module {
///    fn get_document_symbols(&self, doc: &Document) -> Option<VecOrSymbol> {
///        /* ... */
///    }
/// }
///
/// // With `codegen`, the default implementation is replaced by the code_gen implementation
///
/// #[seq(query = "query_name2", kind(symbol(
/// lsp_document_symbols(
///    code_gen(
///        name = self::name,
///        kind = auto_lsp::lsp_types::SymbolKind::FUNCTION,
///    )
/// ))))]
/// struct MyStruct2 {}
///
/// ```
///
/// ```
///
#[proc_macro_attribute]
pub fn seq(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse args

    let attr_meta = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => return e.into_compile_error().into(),
    };

    let attributes = match DarlingInput::from_list(&attr_meta) {
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
    let query_name = &attributes.query;

    let input_attr = input.attrs;
    TokenStream::from(
        StructBuilder::new(
            &attributes,
            &input_attr,
            &input_name,
            &input_builder_name,
            &query_name,
            &fields,
        )
        .to_token_stream(),
    )
}

/// A procedural macro for generating an AST symbol from an enum.
///
/// ## Basic usage
///
/// ```ignore
/// #[choice]
/// enum MyEnum {
///     Variant1(Variant)
/// }
/// ```
///
/// The `choice` macro does not accept any attributes. It invokes the AstSymbol implementation of the inner variant.
///
/// This macro functions similarly to `enum_dispatch` but is tailored for the specific needs of `auto_lsp`.
///
/// However, every variant of the enum **has to be** a struct or enum that implements the `AstSymbol` trait.
///
/// This means that all variants must be a unique symbol, and therefore a `Vec` or Option of `AstSymbol` can't be used.
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
