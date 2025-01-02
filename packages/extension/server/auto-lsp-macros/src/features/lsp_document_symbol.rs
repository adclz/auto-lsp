extern crate proc_macro;

use crate::{
    utilities::{extract_fields::StructFields, format_tokens::path_to_dot_tokens},
    AccessorFeatures, FeaturesCodeGen, ReferenceFeature, SymbolFeatures, PATHS,
};
use darling::{util::PathList, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Path};

use crate::Feature;

#[derive(Debug, FromMeta)]
pub struct DocumentSymbolFeature {
    pub kind: Path,
    pub name: Path,
    pub childrens: Option<PathList>,
}

pub struct DocumentSymbolBuilder<'a> {
    pub input_name: &'a Ident,
    pub fields: &'a StructFields,
}

impl<'a> DocumentSymbolBuilder<'a> {
    pub fn new(input_name: &'a Ident, fields: &'a StructFields) -> Self {
        Self { input_name, fields }
    }

    pub fn default_impl(&self) -> TokenStream {
        let input_name = &self.input_name;
        let document_symbols_path = &PATHS.lsp_document_symbols.path;

        quote! {
            impl #document_symbols_path for #input_name { }
        }
    }
}

impl<'a> FeaturesCodeGen for DocumentSymbolBuilder<'a> {
    fn code_gen(&self, params: &SymbolFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let document_symbols_path = &PATHS.lsp_document_symbols.path;
        let sig = &PATHS.lsp_document_symbols.methods.get_document_symbols.sig;

        match &params.lsp_document_symbols {
            None => self.default_impl(),
            Some(params) => match params {
                Feature::User => quote! {},
                Feature::CodeGen(strategy) => {
                    let kind = &strategy.kind;
                    let name = path_to_dot_tokens(&strategy.name, None);

                    let children_tokens =
                        strategy.childrens.as_ref().map(|vec| {
                            vec.iter().map(|path| {
                            let path_tokens = path_to_dot_tokens(path, None);
                            quote! {
                                #path_tokens
                                    .iter()
                                    .filter_map(|child| child.read().get_document_symbols(doc))
                                    .collect::<Vec<_>>()
                            }
                        }).collect::<Vec<_>>()
                        });

                    let children = if let Some(tokens) = children_tokens {
                        quote! {
                            Some(
                                vec![#(#tokens),*]
                                    .into_iter()
                                    .flatten()
                                    .collect::<Vec<_>>()
                            )
                        }
                    } else {
                        quote! { None }
                    };

                    quote! {
                        impl #document_symbols_path for #input_name {
                            #[allow(deprecated)]
                            #sig {
                                let read = #name.read();

                                let name = read.get_text(doc.get_content(None).as_bytes()).to_string();
                                if name.is_empty() {
                                    return None
                                }

                                Some(lsp_types::DocumentSymbol {
                                    name,
                                    detail: None,
                                    kind: #kind,
                                    tags: None,
                                    deprecated: None,
                                    range: self.get_lsp_range(doc),
                                    selection_range: read.get_lsp_range(doc),
                                    children: #children
                                })
                            }
                        }
                    }
                }
            },
        }
    }

    fn code_gen_accessor(&self, params: &AccessorFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let document_symbols_path = &PATHS.lsp_document_symbols.path;
        let sig = &PATHS.lsp_document_symbols.methods.get_document_symbols.sig;

        match &params.lsp_document_symbols {
            None => self.default_impl(),
            Some(feature) => match feature {
                ReferenceFeature::Disable => self.default_impl(),
                ReferenceFeature::Reference => {
                    quote! {
                        impl #document_symbols_path for #input_name {
                            #sig {
                                if let Some(accessor) = &self.get_target() {
                                    if let Some(accessor) = accessor.to_dyn() {
                                        return accessor.read().get_document_symbols(doc)
                                    }
                                }
                                None
                            }
                        }
                    }
                }
                ReferenceFeature::User => quote! {},
            },
        }
    }
}
