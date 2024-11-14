extern crate proc_macro;

use darling::FromMeta;
use quote::quote;
use syn::Path;

use crate::{
    utilities::{extract_fields::StructFields, format_tokens::path_to_dot_tokens},
    AstStructFeatures, CodeGen, ToCodeGen,
};

use super::lsp_document_symbol::Feature;

#[derive(Debug, FromMeta)]
pub struct InlayHintFeature {
    inlay_hint_fn: Path,
}

pub fn generate_inlay_hint_feature(
    features: &AstStructFeatures,
    code_gen: &mut CodeGen,
    input: &StructFields,
) {
    if let Some(hint) = &features.lsp_inlay_hint {
        codegen_hover_info(&hint, code_gen, input);
    }
}

fn codegen_hover_info(path: &InlayHintFeature, code_gen: &mut CodeGen, input: &StructFields) {
    let call = path_to_dot_tokens(&path.inlay_hint_fn, None);

    let field_names = &input.field_names;
    let field_vec_names = &input.field_vec_names;
    let field_option_names = &input.field_option_names;
    let field_hashmap_names = &input.field_hashmap_names;

    code_gen.impl_ast_item.push(quote! {
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
    });
}

pub struct InlayHintsBuilder<'a> {
    pub params: Option<&'a Feature<InlayHintFeature>>,
    pub fields: &'a StructFields,
}

impl<'a> InlayHintsBuilder<'a> {
    pub fn new(params: Option<&'a Feature<InlayHintFeature>>, fields: &'a StructFields) -> Self {
        Self { params, fields }
    }
}

impl<'a> ToCodeGen for InlayHintsBuilder<'a> {
    fn to_code_gen(&self, codegen: &mut CodeGen) {
        match self.params {
            None => codegen.impl_base.push(quote! {
                fn build_inlay_hint(&self, _acc: &mut Vec<lsp_types::InlayHint>) {}
            }),
            Some(params) => match params {
                Feature::User => (),
                Feature::CodeGen(inlay) => {
                    todo!()
                }
            },
        }
    }
}
