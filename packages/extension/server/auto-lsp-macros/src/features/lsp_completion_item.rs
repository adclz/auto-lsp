extern crate proc_macro;

use crate::{utilities::format_tokens::path_to_dot_tokens, Features, FeaturesCodeGen};
use darling::{util::PathList, FromMeta};
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

pub fn generate_completion_item_feature(
    features: &Features,
    code_gen_impl: &mut Vec<proc_macro2::TokenStream>,
    code_gen_impl_ast_item: &mut Vec<proc_macro2::TokenStream>,
) {
    if let Some(completion_item) = &features.lsp_completion_item {
        let code_gen = codegen_completion_item(&completion_item);
        if code_gen.impl_base.is_some() {
            code_gen_impl.push(code_gen.impl_base.unwrap());
        }
        code_gen_impl_ast_item.push(code_gen.impl_ast_item.unwrap())
    }
}

pub fn codegen_completion_item(completion: &CompletionItemFeature) -> FeaturesCodeGen {
    match completion {
        CompletionItemFeature::CompletionFn(completion_fn) => {
            let completion_fn = path_to_dot_tokens(completion_fn, None);
            FeaturesCodeGen {
                fields: Some(vec![]),
                impl_base: None,
                impl_ast_item: Some(quote! {
                    fn build_completion_items(&self, acc: &mut Vec<lsp_types::CompletionItem>) {
                        #completion_fn(acc)
                    }
                }),
            }
        }
        CompletionItemFeature::Item(item) => {
            let kind = &item.kind;
            let label = &item.label;
            let detail = &item.detail;

            FeaturesCodeGen {
                fields: Some(vec![]),
                impl_base: Some(quote! {
                    const LSP_COMPLETION_ITEM_KIND: &'static lsp_types::CompletionItemKind = &#kind;
                }),
                impl_ast_item: Some(quote! {
                    fn build_completion_items(&self, acc: &mut Vec<lsp_types::CompletionItem>) {
                        acc.push(lsp_types::CompletionItem {
                            label: #label.into(),
                            kind: Some(*Self::LSP_COMPLETION_ITEM_KIND),
                            detail: Some(#detail.into()),
                            ..Default::default()
                        });
                    }
                }),
            }
        }
    }
}
