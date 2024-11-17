extern crate proc_macro;

use darling::FromMeta;
use quote::quote;
use syn::Path;

use crate::{
    utilities::{
        extract_fields::{FieldInfoExtract, StructFields},
        format_tokens::path_to_dot_tokens,
    },
    CodeGen, ToCodeGen,
};

use super::lsp_document_symbol::Feature;

#[derive(Debug, FromMeta)]
pub struct InlayHintFeature {
    inlay_hint_fn: Path,
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
            None => codegen.input.impl_base.push(quote! {
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
