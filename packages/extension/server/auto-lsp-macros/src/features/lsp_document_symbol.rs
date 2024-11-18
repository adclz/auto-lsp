extern crate proc_macro;

use crate::{
    utilities::{extract_fields::StructFields, format_tokens::path_to_dot_tokens}, CodeGen, Paths, ToCodeGen
};
use darling::{util::PathList, FromMeta};
use quote::quote;
use syn::{Ident, Path};

use crate::Feature;

#[derive(Debug, FromMeta)]
pub struct DocumentSymbolFeature {
    pub kind: Path,
    pub name: Path,
    pub childrens: Option<Childrens>,
}

#[derive(Debug, FromMeta)]
pub struct Childrens {
    pub vec: Option<PathList>,
    pub map: Option<PathList>,
}

pub struct DocumentSymbolBuilder<'a> {
    pub input_name: &'a Ident,
    pub paths: &'a Paths,
    pub params: Option<&'a Feature<DocumentSymbolFeature>>,
    pub fields: &'a StructFields,
}

impl<'a> DocumentSymbolBuilder<'a> {
    pub fn new(
        input_name: &'a Ident,
        paths: &'a Paths,
        params: Option<&'a Feature<DocumentSymbolFeature>>,
        fields: &'a StructFields,
    ) -> Self {
        Self {
            paths,
            input_name,
            params,
            fields
         }
    }
}

impl<'a> ToCodeGen for DocumentSymbolBuilder<'a> {
    fn to_code_gen(&self, codegen: &mut CodeGen) {
        let input_name = &self.input_name;
        let document_symbols_path = &self.paths.document_symbols_trait_path;

        match self.params {
            None => codegen.input.other_impl.push(quote! {
                impl #document_symbols_path for #input_name {
                    fn get_document_symbols(&self, _doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::DocumentSymbol> {
                        None
                    }
                }
            }),
            Some(params) => match params {
                Feature::User => (),
                Feature::CodeGen(strategy) => {
                    let kind = &strategy.kind;
                    let name = path_to_dot_tokens(&strategy.name, None);
                
                    let mut vec_tokens = None;
                    let mut map_tokens = None;
                
                    match &strategy.childrens {
                        None => {}
                        Some(paths) => {
                            if let Some(vec) = &paths.vec {
                                let children_tokens = vec.iter().map(|path| {
                                    let path_tokens = path_to_dot_tokens(path, None);
                                    quote! {
                                        #path_tokens
                                            .iter()
                                            .filter_map(|child| child.read().unwrap().get_document_symbols(doc))
                                            .collect::<Vec<_>>()
                                    }
                                });
                
                                vec_tokens = Some(quote! {
                                    Some(
                                        vec![#(#children_tokens),*]
                                            .into_iter()
                                            .flatten()
                                            .collect::<Vec<_>>()
                                    )
                                })
                            };
                
                            if let Some(map) = &paths.map {
                                let children_tokens = map.iter().map(|path| {
                                    let path_tokens = path_to_dot_tokens(path, None);
                                    quote! {
                                        #path_tokens
                                            .values()
                                            .cloned()
                                            .filter_map(|child| child.read().unwrap().get_document_symbols(doc))
                                            .collect::<Vec<_>>()
                                    }
                                });
                
                                map_tokens = Some(quote! {
                                    Some(
                                        vec![#(#children_tokens),*]
                                            .into_iter()
                                            .flatten()
                                            .collect::<Vec<_>>()
                                    )
                                })
                            };
                        }
                    };
                
                    let children = if let (false, false) = (vec_tokens.is_some(), map_tokens.is_some()) {
                        quote! { None }
                    } else {
                        quote! {
                            #vec_tokens
                            #map_tokens
                        }
                    };
                
                    codegen.input.impl_base.push(
                        quote! {
                            const LSP_SYMBOL_KIND: &'static lsp_types::SymbolKind = &#kind;
                        }
                        .into(),
                    );
                
                    codegen.input.other_impl.push(
                        quote! {
                            impl #document_symbols_path for #input_name {
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
                        }
                        .into()
                    );
                }
            },
        }
    }
}
