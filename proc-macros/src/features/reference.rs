extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{
    utilities::extract_fields::StructFields, FeaturesCodeGen, ReferenceFeatures, SymbolFeatures,
    PATHS,
};

pub struct ReferenceBuilder<'a> {
    pub input_name: &'a Ident,
    pub fields: &'a StructFields,
}

impl<'a> ReferenceBuilder<'a> {
    pub fn new(input_name: &'a Ident, fields: &'a StructFields) -> Self {
        Self { input_name, fields }
    }

    pub fn default_impl(&self) -> TokenStream {
        let input_name = &self.input_name;
        let is_reference_path = &PATHS.is_reference.path;
        let accessor_path = &PATHS.accessor.path;

        quote! {
            impl #is_reference_path for #input_name {}

            impl #accessor_path for #input_name {}
        }
    }
}

impl<'a> FeaturesCodeGen for ReferenceBuilder<'a> {
    fn code_gen(&self, _params: &SymbolFeatures) -> impl quote::ToTokens {
        self.default_impl()
    }

    fn code_gen_accessor(&self, _params: &ReferenceFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let is_reference_path = &PATHS.is_reference.path;

        let is_reference_sig = &PATHS.is_reference.is_reference.sig;

        quote! {
            impl #is_reference_path for #input_name {
                #is_reference_sig {
                    true
                }
            }
        }
    }
}
