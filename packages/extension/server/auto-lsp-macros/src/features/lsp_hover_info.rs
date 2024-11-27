extern crate proc_macro;

use darling::FromMeta;
use quote::quote;
use syn::Ident;

use crate::{utilities::extract_fields::StructFields, FeaturesCodeGen, Paths, ToCodeGen};

use crate::Feature;

#[derive(Debug, FromMeta)]
pub struct HoverFeature {}

pub struct HoverInfoBuilder<'a> {
    pub input_name: &'a Ident,
    pub paths: &'a Paths,
    pub params: Option<&'a Feature<HoverFeature>>,
    pub fields: &'a StructFields,
    pub is_accessor: bool,
}

impl<'a> HoverInfoBuilder<'a> {
    pub fn new(
        input_name: &'a Ident,
        paths: &'a Paths,
        params: Option<&'a Feature<HoverFeature>>,
        fields: &'a StructFields,
        is_accessor: bool,
    ) -> Self {
        Self {
            paths,
            input_name,
            params,
            fields,
            is_accessor,
        }
    }
}

impl<'a> ToCodeGen for HoverInfoBuilder<'a> {
    fn to_code_gen(&self, codegen: &mut FeaturesCodeGen) {
        let input_name = &self.input_name;
        let hover_info_path = &self.paths.hover_info_trait;

        if self.is_accessor {
            codegen.input.other_impl.push(quote! {
                impl #hover_info_path for #input_name {
                    fn get_hover(&self, doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::Hover> {
                        if let Some(accessor) = &self.accessor {
                            accessor.get_hover(doc)
                        } else {
                            None
                        }
                    }
                }
            });
            return;
        }

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
