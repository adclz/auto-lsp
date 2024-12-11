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
        let is_accessor_sig = &PATHS.is_accessor.methods.is_accessor.sig;
        let is_accessor_default = &PATHS.is_accessor.methods.is_accessor.default;

        let set_accessor_sig = &PATHS.is_accessor.methods.set_accessor.sig;
        let set_accessor_default = &PATHS.is_accessor.methods.set_accessor.default;

        let accessor_path = &PATHS.accessor.path;
        let accessor_find_sig = &PATHS.accessor.methods.find.sig;
        let accessor_find_default = &PATHS.accessor.methods.find.default;

        quote! {
            impl #is_accessor_path for #input_name {
                #is_accessor_sig {
                    #is_accessor_default
                }

                #set_accessor_sig {
                    #set_accessor_default
                }
            }

            impl #accessor_path for #input_name {
                #accessor_find_sig {
                    #accessor_find_default
                }
            }
        }
    }
}

impl<'a> FeaturesCodeGen for AccessorBuilder<'a> {
    fn code_gen(&self, _params: &SymbolFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let is_accessor_path = &PATHS.is_accessor.path;

        let is_accessor_sig = &PATHS.is_accessor.methods.is_accessor.sig;
        let is_accessor_default = &PATHS.is_accessor.methods.is_accessor.default;

        let set_accessor_sig = &PATHS.is_accessor.methods.set_accessor.sig;
        let set_accessor_default = &PATHS.is_accessor.methods.set_accessor.default;

        let accessor_path = &PATHS.accessor.path;
        let accessor_find_sig = &PATHS.accessor.methods.find.sig;
        let accessor_find_default = &PATHS.accessor.methods.find.default;

        quote! {
            impl #is_accessor_path for #input_name {
                #is_accessor_sig {
                    #is_accessor_default
                }

                #set_accessor_sig {
                    #set_accessor_default
                }
            }

            impl #accessor_path for #input_name {
                #accessor_find_sig {
                    #accessor_find_default
                }
            }
        }
    }

    fn code_gen_accessor(&self, _params: &AccessorFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let is_accessor_path = &PATHS.is_accessor.path;
        let is_accessor_sig = &PATHS.is_accessor.methods.is_accessor.sig;

        let set_accessor_sig = &PATHS.is_accessor.methods.set_accessor.sig;

        quote! {
            impl #is_accessor_path for #input_name {
                #is_accessor_sig {
                    true
                }

                #set_accessor_sig {
                    self._data.set_target(accessor);
                }
            }
        }
    }
}
