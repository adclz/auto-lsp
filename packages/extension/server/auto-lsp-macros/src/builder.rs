use crate::{
    features::{
        lsp_code_lens::CodeLensBuilder, lsp_completion_item::CompletionItemsBuilder,
        lsp_document_symbol::DocumentSymbolBuilder, lsp_hover_info::HoverInfoBuilder,
        lsp_inlay_hint::InlayHintsBuilder, lsp_semantic_token::SemanticTokensBuilder,
    },
    traits::ast_builder::for_struct::AstItemBuilder,
    utilities::extract_fields::StructFields,
    CodeGen, Paths, SymbolFeatures,
};
use proc_macro2::Ident;
use quote::{format_ident, quote, ToTokens};
use syn::DeriveInput;

pub trait ToCodeGen {
    fn to_code_gen(&self, codegen: &mut CodeGen);
}

pub struct StructSymbolBuilder<'a> {
    // Input data
    pub input: &'a DeriveInput,
    pub input_name: &'a Ident,
    pub input_buider_name: Ident,
    pub params: &'a SymbolFeatures,
    pub struct_fields: &'a StructFields,
    // Paths
    pub paths: &'a Paths,
    // Features
    pub lsp_code_lens: CodeLensBuilder<'a>,
    pub lsp_completion_items: CompletionItemsBuilder<'a>,
    pub lsp_document_symbols: DocumentSymbolBuilder<'a>,
    pub lsp_hover_info: HoverInfoBuilder<'a>,
    pub lsp_inlay_hints: InlayHintsBuilder<'a>,
    pub lsp_semantic_tokens: SemanticTokensBuilder<'a>,
    // Item
    pub ast_item: AstItemBuilder<'a>,
}

impl<'a> StructSymbolBuilder<'a> {
    pub fn new(
        input: &'a DeriveInput,
        params: &'a SymbolFeatures,
        query_name: &'a str,
        fields: &'a StructFields,
        paths: &'a Paths,
    ) -> Self {
        Self {
            input,
            input_name: &input.ident,
            input_buider_name: format_ident!("{}Builder", input.ident),
            params,
            struct_fields: fields,
            paths,
            lsp_code_lens: CodeLensBuilder::new(params.lsp_code_lens.as_ref(), fields),
            lsp_completion_items: CompletionItemsBuilder::new(
                params.lsp_completion_items.as_ref(),
                fields,
            ),
            lsp_document_symbols: DocumentSymbolBuilder::new(
                params.lsp_document_symbols.as_ref(),
                fields,
            ),
            lsp_hover_info: HoverInfoBuilder::new(params.lsp_hover_info.as_ref(), fields),
            lsp_inlay_hints: InlayHintsBuilder::new(params.lsp_inlay_hints.as_ref(), fields),
            lsp_semantic_tokens: SemanticTokensBuilder::new(
                params.lsp_semantic_tokens.as_ref(),
                fields,
            ),
            ast_item: AstItemBuilder::new(
                paths,
                query_name,
                &input.ident,
                format_ident!("{}Builder", input.ident),
                fields,
            ),
        }
    }
}

impl<'a> ToTokens for StructSymbolBuilder<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut code_gen = CodeGen::default();

        let ast_item_trait = &self.paths.ast_item_trait_path;

        // Generate ast item
        self.ast_item.to_code_gen(&mut code_gen);

        // Generate features
        self.lsp_code_lens.to_code_gen(&mut code_gen);
        self.lsp_completion_items.to_code_gen(&mut code_gen);
        self.lsp_document_symbols.to_code_gen(&mut code_gen);
        self.lsp_hover_info.to_code_gen(&mut code_gen);
        self.lsp_inlay_hints.to_code_gen(&mut code_gen);
        self.lsp_semantic_tokens.to_code_gen(&mut code_gen);

        let input_name = &self.input.ident;

        let input_fields = code_gen.input.fields;
        let input_methods = code_gen.input.impl_base;
        let input_ast_item_methods = code_gen.input.impl_ast_item;

        let others_tokens = code_gen.new_structs;

        tokens.extend(quote! {
            #[derive(Clone)]
            pub struct #input_name {
                #(#input_fields),*
            }

            impl #input_name {
                #(#input_methods)*
            }

            impl #ast_item_trait for #input_name {
                #(#input_ast_item_methods)*
            }

            #(#others_tokens)*
        });
    }
}
