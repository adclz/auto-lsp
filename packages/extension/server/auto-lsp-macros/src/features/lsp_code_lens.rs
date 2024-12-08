extern crate proc_macro;

use darling::FromMeta;
use quote::quote;
use syn::{Ident, Path};

use crate::{
    utilities::{
        extract_fields::{FieldInfoExtract, StructFields},
        format_tokens::path_to_dot_tokens,
    },
    FeaturesCodeGen, Paths, ToCodeGen, PATHS,
};

use crate::Feature;

#[derive(Debug, FromMeta)]
pub struct CodeLensFeature {
    code_lens_fn: Path,
}

pub struct CodeLensBuilder<'a> {
    pub input_name: &'a Ident,
    pub params: Option<&'a Feature<CodeLensFeature>>,
    pub fields: &'a StructFields,
    pub is_accessor: bool,
}

impl<'a> CodeLensBuilder<'a> {
    pub fn new(
        input_name: &'a Ident,
        params: Option<&'a Feature<CodeLensFeature>>,
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

impl<'a> ToCodeGen for CodeLensBuilder<'a> {
    fn to_code_gen(&self, codegen: &mut FeaturesCodeGen) {
        let input_name = &self.input_name;
        let code_lens_path = &PATHS.lsp_code_lens.path;
        let sig = &PATHS.lsp_code_lens.methods.build_code_lens.sig;
        let default = &PATHS.lsp_code_lens.methods.build_code_lens.default;

        if self.is_accessor {
            codegen.input.other_impl.push(quote! {
                impl #code_lens_path for #input_name {
                    #sig {
                        if let Some(accessor) = &self.accessor {
                            if let Some(accessor) = accessor.to_dyn() {
                                accessor.read().build_code_lens(acc)
                            }
                        }
                    }
                }
            });
            return;
        }

        match self.params {
            None => codegen.input.other_impl.push(quote! {
                impl #code_lens_path for #input_name {
                    #sig { #default }
                }
            }),
            Some(params) => match params {
                Feature::User => (),
                Feature::CodeGen(code_lens) => {
                    let call = path_to_dot_tokens(&code_lens.code_lens_fn, None);

                    let field_names = &self.fields.field_names.get_field_names();
                    let field_vec_names = &self.fields.field_vec_names.get_field_names();
                    let field_option_names = &self.fields.field_option_names.get_field_names();

                    codegen.input.other_impl.push(quote! {
                        impl #code_lens_path for #input_name {
                            #sig {
                                #call(acc);
                                #(
                                    self.#field_names.read().build_code_lens(acc);
                                )*
                                #(
                                    if let Some(field) = self.#field_option_names.as_ref() {
                                        field.read().build_code_lens(acc);
                                    };
                                )*
                                #(
                                    for field in self.#field_vec_names.iter() {
                                        field.read().build_code_lens(acc);
                                    };
                                )*
                            }
                        }
                    });
                }
            },
        }
    }
}
