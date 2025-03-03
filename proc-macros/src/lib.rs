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

/// A procedural macro for generating an AST symbol from a struct.
///
/// ## Basic usage
///
/// ```ignore
/// #[seq(query = "query_name")]
/// struct MyStruct {}
/// ```
///
/// ## Attributes
///
/// - `query_name`: The name of the Tree-sitter query associated with this struct.
///
/// ### LSP attributes
///
/// - `code_actions`: [`BuildCodeActions`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.GetHover.html) trait.
/// - `code_lenses`: [`BuildCodeLenses`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.BuildCodeLenses.html) trait.
/// - `completions`:[`BuildCompletionItems`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.BuildCompletionItems.html) trait.
/// - `declaration`:[`GetGoToDeclaration`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.GetGoToDeclaration.html) trait.
/// - `definition`: [`GetGoToDefinition`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.GetGoToDefinition.html) trait.
/// - `document_symbols`: [`BuildDocumentSymbols`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.BuildDocumentSymbols.html) trait.
/// - `hover`: [`GetHover`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.GetHover.html) trait.
/// - `inlay_hints`: [`BuildInlayHints`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.BuildInlayHints.html) trait.
/// - `invoked_completions`:[`BuildInvokedCompletionItems`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.BuildInvokedCompletionItems.html) trait.
/// - `semantic_tokens`: [`BuildSemanticTokens`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.BuildSemanticTokens.html) trait.
///
/// ### Special attributes
///
/// - `comment`: mark this node as a node that can potentially contain a comment.
/// - `check`: [`Check`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.Check.html) trait.
///
/// ```ignore
/// #[seq(query = "query_name", document_symbols)]
/// struct MyStruct {}
///
/// impl BuildDocumentSymbols for Module {
///    fn build_document_symbols(&self, doc: &Document)  {
///        /* ... */
///    }
/// }
///
/// #[seq(query = "query_name2", symbols)]
/// struct MyStruct2 {}
///
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
            &Paths::default(),
            &attributes,
            &input_attr,
            input_name,
            &input_builder_name,
            query_name,
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

    EnumBuilder::new(&Paths::default(), input_name, &input_builder_name, &fields)
        .to_tokens(&mut tokens);
    tokens.into()
}
