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

        let locator = self.generate_locator();
        let parent = self.generate_parent();

        let ast_item_trait = &self.paths.ast_item_trait;
        let ast_item_builder = &self.paths.ast_item_builder_trait;

        let dyn_symbol = &self.paths.dyn_symbol;

        let try_from_builder = &self.paths.try_from_builder;

        let into = quote! {
            fn try_to_dyn_symbol(&self, check: &mut Vec<#dyn_symbol>) -> Result<#dyn_symbol, lsp_types::Diagnostic> {
                use #try_from_builder;

                let item = #name::try_from_builder(self, check)?;
                Ok(#dyn_symbol::new(item))
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
                    self.unique_field.get_rc().borrow().get_url()
                }

                fn get_range(&self) -> tree_sitter::Range {
                    self.unique_field.get_rc().borrow().get_range()
                }

                fn get_query_index(&self) -> usize {
                    self.unique_field.get_rc().borrow().get_query_index()
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
            #locator
            #parent
        });
    }
}

impl<'a> BuildAstItemBuilder for EnumBuilder<'a> {
    fn generate_builder_fields(&self) -> Vec<TokenStream> {
        let pending_symbol: &syn::Path = &self.paths.pending_symbol;
        vec![quote! { pub unique_field: #pending_symbol }]
    }

    fn generate_builder_new(&self) -> TokenStream {
        let pending_symbol = &self.paths.pending_symbol;

        let variant_types_names = &self.fields.variant_types_names;
        let variant_builder_names = &self.fields.variant_builder_names;

        quote! {
            fn new(url: std::sync::Arc<lsp_types::Url>, query: &tree_sitter::Query, query_index: usize, range: tree_sitter::Range, start_position: tree_sitter::Point, end_position: tree_sitter::Point) -> Option<Self> {
                let query_name = query.capture_names()[query_index as usize];
                #(
                    if #variant_types_names::QUERY_NAMES.contains(&query_name) {
                        match #variant_builder_names::new(url, query, query_index, range, start_position, end_position) {
                            Some(builder) => return Some(Self {
                                unique_field: #pending_symbol::new(builder),
                            }),
                            None => return None,
                        }
                    };
                )*
                None
            }
        }
    }

    fn generate_query_binder(&self) -> TokenStream {
        let maybe_pending_symbol = &self.paths.maybe_pending_symbol;

        quote! {
            fn query_binder(&self, url: std::sync::Arc<lsp_types::Url>, capture: &tree_sitter::QueryCapture, query: &tree_sitter::Query) -> #maybe_pending_symbol {
                self.unique_field.get_rc().borrow().query_binder(url, capture, query)
            }
        }
    }

    fn generate_add(&self) -> TokenStream {
        let pending_symbol = &self.paths.pending_symbol;

        quote! {
            fn add(&mut self, query: &tree_sitter::Query, node: #pending_symbol, source_code: &[u8]) ->
                Result<(), lsp_types::Diagnostic> {
                    self.unique_field.get_rc().borrow_mut().add(query, node, source_code)
            }
        }
    }

    fn generate_try_from(&self) -> TokenStream {
        let name = self.input_name;
        let input_builder_name = &self.input_builder_name;

        let variant_names = &self.fields.variant_names;
        let variant_builder_names = &self.fields.variant_builder_names;

        let try_from_builder = &self.paths.try_from_builder;
        let try_into_builder = &self.paths.try_into_builder;

        let symbol = &self.paths.symbol;
        let dyn_symbol = &self.paths.dyn_symbol;

        quote! {
            impl #try_from_builder<&#input_builder_name> for #name {
                type Error = lsp_types::Diagnostic;

                fn try_from_builder(builder: &#input_builder_name, check: &mut Vec<#dyn_symbol>) -> Result<Self, Self::Error> {
                    use #try_into_builder;

                    #(
                        if let Some(variant) = builder.unique_field.get_rc().borrow().downcast_ref::<#variant_builder_names>() {
                            return Ok(Self::#variant_names(variant.try_into_builder(check)?));
                        };
                    )*
                    panic!("")
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
        let dyn_symbol = &self.paths.dyn_symbol;
        let weak_symbol = &self.paths.weak_symbol;

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

            fn get_parent(&self) -> Option<#weak_symbol> {
                match self {
                    #(
                        Self::#variant_names(variant) => variant.get_parent(),
                    )*
                }
            }

            fn set_parent(&mut self, parent: #weak_symbol) {
                match self {
                    #(
                        Self::#variant_names(variant) => variant.set_parent(parent),
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
        }
    }
}

impl<'a> EnumBuilder<'a> {
    fn generate_locator(&self) -> TokenStream {
        let variant_names = &self.fields.variant_names;
        let input_name = &self.input_name;
        let dyn_symbol = &self.paths.dyn_symbol;
        let locator = &self.paths.locator;

        quote! {
            impl #locator for #input_name {
                fn find_at_offset(&self, offset: usize) -> Option<#dyn_symbol> {
                    match self {
                        #(
                            Self::#variant_names(variant) => variant.find_at_offset(offset),
                        )*
                    }
                }
            }
        }
    }
}

impl<'a> EnumBuilder<'a> {
    fn generate_parent(&self) -> TokenStream {
        let variant_names = &self.fields.variant_names;
        let input_name = &self.input_name;
        let weak_symbol = &self.paths.weak_symbol;
        let parent = &self.paths.parent;

        quote! {
            impl #parent for #input_name {
                fn inject_parent(&mut self, parent: #weak_symbol) {
                    match self {
                        #(
                            Self::#variant_names(variant) => variant.inject_parent(parent),
                        )*
                    }
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
                fn build_inlay_hint(&self, doc: &lsp_textdocument::FullTextDocument, acc: &mut Vec<lsp_types::InlayHint>) {
                    match self {
                        #(
                            Self::#variant_names(variant) => variant.build_inlay_hint(doc, acc),
                        )*
                    }
                }
            }
        }
    }

    fn generate_semantic_tokens(&self) -> TokenStream {
        let variant_names = &self.fields.variant_names;
        let input_name = &self.input_name;
        let semangic_tokens_builder = &self.paths.semantic_tokens_builder;
        let semantic_tokens_path = &self.paths.semantic_tokens_trait;

        quote! {
            impl #semantic_tokens_path for #input_name {
                fn build_semantic_tokens(&self, builder: &mut #semangic_tokens_builder) {
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

                fn get_scope_range(&self) -> Vec<[usize; 2]> {
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
            fn find(&self, doc: &lsp_textdocument::FullTextDocument, ctx: &dyn auto_lsp::workspace::WorkspaceContext) -> Result<(), lsp_types::Diagnostic> {
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
