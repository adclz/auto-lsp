extern crate proc_macro;

use crate::{utilities::format_tokens::path_to_dot_tokens, CodeGen, Features};
use darling::FromMeta;
use quote::quote;
use syn::Path;

#[derive(Debug, FromMeta)]
pub enum CompletionItemFeature {
    CompletionFn(Path),
    Item(CompletionItem),
}

#[derive(Debug, FromMeta)]
pub struct CompletionItem {
    label: String,
    kind: Path,
    detail: String,
}

pub fn generate_completion_item_feature(features: &Features, code_gen: &mut CodeGen) {
    if let Some(completion_item) = &features.lsp_completion_item {
        codegen_completion_item(&completion_item, code_gen);
    }
}

pub fn codegen_completion_item(completion: &CompletionItemFeature, code_gen: &mut CodeGen) {
    match completion {
        CompletionItemFeature::CompletionFn(completion_fn) => {
            let completion_fn = path_to_dot_tokens(completion_fn, None);
            code_gen.impl_ast_item.push(quote! {
                fn build_completion_items(&self, acc: &mut Vec<lsp_types::CompletionItem>) {
                    #completion_fn(acc)
                }
            });
        }
        CompletionItemFeature::Item(item) => {
            let kind = &item.kind;
            let label = &item.label;
            let detail = &item.detail;

            code_gen.impl_base.push(quote! {
                const LSP_COMPLETION_ITEM_KIND: &'static lsp_types::CompletionItemKind = &#kind;
            });

            code_gen.impl_ast_item.push(quote! {
                fn build_completion_items(&self, acc: &mut Vec<lsp_types::CompletionItem>) {
                    acc.push(lsp_types::CompletionItem {
                        label: #label.into(),
                        kind: Some(*Self::LSP_COMPLETION_ITEM_KIND),
                        detail: Some(#detail.into()),
                        ..Default::default()
                    });
                }
            })
        }
    }
}
