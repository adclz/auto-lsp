extern crate proc_macro;

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{
    utilities::extract_fields::StructFields, AccessorFeatures, FeaturesCodeGen, ReferenceFeature,
    SymbolFeatures, PATHS,
};

use crate::Feature;

#[derive(Debug, FromMeta)]
pub struct GotoDefinitionFeature {}

pub struct GotoDefinitionBuilder<'a> {
    pub input_name: &'a Ident,
    pub fields: &'a StructFields,
}

impl<'a> GotoDefinitionBuilder<'a> {
    pub fn new(input_name: &'a Ident, fields: &'a StructFields) -> Self {
        Self { input_name, fields }
    }

    pub fn default_impl(&self) -> TokenStream {
        let input_name = &self.input_name;
        let go_to_definitions_path = &PATHS.lsp_go_to_definition.path;
        let sig = &PATHS.lsp_go_to_definition.methods.go_to_definition.sig;
        let default = &PATHS.lsp_go_to_definition.methods.go_to_definition.default;

        quote! {
            impl #go_to_definitions_path for #input_name {
                #sig { #default }
            }
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
                    panic!("Go to Definition does not provide code generation, instead implement the trait GoToDefinition manually");
                }
            },
        }
    }

    fn code_gen_accessor(&self, params: &AccessorFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let go_to_definitions_path = &PATHS.lsp_go_to_definition.path;
        let sig = &PATHS.lsp_go_to_definition.methods.go_to_definition.sig;

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
                                        return Some(lsp_types::GotoDefinitionResponse::Scalar(lsp_types::Location {
                                            uri: (*read.get_url()).clone(),
                                            range: lsp_types::Range {
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
