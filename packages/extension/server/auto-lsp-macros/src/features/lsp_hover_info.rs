extern crate proc_macro;

use darling::FromMeta;
use quote::quote;
use syn::{Path, TypeTuple};

use crate::{utilities::format_tokens::path_to_dot_tokens, Features, FeaturesCodeGen};

#[derive(Debug, FromMeta)]
pub struct HoverFeature {
    call: Path,
}

pub fn generate_hover_info_feature(
    features: &Features,
    code_gen_impl: &mut Vec<proc_macro2::TokenStream>,
    code_gen_impl_ast_item: &mut Vec<proc_macro2::TokenStream>,
) {
    if let Some(hover) = &features.lsp_hover {
        let code_gen = codegen_hover_info(&hover.call);
        code_gen_impl_ast_item.push(code_gen.impl_ast_item.unwrap())
    }
}

fn codegen_hover_info(path: &Path) -> FeaturesCodeGen {
    let call = path_to_dot_tokens(&path, None);

    FeaturesCodeGen {
        fields: None,
        impl_base: None,
        impl_ast_item: quote! {
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
        }
        .into(),
    }
}
