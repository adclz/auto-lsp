extern crate proc_macro;

use darling::FromMeta;
use quote::quote;
use syn::{Path, TypeTuple};

use crate::{
    utilities::{extract_fields::StructFields, format_tokens::path_to_dot_tokens},
    Features, FeaturesCodeGen,
};

#[derive(Debug, FromMeta)]
pub struct InlayHintFeature {
    inlay_hint_fn: Path,
}

pub fn generate_inlay_hint_feature(
    features: &Features,
    code_gen_impl: &mut Vec<proc_macro2::TokenStream>,
    code_gen_impl_ast_item: &mut Vec<proc_macro2::TokenStream>,
    input: &StructFields,
) {
    if let Some(hint) = &features.lsp_inlay_hint {
        let code_gen = codegen_hover_info(&hint.inlay_hint_fn, input);
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
            fn build_inlay_hint(&self, acc: &mut Vec<lsp_types::InlayHint>) {
                #call(acc);
                #(
                    self.#field_names.read().unwrap().build_inlay_hint(acc);
                )*
                #(
                    if let Some(field) = self.#field_option_names.as_ref() {
                        field.read().unwrap().build_inlay_hint(acc);
                    };
                )*
                #(
                    for field in self.#field_vec_names.iter() {
                        field.read().unwrap().build_inlay_hint(acc);
                    };
                )*
                #(
                    for field in self.#field_hashmap_names.values() {
                        field.read().unwrap().build_inlay_hint(acc);
                    };
                )*
            }
        }
        .into(),
    }
}
