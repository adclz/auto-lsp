use crate::features::accessor;
use crate::utilities::extract_fields::EnumFields;
use crate::{BuildAstItem, BuildAstItemBuilder, Paths};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

pub struct EnumBuilder<'a> {
    pub paths: &'a Paths,
    pub fields: &'a EnumFields,
    pub input_name: &'a Ident,
    pub input_builder_name: &'a Ident,
}

impl<'a> EnumBuilder<'a> {
    pub fn new(
        input_name: &'a Ident,
        input_builder_name: &'a Ident,
        fields: &'a EnumFields,
        paths: &'a Paths,
    ) -> Self {
        Self {
            paths,
            fields,
            input_name,
            input_builder_name,
        }
    }
}

impl<'a> ToTokens for EnumBuilder<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.input_name;
        let input_builder_name = &self.input_builder_name;

        let builder_fields = self.generate_builder_fields();
        let builder_new = self.generate_builder_new();
        let query_binder = self.generate_query_binder();
        let add = self.generate_add();
        let try_from = self.generate_try_from();

        let fields = self.generate_fields();
        let ast_item_methods = self.generate_ast_item_methods();

        let code_lens = self.generate_code_lens();
        let completion_items = self.generate_completion_items();
        let document_symbol = self.generate_document_symbol();
        let hover_info = self.generate_hover_info();
        let inlay_hint = self.generate_inlay_hint();
        let semantic_tokens = self.generate_semantic_tokens();
        let scope = self.generate_scope();
        let accessor = self.generate_accessor();

        let ast_item_trait = &self.paths.ast_item_trait;
        let ast_item_builder = &self.paths.ast_item_builder_trait;
        let ast_item_trait_object_arc = &self.paths.ast_item_trait_object_arc;

        let into = quote! {
            fn try_into_item(&self) -> Result<#ast_item_trait_object_arc, lsp_types::Diagnostic> {
                let item = #name::try_from(self.clone())?;
                Ok(std::sync::Arc::new(std::sync::RwLock::new(item)))
            }
        };

        tokens.extend(quote! {
            pub enum #name {
                #(
                    #fields
                )*
            }

            impl #ast_item_trait for #name {
                #ast_item_methods
            }

            pub struct #input_builder_name {
                #(
                    #builder_fields
                )*
            }

            impl #ast_item_builder for #input_builder_name {
                #builder_new
                #query_binder
                #add
                #into

                fn get_url(&self) -> std::sync::Arc<lsp_types::Url> {
                    self.unique_field.borrow().get_url()
                }

                fn get_range(&self) -> tree_sitter::Range {
                    self.unique_field.borrow().get_range()
                }

                fn get_query_index(&self) -> usize {
                    self.unique_field.borrow().get_query_index()
                }
            }

            #try_from

            #code_lens
            #completion_items
            #document_symbol
            #hover_info
            #inlay_hint
            #semantic_tokens
            #scope
            #accessor
        });
    }
}

impl<'a> BuildAstItemBuilder for EnumBuilder<'a> {
    fn generate_builder_fields(&self) -> Vec<TokenStream> {
        let ast_item_builder_trait_object = &self.paths.ast_item_builder_trait_object;
        vec![quote! { pub unique_field: #ast_item_builder_trait_object }]
    }

    fn generate_builder_new(&self) -> TokenStream {
        let variant_types_names = &self.fields.variant_types_names;
        let variant_builder_names = &self.fields.variant_builder_names;

        quote! {
            fn new(url: std::sync::Arc<lsp_types::Url>, query: &tree_sitter::Query, query_index: usize, range: tree_sitter::Range, start_position: tree_sitter::Point, end_position: tree_sitter::Point) -> Self {
                let query_name = query.capture_names()[query_index as usize];
                #(
                    if let true = #variant_types_names::QUERY_NAMES.contains(&query_name) {
                        return Self {
                            unique_field: std::rc::Rc::new(std::cell::RefCell::new(#variant_builder_names::new(
                                url,
                                query,
                                query_index,
                                range,
                                start_position,
                                end_position
                            )))
                        };
                    };
                )*
                panic!("Unexpected")
            }
        }
    }

    fn generate_query_binder(&self) -> TokenStream {
        let ast_item_builder_trait_object = &self.paths.ast_item_builder_trait_object;
        let variant_types = &self.fields.variant_types_names;
        let variant_builders = &self.fields.variant_builder_names;

        quote! {
            fn static_query_binder(url: std::sync::Arc<lsp_types::Url>, capture: &tree_sitter::QueryCapture, query: &tree_sitter::Query) -> Option<#ast_item_builder_trait_object> {
                let query_name = query.capture_names()[capture.index as usize];
                #(
                    if #variant_types::QUERY_NAMES.contains(&query_name)  {
                            return Some(std::rc::Rc::new(std::cell::RefCell::new(#variant_builders::new(
                                url,
                                &query,
                                capture.index as usize,
                                capture.node.range(),
                                capture.node.start_position(),
                                capture.node.end_position(),
                            ))))
                    };
                )*
                None

            }

            fn query_binder(&self, url: std::sync::Arc<lsp_types::Url>, capture: &tree_sitter::QueryCapture, query: &tree_sitter::Query) -> Option<#ast_item_builder_trait_object> {
                self.unique_field.borrow().query_binder(url, capture, query)
            }
        }
    }

    fn generate_add(&self) -> TokenStream {
        let ast_item_builder_trait_object = &self.paths.ast_item_builder_trait_object;
        let defferred_ast_item_builder = &self.paths.deferred_ast_item_builder;

        quote! {
            fn add(&mut self, query: &tree_sitter::Query, node: #ast_item_builder_trait_object, source_code: &[u8]) ->
                Result<#defferred_ast_item_builder, lsp_types::Diagnostic> {
                    self.unique_field.borrow_mut().add(query, node, source_code)
            }
        }
    }

    fn generate_try_from(&self) -> TokenStream {
        let name = self.input_name;
        let input_builder_name = &self.input_builder_name;

        let variant_names = &self.fields.variant_names;
        let variant_builder_names = &self.fields.variant_builder_names;

        quote! {
            impl TryFrom<&#input_builder_name> for #name {
                type Error = lsp_types::Diagnostic;

                fn try_from(builder: &#input_builder_name) -> Result<Self, Self::Error> {
                    use std::sync::{Arc, RwLock};
                    #(
                        if let Some(variant) = builder.unique_field.borrow().downcast_ref::<#variant_builder_names>() {
                            return Ok(Self::#variant_names(variant.try_into()?));
                        };
                    )*
                    panic!("")
                }
            }

            impl TryFrom<&#input_builder_name> for std::sync::Arc<std::sync::RwLock<#name>> {
                type Error = lsp_types::Diagnostic;

                fn try_from(builder: &#input_builder_name) -> Result<Self, Self::Error> {
                    let item = #name::try_from(builder)?;
                    let result = std::sync::Arc::new(std::sync::RwLock::new(item));
                    result.write().unwrap().inject_parent(std::sync::Arc::downgrade(&result) as _);
                    Ok(result)
                }
            }
        }
    }
}

impl<'a> BuildAstItem for EnumBuilder<'a> {
    fn generate_fields(&self) -> Vec<TokenStream> {
        let variant_names = &self.fields.variant_names;
        let variant_types_names = &self.fields.variant_types_names;

        vec![quote! {
            #(
                #variant_names(#variant_types_names)
            ),*
        }]
    }

    fn generate_ast_item_methods(&self) -> TokenStream {
        let variant_names = &self.fields.variant_names;

        quote! {
            fn get_url(&self) -> std::sync::Arc<lsp_types::Url> {
                match self {
                    #(
                        Self::#variant_names(variant) => variant.get_url(),
                    )*
                }
            }

            fn get_range(&self) -> tree_sitter::Range {
                match self {
                    #(
                        Self::#variant_names(variant) => variant.get_range(),
                    )*
                }
            }

            fn get_parent(&self) -> Option<std::sync::Weak<std::sync::RwLock<dyn AstItem>>> {
                match self {
                    #(
                        Self::#variant_names(variant) => variant.get_parent(),
                    )*
                }
            }

            fn set_parent(&mut self, parent: std::sync::Weak<std::sync::RwLock<dyn AstItem>>) {
                match self {
                    #(
                        Self::#variant_names(variant) => variant.set_parent(parent),
                    )*
                }
            }

            fn inject_parent(&mut self, parent: std::sync::Weak<std::sync::RwLock<dyn AstItem>>) {
                match self {
                    #(
                        Self::#variant_names(variant) => variant.inject_parent(parent),
                    )*
                }
            }

            fn find_at_offset(&self, offset: &usize) -> Option<std::sync::Arc<std::sync::RwLock<dyn AstItem>>> {
                match self {
                    #(
                        Self::#variant_names(variant) => variant.find_at_offset(offset),
                    )*
                }
            }

            fn get_start_position(&self, doc: &lsp_textdocument::FullTextDocument) -> lsp_types::Position {
                match self {
                    #(
                        Self::#variant_names(variant) => variant.get_start_position(doc),
                    )*
                }
            }

            fn get_end_position(&self, doc: &lsp_textdocument::FullTextDocument) -> lsp_types::Position {
                match self {
                    #(
                        Self::#variant_names(variant) => variant.get_end_position(doc),
                    )*
                }
            }


            // LSP
            fn swap_at_offset(&mut self, offset: &usize, item: &std::rc::Rc<std::cell::RefCell<dyn AstItemBuilder>>) {
                match self {
                    #(
                        Self::#variant_names(variant) => variant.swap_at_offset(offset, &item),
                    )*
                }
            }
        }
    }
}

impl<'a> EnumBuilder<'a> {
    fn generate_code_lens(&self) -> TokenStream {
        let variant_names = &self.fields.variant_names;
        let input_name = &self.input_name;
        let code_lens_path = &self.paths.code_lens_trait;

        quote! {
            impl #code_lens_path for #input_name {
                fn build_code_lens(&self, acc: &mut Vec<lsp_types::CodeLens>) {
                    match self {
                        #(
                            Self::#variant_names(variant) => variant.build_code_lens(acc),
                        )*
                    }
                }
            }
        }
    }

    fn generate_completion_items(&self) -> TokenStream {
        let variant_names = &self.fields.variant_names;
        let input_name = &self.input_name;
        let completion_items_path = &self.paths.completion_items_trait;

        quote! {
            impl #completion_items_path for #input_name {
                fn build_completion_items(&self, acc: &mut Vec<lsp_types::CompletionItem>, doc: &lsp_textdocument::FullTextDocument) {
                    match self {
                        #(
                            Self::#variant_names(variant) => variant.build_completion_items(acc, doc),
                        )*
                    }
                }
            }
        }
    }

    fn generate_document_symbol(&self) -> TokenStream {
        let variant_names = &self.fields.variant_names;
        let input_name = &self.input_name;
        let document_symbols_path = &self.paths.document_symbols_trait;

        quote! {
            impl #document_symbols_path for #input_name {
                fn get_document_symbols(&self, doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::DocumentSymbol> {
                    match self {
                        #(
                            Self::#variant_names(variant) => variant.get_document_symbols(doc),
                        )*
                    }
                }
            }
        }
    }

    fn generate_hover_info(&self) -> TokenStream {
        let variant_names = &self.fields.variant_names;
        let input_name = &self.input_name;
        let hover_info_path = &self.paths.hover_info_trait;

        quote! {
            impl #hover_info_path for #input_name {
                fn get_hover(&self, doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::Hover> {
                    match self {
                        #(
                            Self::#variant_names(variant) => variant.get_hover(doc),
                        )*
                    }
                }
            }
        }
    }

    fn generate_inlay_hint(&self) -> TokenStream {
        let variant_names = &self.fields.variant_names;
        let input_name = &self.input_name;
        let inlay_hint_path = &self.paths.inlay_hints_trait;

        quote! {
            impl #inlay_hint_path for #input_name {
                fn build_inlay_hint(&self, acc: &mut Vec<lsp_types::InlayHint>) {
                    match self {
                        #(
                            Self::#variant_names(variant) => variant.build_inlay_hint(acc),
                        )*
                    }
                }
            }
        }
    }

    fn generate_semantic_tokens(&self) -> TokenStream {
        let variant_names = &self.fields.variant_names;
        let input_name = &self.input_name;
        let semantic_tokens_path = &self.paths.semantic_tokens_trait;

        quote! {
            impl #semantic_tokens_path for #input_name {
                fn build_semantic_tokens(&self, builder: &mut auto_lsp::builders::semantic_tokens::SemanticTokensBuilder) {
                    match self {
                        #(
                            Self::#variant_names(variant) => variant.build_semantic_tokens(builder),
                        )*
                    }
                }
            }
        }
    }

    fn generate_scope(&self) -> TokenStream {
        let variant_names = &self.fields.variant_names;
        let input_name = &self.input_name;
        let scope_trait = &self.paths.scope_trait;

        quote! {
            impl #scope_trait for #input_name {
                fn is_scope(&self) -> bool {
                    match self {
                        #(
                            Self::#variant_names(variant) => variant.is_scope(),
                        )*
                    }
                }

                fn get_scope_range(&self) -> [usize; 2] {
                    match self {
                        #(
                            Self::#variant_names(variant) => variant.get_scope_range(),
                        )*
                    }
                }
            }
        }
    }

    fn generate_accessor(&self) -> TokenStream {
        let variant_names = &self.fields.variant_names;
        let input_name = &self.input_name;
        let is_accessor_trait = &self.paths.is_accessor_trait;
        let accessor_trait = &self.paths.accessor_trait;

        quote! {
        impl #is_accessor_trait for #input_name {
            fn is_accessor(&self) -> &'static bool {
                match self {
                    #(
                        Self::#variant_names(variant) => variant.is_accessor(),
                    )*
                }
            }
        }

        impl #accessor_trait for #input_name {
            fn find(&self, doc: &lsp_textdocument::FullTextDocument, ctx: &dyn auto_lsp::traits::workspace::WorkspaceContext) {
                    match self {
                        #(
                            Self::#variant_names(variant) => variant.find(doc, ctx),
                        )*
                    }
                }
            }
        }
    }
}
