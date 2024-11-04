extern crate proc_macro;

use darling::FromMeta;
use quote::quote;
use syn::{Path, TypeTuple};

use crate::{utilities::format_tokens::path_to_dot_tokens, CodeGen, Features};

#[derive(Debug, FromMeta)]
pub struct HoverFeature {
    call: Path,
}

pub fn generate_hover_info_feature(features: &Features, code_gen: &mut CodeGen) {
    if let Some(hover) = &features.lsp_hover {
        codegen_hover_info(hover, code_gen);
    }
}

fn codegen_hover_info(path: &HoverFeature, code_gen: &mut CodeGen) {
    let call = path_to_dot_tokens(&path.call, None);

    code_gen.impl_ast_item.push(quote! {
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
