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
pub struct HoverFeature {}

pub struct HoverInfoBuilder<'a> {
    pub input_name: &'a Ident,
    pub fields: &'a StructFields,
}

impl<'a> HoverInfoBuilder<'a> {
    pub fn new(input_name: &'a Ident, fields: &'a StructFields) -> Self {
        Self { input_name, fields }
    }

    pub fn default_impl(&self) -> TokenStream {
        let input_name = &self.input_name;
        let hover_info_path = &PATHS.lsp_hover_info.path;
        let sig = &PATHS.lsp_hover_info.methods.get_hover.sig;
        let default = &PATHS.lsp_hover_info.methods.get_hover.default;

        quote! {
            impl #hover_info_path for #input_name {
                #sig { #default }
            }
        }
    }
}

impl<'a> FeaturesCodeGen for HoverInfoBuilder<'a> {
    fn code_gen(&self, params: &SymbolFeatures) -> impl quote::ToTokens {
        match &params.lsp_hover_info {
            None => self.default_impl(),
            Some(params) => match params {
                Feature::User => quote! {},
                Feature::CodeGen(_) => {
                    panic!("Hover Info does not provide code generation, instead implement the trait GoToDefinition manually");
                }
            },
        }
    }

    fn code_gen_accessor(&self, params: &AccessorFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let hover_info_path = &PATHS.lsp_hover_info.path;
        let sig = &PATHS.lsp_hover_info.methods.get_hover.sig;

        match &params.lsp_hover_info {
            None => self.default_impl(),
            Some(params) => match params {
                ReferenceFeature::Disable => self.default_impl(),
                ReferenceFeature::Reference => {
                    quote! {
                        impl #hover_info_path for #input_name {
                            #sig {
                                if let Some(accessor) = &self.accessor {
                                    if let Some(accessor) = accessor.to_dyn() {
                                        return accessor.read().get_hover(doc)
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
