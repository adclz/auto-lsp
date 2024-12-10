extern crate proc_macro;

use darling::{ast, util, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Path};

use crate::{
    utilities::extract_fields::StructFields, AccessorFeatures, Feature, FeaturesCodeGen,
    StructHelpers, SymbolFeatures, PATHS,
};

#[derive(Debug, FromMeta)]
pub struct CheckFeature {}

#[derive(Debug, FromMeta)]
pub struct DuplicateCheck {
    check_fn: Path,
    #[darling(multiple)]
    merge: Vec<OtherDuplicateCheck>,
}

#[derive(Debug, Clone, FromMeta)]
struct OtherDuplicateCheck {
    vec: Path,
    check_fn: Path,
}

pub struct CheckBuilder<'a> {
    pub input_name: &'a Ident,
    pub fields: &'a StructFields,
    pub helper: &'a ast::Data<util::Ignored, StructHelpers>,
}

impl<'a> CheckBuilder<'a> {
    pub fn new(
        input_name: &'a Ident,
        helper: &'a ast::Data<util::Ignored, StructHelpers>,
        fields: &'a StructFields,
    ) -> Self {
        Self {
            input_name,
            fields,
            helper,
        }
    }

    pub fn default_impl(&self) -> TokenStream {
        let input_name = &self.input_name;
        let check = &PATHS.check.path;
        let must_check_sig = &PATHS.check.methods.must_check.sig;
        let must_check_default = &PATHS.check.methods.must_check.default;

        let check_sig = &PATHS.check.methods.check.sig;
        let check_default = &PATHS.check.methods.check.default;

        quote! {
            impl #check for #input_name {
                #must_check_sig {
                    #must_check_default
                }

                #check_sig {
                    #check_default
                }
            }
        }
    }
}

impl<'a> FeaturesCodeGen for CheckBuilder<'a> {
    fn code_gen(&self, params: &SymbolFeatures) -> impl quote::ToTokens {
        match &params.check {
            None => self.default_impl(),
            Some(params) => match params {
                Feature::User => quote! {},
                Feature::CodeGen(_) => {
                    panic!("Check does not provide code generation, instead implement the trait GoToDefinition manually");
                }
            },
        }
    }

    fn code_gen_accessor(&self, _params: &AccessorFeatures) -> impl quote::ToTokens {
        self.default_impl()
    }
}
