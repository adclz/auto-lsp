extern crate proc_macro;

use darling::FromMeta;
use quote::quote;
use syn::{Ident, Path};

use crate::{
    utilities::{extract_fields::StructFields, format_tokens::path_to_dot_tokens},
    Feature, FeaturesCodeGen, Paths, ToCodeGen,
};

pub struct AccessorBuilder<'a> {
    pub input_name: &'a Ident,
    pub paths: &'a Paths,
    pub fields: &'a StructFields,
    pub is_accessor: bool,
}

impl<'a> AccessorBuilder<'a> {
    pub fn new(
        input_name: &'a Ident,
        is_accessor: bool,
        paths: &'a Paths,
        fields: &'a StructFields,
    ) -> Self {
        Self {
            paths,
            input_name,
            fields,
            is_accessor,
        }
    }
}

impl<'a> ToCodeGen for AccessorBuilder<'a> {
    fn to_code_gen(&self, codegen: &mut FeaturesCodeGen) {
        let input_name = &self.input_name;
        let is_accessor_path = &self.paths.is_accessor_trait;
        let accessor_path = &self.paths.accessor_trait;

        let bool = if self.is_accessor {
            quote! { true }
        } else {
            quote! { false }
        };

        codegen.input.impl_base.push(
            quote! {
                pub const IS_ACCESSOR: &'static bool = &#bool;
            }
            .into(),
        );

        codegen.input.other_impl.push(quote! {
            impl #is_accessor_path for #input_name {
                fn is_accessor(&self) -> &'static bool {
                    &Self::IS_ACCESSOR
                }
            }
        });

        if !self.is_accessor {
            codegen.input.other_impl.push(quote! {
            impl #accessor_path for #input_name {
                fn find(&self, doc: &lsp_textdocument::FullTextDocument, ctx: &dyn auto_lsp::traits::workspace::WorkspaceContext) {}
            }
        });
        }
    }
}
