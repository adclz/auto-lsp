use crate::utilities::extract_fields::EnumFields;
use crate::FeaturesCodeGen;
use constcat::concat_slices;
use darling::FromMeta;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use std::collections::HashSet;

pub fn generate_enum_ast_item(input: &EnumFields) -> FeaturesCodeGen {
    let variant_names = &input.variant_names;
    let variant_types = &input.variant_types_names;

    FeaturesCodeGen {
        fields: Some(vec![
        ]),
        impl_base: Some(quote! {
            pub const QUERY_NAMES: &[&str] = constcat::concat_slices!([&str]: #( &#variant_types::QUERY_NAMES ),*);
        }),
        impl_ast_item: Some(
            quote! {
                    fn get_range(&self) -> tree_sitter::Range {
                        match self {
                            #(
                                Self::#variant_names(variant) => variant.get_range(),
                            )*
                        }
                    }

                    fn get_parent(&self) -> Option<std::sync::Arc<std::sync::RwLock<dyn AstItem>>> {
                        match self {
                            #(
                                Self::#variant_names(variant) => variant.get_parent(),
                            )*
                        }
                    }

                    fn set_parent(&mut self, parent: std::sync::Arc<std::sync::RwLock<dyn AstItem>>) {
                        match self {
                            #(
                                Self::#variant_names(variant) => variant.set_parent(parent),
                            )*
                        }
                    }

                    fn inject_parent(&mut self, parent: std::sync::Arc<std::sync::RwLock<dyn AstItem>>) {
                        match self {
                            #(
                                Self::#variant_names(variant) => variant.inject_parent(parent),
                            )*
                        }
                    }

                    fn find_at_offset(&self, offset: &usize) -> Option<std::sync::Arc<std::sync::RwLock<dyn AstItem>>> {
                        match self {
                            #(
                                Self::#variant_names(variant) => variant.find_at_offset(offset),
                            )*
                        }
                    }

                    fn get_start_position(&self, doc: &lsp_textdocument::FullTextDocument) -> lsp_types::Position {
                        match self {
                            #(
                                Self::#variant_names(variant) => variant.get_start_position(doc),
                            )*
                        }
                    }

                    fn get_end_position(&self, doc: &lsp_textdocument::FullTextDocument) -> lsp_types::Position {
                        match self {
                            #(
                                Self::#variant_names(variant) => variant.get_end_position(doc),
                            )*
                        }
                    }

                    fn is_borrowable(&self, other: &dyn AstItem) -> bool {
                        match self {
                            #(
                                Self::#variant_names(variant) => variant.is_borrowable(other),
                            )*
                        }
                    }


                    // LSP
                    fn get_document_symbols(&self, doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::DocumentSymbol> {

                        match self {
                            #(
                                Self::#variant_names(variant) => variant.get_document_symbols(doc),
                            )*
                        }
                    }

                    fn get_hover(&self, doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::Hover> {
                        match self {
                            #(
                                Self::#variant_names(variant) => variant.get_hover(doc),
                            )*
                        }
                    }

                    fn build_semantic_tokens(&self, builder: &mut auto_lsp::builders::semantic_tokens::SemanticTokensBuilder) {
                        match self {
                            #(
                                Self::#variant_names(variant) => variant.build_semantic_tokens(builder),
                            )*
                        }
                    }

                    fn build_inlay_hint(&self, acc: &mut Vec<lsp_types::InlayHint>) {
                        match self {
                            #(
                                Self::#variant_names(variant) => variant.build_inlay_hint(acc),
                            )*
                        }
                    }

                    fn build_code_lens(&self, acc: &mut Vec<lsp_types::CodeLens>) {
                        match self {
                            #(
                                Self::#variant_names(variant) => variant.build_code_lens(acc),
                            )*
                        }
                    }

                    fn swap_at_offset(&mut self, offset: &usize, item: &std::rc::Rc<std::cell::RefCell<dyn AstItemBuilder>>) {
                        match self {
                            #(
                                Self::#variant_names(variant) => variant.swap_at_offset(offset, &item),
                            )*
                        }
                    }
                }
            .into(),
        ),
    }
}
