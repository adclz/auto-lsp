extern crate proc_macro;

use darling::FromMeta;
use quote::quote;
use syn::{Ident, Path};

use crate::{
    utilities::{extract_fields::StructFields, format_tokens::path_to_dot_tokens},
    Feature, FeaturesCodeGen, Paths, ToCodeGen, PATHS,
};

pub struct AccessorBuilder<'a> {
    pub input_name: &'a Ident,
    pub fields: &'a StructFields,
    pub is_accessor: bool,
}

impl<'a> AccessorBuilder<'a> {
    pub fn new(input_name: &'a Ident, is_accessor: bool, fields: &'a StructFields) -> Self {
        Self {
            input_name,
            fields,
            is_accessor,
        }
    }
}

impl<'a> ToCodeGen for AccessorBuilder<'a> {
    fn to_code_gen(&self, codegen: &mut FeaturesCodeGen) {
        let input_name = &self.input_name;
        let is_accessor_path = &PATHS.is_accessor.path;
        let accessor_path = &PATHS.accessor.path;

        let is_accessor_sig = &PATHS.is_accessor.methods.is_accessor.sig;
        let is_accessor_default = &PATHS.is_accessor.methods.is_accessor.default;

        let set_accessor_sig = &PATHS.is_accessor.methods.set_accessor.sig;
        let set_accessor_default = &PATHS.is_accessor.methods.set_accessor.default;

        let bool = if self.is_accessor {
            quote! { true }
        } else {
            quote! { false }
        };

        codegen.input.impl_base.push(
            quote! {
                pub const IS_ACCESSOR: bool = #bool;
            }
            .into(),
        );

        let weak_symbol = &PATHS.weak_symbol;

        if self.is_accessor {
            codegen.input.other_impl.push(quote! {
                impl #is_accessor_path for #input_name {
                    #is_accessor_sig {
                        Self::IS_ACCESSOR
                    }

                    #set_accessor_sig {
                        self.accessor = Some(accessor);
                    }
                }
            });
        } else {
            codegen.input.other_impl.push(quote! {
                impl #is_accessor_path for #input_name {
                    #is_accessor_sig {
                        Self::IS_ACCESSOR
                    }

                    #set_accessor_sig {

                    }
                }
            });
        }

        if !self.is_accessor {
            codegen.input.other_impl.push(quote! {
            impl #accessor_path for #input_name {
                fn find(&self, doc: &lsp_textdocument::FullTextDocument, ctx: &dyn auto_lsp::workspace::WorkspaceContext) -> Result<Option<#weak_symbol>, lsp_types::Diagnostic> {
                    Ok(None)
                }
            }
        });
        }
    }
}
