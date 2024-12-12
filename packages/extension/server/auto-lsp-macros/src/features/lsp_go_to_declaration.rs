extern crate proc_macro;

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::Feature;
use crate::{
    utilities::extract_fields::StructFields, AccessorFeatures, FeaturesCodeGen, ReferenceFeature,
    SymbolFeatures, PATHS,
};

#[derive(Debug, FromMeta)]
pub struct GoToDeclarationFeature {}

pub struct GoToDeclarationBuilder<'a> {
    pub input_name: &'a Ident,
    pub fields: &'a StructFields,
}

impl<'a> GoToDeclarationBuilder<'a> {
    pub fn new(input_name: &'a Ident, fields: &'a StructFields) -> Self {
        Self { input_name, fields }
    }

    pub fn default_impl(&self) -> TokenStream {
        let input_name = &self.input_name;
        let go_to_declarations_path = &PATHS.lsp_go_to_declaration.path;

        quote! {
            impl #go_to_declarations_path for #input_name { }
        }
    }
}

impl<'a> FeaturesCodeGen for GoToDeclarationBuilder<'a> {
    fn code_gen(&self, params: &SymbolFeatures) -> impl quote::ToTokens {
        match &params.lsp_go_to_declaration {
            None => self.default_impl(),
            Some(params) => match params {
                Feature::User => quote! {},
                Feature::CodeGen(_) => {
                    panic!("Go to Definition does not provide code generation, instead implement the trait GoToDeclaration manually");
                }
            },
        }
    }

    fn code_gen_accessor(&self, params: &AccessorFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let go_to_declarations_path = &PATHS.lsp_go_to_declaration.path;
        let sig = &PATHS.lsp_go_to_declaration.methods.go_to_declaration.sig;

        match &params.lsp_go_to_declaration {
            None => self.default_impl(),
            Some(params) => match params {
                ReferenceFeature::Disable => self.default_impl(),
                ReferenceFeature::Reference => {
                    quote! {
                        impl #go_to_declarations_path for #input_name {
                            #sig {
                                if let Some(accessor) = &self.get_target() {
                                    if let Some(accessor) = accessor.to_dyn() {
                                        let read = accessor.read();
                                        return Some(lsp_types::request::GotoDeclarationResponse::Scalar(lsp_types::Location {
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
