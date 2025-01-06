extern crate proc_macro;

use darling::{ast, util, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Path};

use crate::{
    utilities::extract_fields::StructFields, AccessorFeatures, Feature, FeaturesCodeGen,
    ReferenceFeature, StructHelpers, SymbolFeatures, PATHS,
};

#[derive(Debug, FromMeta)]
pub struct CommentFeature {}

pub struct CommentBuilder<'a> {
    pub input_name: &'a Ident,
    pub fields: &'a StructFields,
}

impl<'a> CommentBuilder<'a> {
    pub fn new(input_name: &'a Ident, fields: &'a StructFields) -> Self {
        Self { input_name, fields }
    }

    pub fn default_impl(&self) -> TokenStream {
        let input_name = &self.input_name;
        let is_comment = &PATHS.is_comment.path;

        quote! {
            impl #is_comment for #input_name {}
        }
    }
}

impl<'a> FeaturesCodeGen for CommentBuilder<'a> {
    fn code_gen(&self, params: &SymbolFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let is_comment = &PATHS.is_comment.path;

        match &params.comment {
            None => self.default_impl(),
            Some(params) => match params {
                Feature::User => quote! {
                    impl #is_comment for #input_name {
                        fn is_comment(&self) -> bool {
                            true
                        }
                    }
                },
                Feature::CodeGen(_) => {
                    panic!("Comment does not provide code generation");
                }
            },
        }
    }

    fn code_gen_accessor(&self, params: &AccessorFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let is_comment = &PATHS.is_comment.path;

        match &params.comment {
            None => self.default_impl(),
            Some(params) => match &params {
                ReferenceFeature::Disable => self.default_impl(),
                ReferenceFeature::Reference => quote! {
                    impl #is_comment for #input_name {
                        fn is_comment(&self) -> bool {
                            true
                        }
                    }
                },
                ReferenceFeature::User => quote! {},
            },
        }
    }
}
