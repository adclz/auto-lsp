extern crate proc_macro;

use darling::FromMeta;
use quote::quote;
use syn::{Path, TypeTuple};

use crate::{
    utilities::{extract_fields::StructFields, format_tokens::path_to_dot_tokens},
    Features, FeaturesCodeGen,
};

#[derive(Debug, FromMeta)]
pub struct CodeLensFeature {
    code_lens_fn: Path,
}

pub fn generate_code_lens_feature(
    features: &Features,
    code_gen_impl: &mut Vec<proc_macro2::TokenStream>,
    code_gen_impl_ast_item: &mut Vec<proc_macro2::TokenStream>,
    input: &StructFields,
) {
    if let Some(hint) = &features.lsp_code_lens {
        let code_gen = codegen_hover_info(&hint.code_lens_fn, input);
        code_gen_impl_ast_item.push(code_gen.impl_ast_item.unwrap())
    }
}

fn codegen_hover_info(path: &Path, input: &StructFields) -> FeaturesCodeGen {
    let call = path_to_dot_tokens(&path, None);

    let field_names = &input.field_names;
    let field_vec_names = &input.field_vec_names;
    let field_option_names = &input.field_option_names;
    let field_hashmap_names = &input.field_hashmap_names;

    FeaturesCodeGen {
        fields: None,
        impl_base: None,
        impl_ast_item: quote! {
            fn build_code_lens(&self, acc: &mut Vec<lsp_types::CodeLens>) {
                #call(acc);
                #(
                    self.#field_names.read().unwrap().build_code_lens(acc);
                )*
                #(
                    if let Some(field) = self.#field_option_names.as_ref() {
                        field.read().unwrap().build_code_lens(acc);
                    };
                )*
                #(
                    for field in self.#field_vec_names.iter() {
                        field.read().unwrap().build_code_lens(acc);
                    };
                )*
                #(
                    for field in self.#field_hashmap_names.values() {
                        field.read().unwrap().build_code_lens(acc);
                    };
                )*
            }
        }
        .into(),
    }
}
