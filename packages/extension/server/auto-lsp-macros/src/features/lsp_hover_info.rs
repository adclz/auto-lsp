extern crate proc_macro;

use darling::FromMeta;
use quote::quote;
use syn::Ident;

use crate::{utilities::extract_fields::StructFields, FeaturesCodeGen, Paths, ToCodeGen};

use crate::{Feature, PATHS};

#[derive(Debug, FromMeta)]
pub struct HoverFeature {}

pub struct HoverInfoBuilder<'a> {
    pub input_name: &'a Ident,
    pub params: Option<&'a Feature<HoverFeature>>,
    pub fields: &'a StructFields,
    pub is_accessor: bool,
}

impl<'a> HoverInfoBuilder<'a> {
    pub fn new(
        input_name: &'a Ident,
        params: Option<&'a Feature<HoverFeature>>,
        fields: &'a StructFields,
        is_accessor: bool,
    ) -> Self {
        Self {
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
        let hover_info_path = &PATHS.lsp_hover_info.path;
        let sig = &PATHS.lsp_hover_info.methods.get_hover.sig;
        let default = &PATHS.lsp_hover_info.methods.get_hover.default;

        if self.is_accessor {
            codegen.input.other_impl.push(quote! {
                impl #hover_info_path for #input_name {
                    #sig {
                        if let Some(accessor) = &self.accessor {
                            if let Some(accessor) = accessor.to_dyn() {
                                return accessor.read().get_hover(doc)
                            }
                        }
                        None
                    }
                }
            });
            return;
        }

        match self.params {
            None => codegen.input.other_impl.push(quote! {
                impl #hover_info_path for #input_name {
                    #sig { #default }
                }
            }),
            Some(params) => match params {
                Feature::User => (),
                Feature::CodeGen(_) => {
                    panic!("Hover Info does not provide code generation, instead implement the trait HoverInfo manually");
                }
            },
        }
    }
}
