extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{
    utilities::extract_fields::StructFields, AccessorFeatures, FeaturesCodeGen, SymbolFeatures,
    PATHS,
};

pub struct AccessorBuilder<'a> {
    pub input_name: &'a Ident,
    pub fields: &'a StructFields,
}

impl<'a> AccessorBuilder<'a> {
    pub fn new(input_name: &'a Ident, fields: &'a StructFields) -> Self {
        Self { input_name, fields }
    }

    pub fn default_impl(&self) -> TokenStream {
        let input_name = &self.input_name;
        let is_accessor_path = &PATHS.is_accessor.path;
        let accessor_path = &PATHS.accessor.path;

        quote! {
            impl #is_accessor_path for #input_name {}

            impl #accessor_path for #input_name {}
        }
    }
}

impl<'a> FeaturesCodeGen for AccessorBuilder<'a> {
    fn code_gen(&self, _params: &SymbolFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let is_accessor_path = &PATHS.is_accessor.path;
        let accessor_path = &PATHS.accessor.path;

        quote! {
            impl #is_accessor_path for #input_name {}

            impl #accessor_path for #input_name {}
        }
    }

    fn code_gen_accessor(&self, _params: &AccessorFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let is_accessor_path = &PATHS.is_accessor.path;

        let is_accessor_sig = &PATHS.is_accessor.methods.is_accessor.sig;
        let set_accessor_sig = &PATHS.is_accessor.methods.set_accessor.sig;
        let get_accessor = &PATHS.is_accessor.methods.get_accessor.sig;
        let reset_accessor_sig = &PATHS.is_accessor.methods.reset_accessor.sig;

        quote! {
            impl #is_accessor_path for #input_name {
                #is_accessor_sig {
                    true
                }

                #set_accessor_sig {
                    self._data.set_target(accessor);
                }

                #get_accessor {
                    self._data.get_target()
                }

                #reset_accessor_sig {
                    self._data.reset_target();
                }
            }
        }
    }
}
