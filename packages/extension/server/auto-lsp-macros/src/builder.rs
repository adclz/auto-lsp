use syn::DeriveInput;

use crate::{
    features::{
        lsp_code_lens::CodeLensBuilder, lsp_completion_item::CompletionItemsBuilder,
        lsp_document_symbol::DocumentSymbolBuilder, lsp_hover_info::HoverInfoBuilder,
        lsp_inlay_hint::InlayHintsBuilder, lsp_semantic_token::SemanticTokensBuilder,
    },
    utilities::extract_fields::StructFields,
    CodeGen, SymbolFeatures,
};
use quote::{quote, ToTokens};

pub trait ToCodeGen {
    fn to_code_gen(&self, codegen: &mut CodeGen);
}

pub struct StructSymbolBuilder<'a> {
    // Input data
    pub input: &'a mut DeriveInput,
    pub params: &'a SymbolFeatures,
    pub struct_fields: &'a StructFields,
    // Features
    pub lsp_code_lens: CodeLensBuilder<'a>,
    pub lsp_completion_items: CompletionItemsBuilder<'a>,
    pub lsp_document_symbols: DocumentSymbolBuilder<'a>,
    pub lsp_hover_info: HoverInfoBuilder<'a>,
    pub lsp_inlay_hints: InlayHintsBuilder<'a>,
    pub lsp_semantic_tokens: SemanticTokensBuilder<'a>,
}

impl<'a> StructSymbolBuilder<'a> {
    pub fn new(
        input: &'a mut DeriveInput,
        params: &'a SymbolFeatures,
        fields: &'a StructFields,
    ) -> Self {
        Self {
            input,
            params,
            struct_fields: fields,
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
        }
    }
}

impl<'a> ToTokens for StructSymbolBuilder<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut code_gen = CodeGen::default();

        // Generate features
        self.lsp_code_lens.to_code_gen(&mut code_gen);
        self.lsp_completion_items.to_code_gen(&mut code_gen);
        self.lsp_document_symbols.to_code_gen(&mut code_gen);
        self.lsp_hover_info.to_code_gen(&mut code_gen);
        self.lsp_inlay_hints.to_code_gen(&mut code_gen);
        self.lsp_semantic_tokens.to_code_gen(&mut code_gen);

        let base_methods = code_gen.impl_base;
        let ast_item_methods = code_gen.impl_ast_item;
        let input = &self.input;
        let input_name = &self.input.ident;

        tokens.extend(quote! {
            #[derive(Clone)]
            #input

            impl #input_name {
                #(#base_methods)*
            }

            impl AstItem for #input_name {
                #(#ast_item_methods)*
            }
        });
    }
}
