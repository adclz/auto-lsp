extern crate proc_macro;

use darling::{ast, util, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{
    field_builder::Fields, r#struct::feature_builder::FeaturesCodeGen, Feature, ReferenceFeatures,
    StructHelpers, SymbolFeatures, PATHS,
};

#[derive(Debug, FromMeta)]
pub struct CheckFeature {}

pub struct CheckBuilder<'a> {
    pub input_name: &'a Ident,
    pub fields: &'a Fields,
    pub helper: &'a ast::Data<util::Ignored, StructHelpers>,
}

impl<'a> CheckBuilder<'a> {
    pub fn new(
        input_name: &'a Ident,
        helper: &'a ast::Data<util::Ignored, StructHelpers>,
        fields: &'a Fields,
    ) -> Self {
        Self {
            input_name,
            fields,
            helper,
        }
    }

    pub fn default_impl(&self) -> TokenStream {
        let input_name = &self.input_name;
        let is_check = &PATHS.is_check.path;
        let check = &PATHS.check.path;

        quote! {
            impl #is_check for #input_name {}

            impl #check for #input_name {}
        }
    }
}

impl<'a> FeaturesCodeGen for CheckBuilder<'a> {
    fn code_gen(&self, params: &SymbolFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let is_check = &PATHS.is_check.path;

        match &params.check {
            None => self.default_impl(),
            Some(params) => match params {
                Feature::User => quote! {
                    impl #is_check for #input_name {
                        fn must_check(&self) -> bool {
                            true
                        }
                    }
                },
                Feature::CodeGen(_) => {
                    panic!("Check does not provide code generation, instead implement the trait GetGoToDefinition manually");
                }
            },
        }
    }

    fn code_gen_accessor(&self, _params: &ReferenceFeatures) -> impl quote::ToTokens {
        self.default_impl()
    }
}
