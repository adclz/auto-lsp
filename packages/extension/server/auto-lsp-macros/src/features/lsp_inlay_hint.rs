extern crate proc_macro;

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{
    utilities::extract_fields::{FieldInfoExtract, StructFields},
    AccessorFeatures, FeaturesCodeGen, ReferenceFeature, SymbolFeatures, PATHS,
};

use crate::Feature;

#[derive(Debug, FromMeta)]
pub struct InlayHintFeature {
    query: Option<bool>,
}

pub struct InlayHintsBuilder<'a> {
    pub input_name: &'a Ident,
    pub fields: &'a StructFields,
}

impl<'a> InlayHintsBuilder<'a> {
    pub fn new(input_name: &'a Ident, fields: &'a StructFields) -> Self {
        Self { input_name, fields }
    }

    pub fn default_impl(&self) -> TokenStream {
        let input_name = &self.input_name;
        let inlay_hint_path = &PATHS.lsp_inlay_hint.path;

        quote! {
            impl #inlay_hint_path for #input_name { }
        }
    }
}

impl<'a> FeaturesCodeGen for InlayHintsBuilder<'a> {
    fn code_gen(&self, params: &SymbolFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let inlay_hint_path = &PATHS.lsp_inlay_hint.path;
        let sig = &PATHS.lsp_inlay_hint.methods.build_inlay_hint.sig;

        match &params.lsp_inlay_hints {
            None => self.default_impl(),
            Some(params) => match params {
                Feature::User => quote! {},
                Feature::CodeGen(opt) => {
                    let field_names = &self.fields.field_names.get_field_names();
                    let field_vec_names = &self.fields.field_vec_names.get_field_names();
                    let field_option_names = &self.fields.field_option_names.get_field_names();

                    if opt.query.is_some() {
                        quote! {
                            impl #inlay_hint_path for #input_name {
                                #sig {
                                    acc.push(lsp_types::InlayHint {
                                        position: self.get_start_position(doc),
                                        kind: Some(lsp_types::InlayHintKind::TYPE),
                                        label: lsp_types::InlayHintLabel::String(Self::QUERY_NAMES[0].to_string()),
                                        text_edits: None,
                                        tooltip: None,
                                        padding_left: None,
                                        padding_right: Some(true),
                                        data: None,
                                    });
                                    #(
                                        self.#field_names.read().build_inlay_hint(doc, acc);
                                    )*
                                    #(
                                        if let Some(field) = self.#field_option_names.as_ref() {
                                            field.read().build_inlay_hint(doc, acc);
                                        };
                                    )*
                                    #(
                                        for field in self.#field_vec_names.iter() {
                                            field.read().build_inlay_hint(doc, acc);
                                        };
                                    )*
                                }
                            }
                        }
                    } else {
                        panic!("Inlay Hint does not provide (yet) code generation, instead implement the trait InlayHint manually");
                    }
                }
            },
        }
    }

    fn code_gen_accessor(&self, params: &AccessorFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let hover_info_path = &PATHS.lsp_hover_info.path;
        let sig = &PATHS.lsp_hover_info.methods.get_hover.sig;

        match &params.lsp_inlay_hints {
            None => self.default_impl(),
            Some(params) => match params {
                ReferenceFeature::Disable => self.default_impl(),
                ReferenceFeature::Reference => {
                    quote! {
                        impl #hover_info_path for #input_name {
                            #sig {
                                if let Some(accessor) = &self.get_target() {
                                    if let Some(accessor) = accessor.to_dyn() {
                                        accessor.read().build_inlay_hint(doc, acc)
                                    }
                                }
                            }
                        }
                    }
                }
                ReferenceFeature::User => quote! {},
            },
        }
    }
}
