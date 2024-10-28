extern crate proc_macro;

use darling::{ast::NestedMeta, util::PathList, Error, FromMeta};
use proc_macro2::{Group, TokenStream, TokenTree};
use syn::{Lit, Meta, Path};

use crate::{utilities::format_tokens::path_to_dot_tokens, Features, FeaturesCodeGen};

#[derive(Debug, FromMeta)]
pub enum UniquenessFeature {
    list(PathList),
    uniquness_fn(Path),
}

pub fn generate_uniqueness_feature(
    features: &Features,
    code_gen_impl: &mut Vec<proc_macro2::TokenStream>,
    code_gen_impl_ast_item: &mut Vec<proc_macro2::TokenStream>,
) {
    if let Some(semantic) = &features.uniqueness {
        let code_gen = codegen(&semantic);
        code_gen_impl_ast_item.push(code_gen.impl_ast_item.unwrap())
    }
}

fn codegen(features: &UniquenessFeature) -> FeaturesCodeGen {
    FeaturesCodeGen {
        fields: None,
        impl_base: None,
        impl_ast_item: None,
    }
    .into()
}
