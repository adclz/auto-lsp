extern crate proc_macro;

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Path};

use crate::{
    utilities::{extract_fields::StructFields, format_tokens::path_to_dot_tokens},
    AccessorFeatures, FeaturesCodeGen, ReferenceFeature, SymbolFeatures, PATHS,
};

use crate::Feature;

#[derive(Debug, FromMeta)]
pub struct CompletionItemFeature {
    item: CompletionItem,
}

#[derive(Debug, FromMeta)]
pub struct CompletionItem {
    label: Path,
    kind: Path,
}

pub struct CompletionItemsBuilder<'a> {
    pub input_name: &'a Ident,
    pub fields: &'a StructFields,
}

impl<'a> CompletionItemsBuilder<'a> {
    pub fn new(input_name: &'a Ident, fields: &'a StructFields) -> Self {
        Self { input_name, fields }
    }

    pub fn default_impl(&self) -> TokenStream {
        let input_name = &self.input_name;
        let completion_items_path = &PATHS.lsp_completion_items.path;
        let sig = &PATHS
            .lsp_completion_items
            .methods
            .build_completion_items
            .sig;
        let default = &PATHS
            .lsp_completion_items
            .methods
            .build_completion_items
            .default;

        quote! {
            impl #completion_items_path for #input_name {
                #sig { #default }
            }
        }
    }
}

impl<'a> FeaturesCodeGen for CompletionItemsBuilder<'a> {
    fn code_gen(&self, params: &SymbolFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let completion_items_path = &PATHS.lsp_completion_items.path;
        let sig = &PATHS
            .lsp_completion_items
            .methods
            .build_completion_items
            .sig;

        match &params.lsp_completion_items {
            None => self.default_impl(),
            Some(params) => match params {
                Feature::User => quote! {},
                Feature::CodeGen(completion) => {
                    let item = &completion.item;
                    let kind = &item.kind;
                    let label = path_to_dot_tokens(&item.label, None);

                    quote! {
                        impl #completion_items_path for #input_name {
                            #sig {
                                let read = #label.read();

                                acc.push(lsp_types::CompletionItem {
                                    label: read.get_text(doc.get_content(None).as_bytes()).to_string(),
                                    kind: Some(#kind),
                                    detail: None,
                                    ..Default::default()
                                });
                            }
                        }
                    }
                }
            },
        }
    }

    fn code_gen_accessor(&self, params: &AccessorFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let completion_items_path = &PATHS.lsp_completion_items.path;
        let sig = &PATHS
            .lsp_completion_items
            .methods
            .build_completion_items
            .sig;

        match &params.lsp_completion_items {
            None => self.default_impl(),
            Some(params) => match params {
                ReferenceFeature::Disable => self.default_impl(),
                ReferenceFeature::Reference => {
                    quote! {
                        impl #completion_items_path for #input_name {
                            #sig {
                                if let Some(accessor) = &self.get_target() {
                                    if let Some(accessor) = accessor.to_dyn() {
                                        accessor.read().build_completion_items(doc, acc)
                                    }
                                }
                            }
                        }
                    }
                }
                ReferenceFeature::User => quote! {},
            },
        }
    }
}
