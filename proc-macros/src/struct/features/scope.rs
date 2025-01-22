extern crate proc_macro;

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Path};

use crate::field_builder::Fields;
use crate::utilities::path_to_dot_tokens;
use crate::{r#struct::feature_builder::FeaturesCodeGen, ReferenceFeatures, SymbolFeatures, PATHS};

use crate::Feature;
#[derive(Debug, FromMeta)]
pub struct ScopeFeature {
    range: ScopeRange,
}

#[derive(Debug, FromMeta)]
pub struct ScopeRange {
    start: Path,
    end: Path,
}

pub struct ScopeBuilder<'a> {
    pub input_name: &'a Ident,
    pub fields: &'a Fields,
}

impl<'a> ScopeBuilder<'a> {
    pub fn new(input_name: &'a Ident, fields: &'a Fields) -> Self {
        Self { input_name, fields }
    }

    pub fn default_impl(&self) -> TokenStream {
        let input_name = &self.input_name;
        let is_scope_path = &PATHS.is_scope.path;

        let scope_path = &PATHS.scope.path;
        quote! {
            impl #is_scope_path for #input_name {}

            impl #scope_path for #input_name {}

        }
    }
}

impl<'a> FeaturesCodeGen for ScopeBuilder<'a> {
    fn code_gen(&self, params: &SymbolFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let is_scope_path = &PATHS.is_scope.path;
        let scope_path = &PATHS.scope.path;

        let is_scope_sig = &PATHS.is_scope.is_scope.sig;

        let get_scope_range_sig = &PATHS.scope.get_scope_range.sig;

        match &params.scope {
            None => self.default_impl(),
            Some(params) => match params {
                Feature::User => quote! {
                    impl #is_scope_path for #input_name {
                        #is_scope_sig {
                            true
                        }
                    }
                },
                Feature::CodeGen(scope) => {
                    let range = &scope.range;
                    let start = path_to_dot_tokens(&range.start, None);
                    let end = path_to_dot_tokens(&range.end, None);

                    quote! {
                        impl #is_scope_path for #input_name {
                            #is_scope_sig {
                                true
                            }
                        }

                        impl #scope_path for #input_name {
                            #get_scope_range_sig {
                                let start = #start.read().get_range().start;
                                let end = #end.read().get_range().end;

                                vec!([start, end])
                            }
                        }
                    }
                }
            },
        }
    }

    fn code_gen_reference(&self, _params: &ReferenceFeatures) -> impl quote::ToTokens {
        self.default_impl()
    }
}
