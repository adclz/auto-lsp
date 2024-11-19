use crate::{
    features::{
        lsp_code_lens::CodeLensBuilder, lsp_completion_item::CompletionItemsBuilder,
        lsp_document_symbol::DocumentSymbolBuilder, lsp_hover_info::HoverInfoBuilder,
        lsp_inlay_hint::InlayHintsBuilder, lsp_semantic_token::SemanticTokensBuilder,
    },
    utilities::extract_fields::StructFields,
    Paths, SymbolFeatures,
};
use proc_macro2::Ident;

pub trait ToCodeGen {
    fn to_code_gen(&self, codegen: &mut FeaturesCodeGen);
}

#[derive(Default)]
pub struct InputCodeGen {
    pub fields: Vec<proc_macro2::TokenStream>,        // Fields
    pub impl_base: Vec<proc_macro2::TokenStream>,     // Impl <>
    pub impl_ast_item: Vec<proc_macro2::TokenStream>, // Impl AstItem for <>
    pub other_impl: Vec<proc_macro2::TokenStream>,    // Other impl
}

#[derive(Default)]
pub struct FeaturesCodeGen {
    pub input: InputCodeGen,
}

pub struct Features<'a> {
    pub lsp_code_lens: CodeLensBuilder<'a>,
    pub lsp_completion_items: CompletionItemsBuilder<'a>,
    pub lsp_document_symbols: DocumentSymbolBuilder<'a>,
    pub lsp_hover_info: HoverInfoBuilder<'a>,
    pub lsp_inlay_hints: InlayHintsBuilder<'a>,
    pub lsp_semantic_tokens: SemanticTokensBuilder<'a>,
}

impl<'a> Features<'a> {
    pub fn new(
        params: Option<&'a SymbolFeatures>,
        input_name: &'a Ident,
        paths: &'a Paths,
        fields: &'a StructFields,
    ) -> Self {
        Self {
            lsp_code_lens: CodeLensBuilder::new(
                input_name,
                paths,
                params.and_then(|a| a.lsp_code_lens.as_ref()),
                fields,
            ),
            lsp_completion_items: CompletionItemsBuilder::new(
                input_name,
                paths,
                params.and_then(|a| a.lsp_completion_items.as_ref()),
                fields,
            ),
            lsp_document_symbols: DocumentSymbolBuilder::new(
                input_name,
                paths,
                params.and_then(|a| a.lsp_document_symbols.as_ref()),
                fields,
            ),
            lsp_hover_info: HoverInfoBuilder::new(
                input_name,
                paths,
                params.and_then(|a| a.lsp_hover_info.as_ref()),
                fields,
            ),
            lsp_inlay_hints: InlayHintsBuilder::new(
                input_name,
                paths,
                params.and_then(|a| a.lsp_inlay_hints.as_ref()),
                fields,
            ),
            lsp_semantic_tokens: SemanticTokensBuilder::new(
                input_name,
                paths,
                params.and_then(|a| a.lsp_semantic_tokens.as_ref()),
                fields,
            ),
        }
    }
}

impl<'a> ToCodeGen for Features<'a> {
    fn to_code_gen(&self, codegen: &mut FeaturesCodeGen) {
        self.lsp_code_lens.to_code_gen(codegen);
        self.lsp_completion_items.to_code_gen(codegen);
        self.lsp_document_symbols.to_code_gen(codegen);
        self.lsp_hover_info.to_code_gen(codegen);
        self.lsp_inlay_hints.to_code_gen(codegen);
        self.lsp_semantic_tokens.to_code_gen(codegen);
    }
}
