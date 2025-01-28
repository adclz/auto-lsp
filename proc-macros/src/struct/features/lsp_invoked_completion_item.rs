extern crate proc_macro;

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Path};

use crate::field_builder::Fields;
use crate::utilities::path_to_dot_tokens;
use crate::{
    r#struct::feature_builder::FeaturesCodeGen, ReferenceFeature, ReferenceFeatures,
    SymbolFeatures, PATHS,
};

use crate::Feature;

#[derive(Debug, FromMeta)]
pub struct InvokedCompletionItemFeature {}

pub struct InvokedCompletionItemsBuilder<'a> {
    pub input_name: &'a Ident,
    pub fields: &'a Fields,
}

impl<'a> InvokedCompletionItemsBuilder<'a> {
    pub fn new(input_name: &'a Ident, fields: &'a Fields) -> Self {
        Self { input_name, fields }
    }

    pub fn default_impl(&self) -> TokenStream {
        let input_name = &self.input_name;
        let invoked_completion_items_path = &PATHS.lsp_invoked_completion_items.path;

        quote! {
            impl #invoked_completion_items_path for #input_name { }
        }
    }
}

impl<'a> FeaturesCodeGen for InvokedCompletionItemsBuilder<'a> {
    fn code_gen(&self, params: &SymbolFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let invoked_completion_items_path = &PATHS.lsp_invoked_completion_items.path;
        let sig = &PATHS
            .lsp_invoked_completion_items
            .build_invoked_completion_items
            .sig;

        match &params.lsp_invoked_completion_items {
            None => self.default_impl(),
            Some(params) => match params {
                Feature::User => quote! {},
                Feature::CodeGen(_) => {
                    panic!("Invoked Completion Item does not provide code generation, instead implement the trait BuildInvokedCOmpletionItems manually");
                }
            },
        }
    }

    fn code_gen_reference(&self, params: &ReferenceFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let invoked_completion_items_path = &PATHS.lsp_invoked_completion_items.path;
        let sig = &PATHS
            .lsp_invoked_completion_items
            .build_invoked_completion_items
            .sig;

        match &params.lsp_completion_items {
            None => self.default_impl(),
            Some(params) => match params {
                ReferenceFeature::Disable => self.default_impl(),
                ReferenceFeature::Reference => {
                    quote! {
                        impl #invoked_completion_items_path for #input_name {
                            #sig {
                                if let Some(reference) = &self.get_target() {
                                    if let Some(reference) = reference.to_dyn() {
                                        reference.read().build_invoked_completion_items(doc, acc)
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
