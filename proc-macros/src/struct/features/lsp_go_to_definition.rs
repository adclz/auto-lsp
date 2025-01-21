extern crate proc_macro;

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::feature_builder::FeaturesCodeGen;
use crate::fields_builder::Fields;
use crate::{ReferenceFeature, ReferenceFeatures, SymbolFeatures, PATHS};

use crate::Feature;

#[derive(Debug, FromMeta)]
pub struct GotoDefinitionFeature {}

pub struct GotoDefinitionBuilder<'a> {
    pub input_name: &'a Ident,
    pub fields: &'a Fields,
}

impl<'a> GotoDefinitionBuilder<'a> {
    pub fn new(input_name: &'a Ident, fields: &'a Fields) -> Self {
        Self { input_name, fields }
    }

    pub fn default_impl(&self) -> TokenStream {
        let input_name = &self.input_name;
        let go_to_definitions_path = &PATHS.lsp_go_to_definition.path;

        quote! {
            impl #go_to_definitions_path for #input_name { }
        }
    }
}

impl<'a> FeaturesCodeGen for GotoDefinitionBuilder<'a> {
    fn code_gen(&self, params: &SymbolFeatures) -> impl quote::ToTokens {
        match &params.lsp_go_to_definition {
            None => self.default_impl(),
            Some(params) => match params {
                Feature::User => quote! {},
                Feature::CodeGen(_) => {
                    panic!("Go to Definition does not provide code generation, instead implement the trait GetGoToDefinition manually");
                }
            },
        }
    }

    fn code_gen_accessor(&self, params: &ReferenceFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let go_to_definitions_path = &PATHS.lsp_go_to_definition.path;
        let sig = &PATHS.lsp_go_to_definition.go_to_definition.sig;

        match &params.lsp_go_to_definition {
            None => self.default_impl(),
            Some(params) => match params {
                ReferenceFeature::Disable => self.default_impl(),
                ReferenceFeature::Reference => {
                    quote! {
                        impl #go_to_definitions_path for #input_name {
                            #sig {
                                if let Some(accessor) = &self.get_target() {
                                    if let Some(accessor) = accessor.to_dyn() {
                                        let read = accessor.read();
                                        return Some(auto_lsp::lsp_types::GotoDefinitionResponse::Scalar(auto_lsp::lsp_types::Location {
                                            uri: (*read.get_url()).clone(),
                                            range: auto_lsp::lsp_types::Range {
                                                start: read.get_start_position(doc),
                                                end: read.get_end_position(doc),
                                            },
                                        }))
                                    }
                                }
                                None
                            }
                        }
                    }
                }
                ReferenceFeature::User => quote! {},
            },
        }
    }
}
