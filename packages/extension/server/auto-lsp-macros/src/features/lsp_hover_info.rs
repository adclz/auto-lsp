extern crate proc_macro;

use darling::FromMeta;
use quote::quote;
use syn::Path;

use crate::{
    utilities::{extract_fields::StructFields, format_tokens::path_to_dot_tokens},
    CodeGen, ToCodeGen,
};

use super::lsp_document_symbol::Feature;

#[derive(Debug, FromMeta)]
pub struct HoverFeature {
    call: Path,
}

pub struct HoverInfoBuilder<'a> {
    pub params: Option<&'a Feature<HoverFeature>>,
    pub fields: &'a StructFields,
}

impl<'a> HoverInfoBuilder<'a> {
    pub fn new(params: Option<&'a Feature<HoverFeature>>, fields: &'a StructFields) -> Self {
        Self { params, fields }
    }
}

impl<'a> ToCodeGen for HoverInfoBuilder<'a> {
    fn to_code_gen(&self, codegen: &mut CodeGen) {
        match self.params {
            None => codegen.input.impl_base.push(quote! {
                fn get_hover(&self, _doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::Hover> {
                    None
                }
            }),
            Some(params) => match params {
                Feature::User => (),
                Feature::CodeGen(hover) => {
                    let call = path_to_dot_tokens(&hover.call, None);

                    codegen.input.impl_ast_item.push(quote! {
                        fn get_hover(&self, doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::Hover> {
                            match #call(doc) {
                                Some(hover) => Some(lsp_types::Hover {
                                    contents: lsp_types::HoverContents::Markup(
                                        lsp_types::MarkupContent {
                                            kind: lsp_types::MarkupKind::Markdown,
                                            value: hover
                                        }
                                    ),
                                    range: Some(self.get_lsp_range(doc))
                                }),
                                None => None
                            }
                        }
                    });
                }
            },
        }
    }
}
