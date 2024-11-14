extern crate proc_macro;

use crate::{
    utilities::{extract_fields::StructFields, format_tokens::path_to_dot_tokens},
    AstStructFeatures, CodeGen, ToCodeGen,
};
use darling::FromMeta;
use quote::quote;
use syn::Path;

use super::lsp_document_symbol::Feature;

#[derive(Debug, FromMeta)]
pub enum CompletionItemFeature {
    CompletionFn(Path),
    Item(CompletionItem),
}

#[derive(Debug, FromMeta)]
pub struct CompletionItem {
    label: Path,
    kind: Path,
}

pub fn generate_completion_item_feature(features: &AstStructFeatures, code_gen: &mut CodeGen) {
    if let Some(completion_item) = &features.lsp_completion_item {
        codegen_completion_item(&completion_item, code_gen);
    }
}

pub fn codegen_completion_item(completion: &CompletionItemFeature, code_gen: &mut CodeGen) {
    match completion {
        CompletionItemFeature::CompletionFn(completion_fn) => {
            let completion_fn = path_to_dot_tokens(completion_fn, None);
            code_gen.impl_ast_item.push(quote! {
                fn build_completion_items(&self, acc: &mut Vec<lsp_types::CompletionItem>, doc: &lsp_textdocument::FullTextDocument) {
                    #completion_fn(acc, doc)
                }
            });
        }
        CompletionItemFeature::Item(item) => {
            let kind = &item.kind;
            let label = path_to_dot_tokens(&item.label, None);

            code_gen.impl_base.push(quote! {
                const LSP_COMPLETION_ITEM_KIND: &'static lsp_types::CompletionItemKind = &#kind;
            });

            code_gen.impl_ast_item.push(quote! {
                fn build_completion_items(&self, acc: &mut Vec<lsp_types::CompletionItem>, doc: &lsp_textdocument::FullTextDocument) {
                    let read = #label.read().unwrap();

                    acc.push(lsp_types::CompletionItem {
                        label: read.get_text(doc.get_content(None).as_bytes()).to_string(),
                        kind: Some(*Self::LSP_COMPLETION_ITEM_KIND),
                        detail: None,
                        ..Default::default()
                    });
                }
            })
        }
    }
}

pub struct CompletionItemsBuilder<'a> {
    pub params: Option<&'a Feature<CompletionItemFeature>>,
    pub fields: &'a StructFields,
}

impl<'a> CompletionItemsBuilder<'a> {
    pub fn new(
        params: Option<&'a Feature<CompletionItemFeature>>,
        fields: &'a StructFields,
    ) -> Self {
        Self { params, fields }
    }
}

impl<'a> ToCodeGen for CompletionItemsBuilder<'a> {
    fn to_code_gen(&self, codegen: &mut CodeGen) {
        match self.params {
            None => codegen.impl_base.push(quote! {
                fn build_completion_items(
                    &self,
                    _acc: &mut Vec<CompletionItem>,
                    _doc: &lsp_textdocument::FullTextDocument,
                ) {}
            }),
            Some(params) => match params {
                Feature::User => (),
                Feature::CodeGen(strategy) => {
                    codegen_completion_item(strategy, codegen);
                }
            },
        }
    }
}
