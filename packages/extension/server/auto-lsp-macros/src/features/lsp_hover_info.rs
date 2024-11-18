extern crate proc_macro;

use darling::FromMeta;
use quote::quote;
use syn::Ident;

use crate::{utilities::extract_fields::StructFields, CodeGen, Paths, ToCodeGen};

use crate::Feature;

#[derive(Debug, FromMeta)]
pub struct HoverFeature {}

pub struct HoverInfoBuilder<'a> {
    pub input_name: &'a Ident,
    pub paths: &'a Paths,
    pub params: Option<&'a Feature<HoverFeature>>,
    pub fields: &'a StructFields,
}

impl<'a> HoverInfoBuilder<'a> {
    pub fn new(
        input_name: &'a Ident,
        paths: &'a Paths,
        params: Option<&'a Feature<HoverFeature>>,
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

impl<'a> ToCodeGen for HoverInfoBuilder<'a> {
    fn to_code_gen(&self, codegen: &mut CodeGen) {
        let input_name = &self.input_name;
        let hover_info_path = &self.paths.hover_info_trait_path;

        match self.params {
            None => codegen.input.other_impl.push(quote! {
                impl #hover_info_path for #input_name {
                    fn get_hover(&self, _doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::Hover> {
                        None
                    }
                }
            }),
            Some(params) => match params {
                Feature::User => (),
                Feature::CodeGen(_) => {
                    panic!("Hover Info does not provide code generation, instead implement the trait HoverInfo manually");
                }
            }
        }
    }
}
