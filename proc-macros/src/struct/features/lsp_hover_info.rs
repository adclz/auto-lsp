extern crate proc_macro;

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::feature_builder::FeaturesCodeGen;
use crate::field_builder::Fields;
use crate::{ReferenceFeature, ReferenceFeatures, SymbolFeatures, PATHS};

use crate::Feature;
#[derive(Debug, FromMeta)]
pub struct HoverFeature {}

pub struct HoverBuilder<'a> {
    pub input_name: &'a Ident,
    pub fields: &'a Fields,
}

impl<'a> HoverBuilder<'a> {
    pub fn new(input_name: &'a Ident, fields: &'a Fields) -> Self {
        Self { input_name, fields }
    }

    pub fn default_impl(&self) -> TokenStream {
        let input_name = &self.input_name;
        let hover_info_path = &PATHS.lsp_hover_info.path;

        quote! {
            impl #hover_info_path for #input_name { }
        }
    }
}

impl<'a> FeaturesCodeGen for HoverBuilder<'a> {
    fn code_gen(&self, params: &SymbolFeatures) -> impl quote::ToTokens {
        match &params.lsp_hover_info {
            None => self.default_impl(),
            Some(params) => match params {
                Feature::User => quote! {},
                Feature::CodeGen(_) => {
                    panic!("Hover Info does not provide code generation, instead implement the trait GetGoToDefinition manually");
                }
            },
        }
    }

    fn code_gen_reference(&self, params: &ReferenceFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let hover_info_path = &PATHS.lsp_hover_info.path;
        let sig = &PATHS.lsp_hover_info.get_hover.sig;

        match &params.lsp_hover_info {
            None => self.default_impl(),
            Some(params) => match params {
                ReferenceFeature::Disable => self.default_impl(),
                ReferenceFeature::Reference => {
                    quote! {
                        impl #hover_info_path for #input_name {
                            #sig {
                                if let Some(reference) = &self.get_target() {
                                    if let Some(reference) = reference.to_dyn() {
                                        return reference.read().get_hover(doc)
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
