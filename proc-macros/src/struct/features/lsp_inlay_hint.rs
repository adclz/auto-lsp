extern crate proc_macro;

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::field_builder::{FieldInfoExtract, Fields};
use crate::{
    r#struct::feature_builder::FeaturesCodeGen, ReferenceFeature, ReferenceFeatures,
    SymbolFeatures, PATHS,
};

use crate::Feature;

#[derive(Debug, FromMeta)]
pub struct InlayHintFeature {
    query: Option<bool>,
}

pub struct InlayHintsBuilder<'a> {
    pub input_name: &'a Ident,
    pub fields: &'a Fields,
}

impl<'a> InlayHintsBuilder<'a> {
    pub fn new(input_name: &'a Ident, fields: &'a Fields) -> Self {
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
        let sig = &PATHS.lsp_inlay_hint.build_inlay_hint.sig;

        match &params.lsp_inlay_hints {
            None => self.default_impl(),
            Some(params) => match params {
                Feature::User => quote! {},
                Feature::CodeGen(opt) => {
                    let field_names = &self.fields.field_names.get_field_names();
                    let field_vec_names = &self.fields.field_vec_names.get_field_names();
                    let field_option_names = &self.fields.field_option_names.get_field_names();
                    let queryable = &PATHS.queryable.path;

                    if opt.query.is_some() {
                        quote! {
                            impl #inlay_hint_path for #input_name {
                                #sig {
                                    use #queryable;
                                    let range = self.get_range();
                                    acc.push(auto_lsp::lsp_types::InlayHint {
                                        position: self.get_start_position(doc),
                                        kind: Some(auto_lsp::lsp_types::InlayHintKind::TYPE),
                                        label: auto_lsp::lsp_types::InlayHintLabel::String(
                                            format!("{}[{}-{}] {}", if self.is_comment() { "C" } else { "" }, range.start, range.end, Self::QUERY_NAMES[0].to_string())
                                        ),
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

    fn code_gen_reference(&self, params: &ReferenceFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let hover_info_path = &PATHS.lsp_hover_info.path;
        let sig = &PATHS.lsp_hover_info.get_hover.sig;

        match &params.lsp_inlay_hints {
            None => self.default_impl(),
            Some(params) => match params {
                ReferenceFeature::Disable => self.default_impl(),
                ReferenceFeature::Reference => {
                    quote! {
                        impl #hover_info_path for #input_name {
                            #sig {
                                if let Some(reference) = &self.get_target() {
                                    if let Some(reference) = reference.to_dyn() {
                                        reference.read().build_inlay_hint(doc, acc)
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
