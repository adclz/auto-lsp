extern crate proc_macro;

use crate::{
    utilities::{extract_fields::StructFields, format_tokens::path_to_dot_tokens},
    FeaturesCodeGen, Paths, ToCodeGen, PATHS,
};
use darling::{util::PathList, FromMeta};
use quote::quote;
use syn::{Ident, Path};

use crate::Feature;

#[derive(Debug, FromMeta)]
pub struct GotoDefinitionFeature {}

pub struct GotoDefinitionBuilder<'a> {
    pub input_name: &'a Ident,
    pub params: Option<&'a Feature<GotoDefinitionFeature>>,
    pub fields: &'a StructFields,
    pub is_accessor: bool,
}

impl<'a> GotoDefinitionBuilder<'a> {
    pub fn new(
        input_name: &'a Ident,
        params: Option<&'a Feature<GotoDefinitionFeature>>,
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

impl<'a> ToCodeGen for GotoDefinitionBuilder<'a> {
    fn to_code_gen(&self, codegen: &mut FeaturesCodeGen) {
        let input_name = &self.input_name;
        let go_to_definitions_path = &PATHS.lsp_go_to_definition.path;
        let sig = &PATHS.lsp_go_to_definition.methods.go_to_definition.sig;
        let default = &PATHS.lsp_go_to_definition.methods.go_to_definition.default;

        if self.is_accessor {
            codegen.input.other_impl.push(quote! {
                impl #go_to_definitions_path for #input_name {
                    #sig {
                        if let Some(accessor) = &self.accessor {
                            if let Some(accessor) = accessor.to_dyn() {
                                let read = accessor.read();
                                return Some(lsp_types::GotoDefinitionResponse::Scalar(lsp_types::Location {
                                    uri: (*read.get_url()).clone(),
                                    range: lsp_types::Range {
                                        start: read.get_start_position(doc),
                                        end: read.get_end_position(doc),
                                    },
                                }))
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
                impl #go_to_definitions_path for #input_name {
                    #sig {
                        #default
                    }
                }
            }),
            Some(params) => match params {
                Feature::User => (),
                Feature::CodeGen(_) => {
                    panic!("Go to Definition does not provide code generation, instead implement the trait GoToDefinition manually");
                }
            },
        }
    }
}
