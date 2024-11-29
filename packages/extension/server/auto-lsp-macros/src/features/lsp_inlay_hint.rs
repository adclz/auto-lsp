extern crate proc_macro;

use darling::FromMeta;
use quote::quote;
use syn::{Ident, Path};

use crate::utilities::extract_fields::FieldInfoExtract;
use crate::{utilities::extract_fields::StructFields, FeaturesCodeGen, ToCodeGen};

use crate::{Feature, Paths};

#[derive(Debug, FromMeta)]
pub struct InlayHintFeature {
    query: Option<bool>,
}

pub struct InlayHintsBuilder<'a> {
    pub input_name: &'a Ident,
    pub paths: &'a Paths,
    pub params: Option<&'a Feature<InlayHintFeature>>,
    pub fields: &'a StructFields,
    pub is_accessor: bool,
}

impl<'a> InlayHintsBuilder<'a> {
    pub fn new(
        input_name: &'a Ident,
        paths: &'a Paths,
        params: Option<&'a Feature<InlayHintFeature>>,
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

impl<'a> ToCodeGen for InlayHintsBuilder<'a> {
    fn to_code_gen(&self, codegen: &mut FeaturesCodeGen) {
        let input_name = &self.input_name;
        let inlay_hint_path = &self.paths.inlay_hints_trait;

        if self.is_accessor {
            codegen.input.other_impl.push(quote! {
                impl #inlay_hint_path for #input_name {
                    fn build_inlay_hint(&self, doc: &lsp_textdocument::FullTextDocument, acc: &mut Vec<lsp_types::InlayHint>) {
                        if let Some(accessor) = &self.accessor {
                            if let Some(accessor) = accessor.to_dyn() {
                                accessor.read().build_inlay_hint(doc, acc)
                            }
                        }
                    }
                }
            });
            return;
        }

        match self.params {
            None => codegen.input.other_impl.push(quote! {
                impl #inlay_hint_path for #input_name {
                    fn build_inlay_hint(&self, doc: &lsp_textdocument::FullTextDocument, _acc: &mut Vec<lsp_types::InlayHint>) {}
                }
            }),
            Some(params) => match params {
                Feature::User => (),
                Feature::CodeGen(opt) => {
                    let field_names = &self.fields.field_names.get_field_names();
                    let field_vec_names = &self.fields.field_vec_names.get_field_names();
                    let field_option_names = &self.fields.field_option_names.get_field_names();
                    let field_hashmap_names = &self.fields.field_hashmap_names.get_field_names();

                    if opt.query.is_some() {
                        codegen.input.other_impl.push(quote! {
                            impl #inlay_hint_path for #input_name {
                                fn build_inlay_hint(&self, doc: &lsp_textdocument::FullTextDocument, acc: &mut Vec<lsp_types::InlayHint>) {
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
                                    #(
                                        for field in self.#field_hashmap_names.values() {
                                            field.read().build_inlay_hint(doc, acc);
                                        };
                                    )*
                                }
                            }
                        });
                    } else {
                    panic!("Inlay Hint does not provide (yet) code generation, instead implement the trait InlayHint manually");
                }
            },
        }
    }
    }
}
