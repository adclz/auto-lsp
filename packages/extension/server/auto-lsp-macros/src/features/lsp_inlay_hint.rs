extern crate proc_macro;

use darling::FromMeta;
use quote::quote;
use syn::{Ident, Path};

use crate::{utilities::extract_fields::StructFields, CodeGen, ToCodeGen};

use crate::{Feature, Paths};

#[derive(Debug, FromMeta)]
pub struct InlayHintFeature {}

pub struct InlayHintsBuilder<'a> {
    pub input_name: &'a Ident,
    pub paths: &'a Paths,
    pub params: Option<&'a Feature<InlayHintFeature>>,
    pub fields: &'a StructFields,
}

impl<'a> InlayHintsBuilder<'a> {
    pub fn new(
        input_name: &'a Ident,
        paths: &'a Paths,
        params: Option<&'a Feature<InlayHintFeature>>,
        fields: &'a StructFields,
    ) -> Self {
        Self {
            paths,
            input_name,
            params,
            fields,
        }
    }
}

impl<'a> ToCodeGen for InlayHintsBuilder<'a> {
    fn to_code_gen(&self, codegen: &mut CodeGen) {
        let input_name = &self.input_name;
        let inlay_hint_path = &self.paths.inlay_hints_trait;

        match self.params {
            None => codegen.input.other_impl.push(quote! {
                impl #inlay_hint_path for #input_name {
                    fn build_inlay_hint(&self, _acc: &mut Vec<lsp_types::InlayHint>) {}
                }
            }),
            Some(params) => match params {
                Feature::User => (),
                Feature::CodeGen(_) => {
                    panic!("Inlay Hint does not provide (yet) code generation, instead implement the trait InlayHint manually");
                }
            },
        }
    }
}
