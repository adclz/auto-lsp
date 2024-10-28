extern crate proc_macro;

use crate::{utilities::format_tokens::path_to_dot_tokens, Features, FeaturesCodeGen};
use darling::{util::PathList, FromMeta};
use quote::quote;
use syn::Path;

#[derive(Debug, FromMeta)]
pub struct DocumentSymbolFeature {
    pub kind: Path,
    pub strategy: DocumentSymbolStrategy,
}

#[derive(Debug, FromMeta)]
pub struct DocumentSymbolStrategy {
    pub name: Path,
    pub childrens: Option<PathList>,
}

pub fn generate_document_symbol_feature(
    features: &Features,
    code_gen_impl: &mut Vec<proc_macro2::TokenStream>,
    code_gen_impl_ast_item: &mut Vec<proc_macro2::TokenStream>,
) {
    if let Some(document_symbol) = &features.lsp_document_symbols {
        let code_gen = codegen_document_symbol(&document_symbol.kind, &document_symbol.strategy);
        code_gen_impl.push(code_gen.impl_base.unwrap());
        code_gen_impl_ast_item.push(code_gen.impl_ast_item.unwrap())
    }
}

pub fn codegen_document_symbol(kind: &Path, strategy: &DocumentSymbolStrategy) -> FeaturesCodeGen {
    let name = path_to_dot_tokens(&strategy.name, None);

    let children = match &strategy.childrens {
        None => quote! { None },
        Some(paths) => {
            let children_tokens = paths.iter().map(|path| {
                let path_tokens = path_to_dot_tokens(path, None);
                quote! {
                    #path_tokens
                        .iter()
                        .filter_map(|child| child.read().unwrap().get_document_symbols(doc))
                        .collect::<Vec<_>>()
                }
            });

            quote! {
                Some(
                    vec![#(#children_tokens),*]
                        .into_iter()
                        .flatten()
                        .collect::<Vec<_>>()
                )
            }
        }
    };

    FeaturesCodeGen {
        fields: None,
        impl_base: Some(quote! {
            const LSP_SYMBOL_KIND: &'static lsp_types::SymbolKind = &#kind;
        }),
        impl_ast_item: Some(
            quote! {
                #[allow(deprecated)]
                fn get_document_symbols(&self, doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::DocumentSymbol> {
                    let read = #name.read().unwrap();

                    Some(lsp_types::DocumentSymbol {
                        name: read.get_text(doc.get_content(None).as_bytes()).to_string(),
                        detail: None,
                        kind: *Self::LSP_SYMBOL_KIND,
                        tags: None,
                        deprecated: None,
                        range: self.get_lsp_range(doc),
                        selection_range: read.get_lsp_range(doc),
                        children: #children
                    })
                }
            }
            .into(),
        ),
    }
}
