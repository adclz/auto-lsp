use crate::{
    features::{
        accessor::AccessorBuilder, duplicate::CheckDuplicateBuilder,
        lsp_code_lens::CodeLensBuilder, lsp_completion_item::CompletionItemsBuilder,
        lsp_document_symbol::DocumentSymbolBuilder, lsp_go_to_definition::GotoDefinitionBuilder,
        lsp_hover_info::HoverInfoBuilder, lsp_inlay_hint::InlayHintsBuilder,
        lsp_semantic_token::SemanticTokensBuilder, scope::ScopeBuilder,
    },
    utilities::extract_fields::StructFields,
    Paths, StructHelpers, SymbolFeatures,
};
use darling::{ast, util};
use proc_macro2::Ident;

pub trait ToCodeGen {
    fn to_code_gen(&self, codegen: &mut FeaturesCodeGen);
}

#[derive(Default)]
pub struct InputCodeGen {
    pub fields: Vec<proc_macro2::TokenStream>,      // Fields
    pub impl_base: Vec<proc_macro2::TokenStream>,   // Impl <>
    pub impl_symbol: Vec<proc_macro2::TokenStream>, // Impl AstSymbol for <>
    pub other_impl: Vec<proc_macro2::TokenStream>,  // Other impl
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
    pub lsp_go_to_definition: GotoDefinitionBuilder<'a>,
    pub scope: ScopeBuilder<'a>,
    pub accessor: AccessorBuilder<'a>,
    pub duplicate: CheckDuplicateBuilder<'a>,
}

impl<'a> Features<'a> {
    pub fn new(
        features_attributes: Option<&'a SymbolFeatures>,
        helper_attributes: &'a ast::Data<util::Ignored, StructHelpers>,
        is_accessor: bool,
        input_name: &'a Ident,
        paths: &'a Paths,
        fields: &'a StructFields,
    ) -> Self {
        Self {
            lsp_code_lens: CodeLensBuilder::new(
                input_name,
                features_attributes.and_then(|a| a.lsp_code_lens.as_ref()),
                fields,
                is_accessor,
            ),
            lsp_completion_items: CompletionItemsBuilder::new(
                input_name,
                features_attributes.and_then(|a| a.lsp_completion_items.as_ref()),
                fields,
                is_accessor,
            ),
            lsp_document_symbols: DocumentSymbolBuilder::new(
                input_name,
                features_attributes.and_then(|a| a.lsp_document_symbols.as_ref()),
                fields,
                is_accessor,
            ),
            lsp_hover_info: HoverInfoBuilder::new(
                input_name,
                features_attributes.and_then(|a| a.lsp_hover_info.as_ref()),
                fields,
                is_accessor,
            ),
            lsp_inlay_hints: InlayHintsBuilder::new(
                input_name,
                features_attributes.and_then(|a| a.lsp_inlay_hints.as_ref()),
                fields,
                is_accessor,
            ),
            lsp_semantic_tokens: SemanticTokensBuilder::new(
                input_name,
                features_attributes.and_then(|a| a.lsp_semantic_tokens.as_ref()),
                fields,
                is_accessor,
            ),
            lsp_go_to_definition: GotoDefinitionBuilder::new(
                input_name,
                features_attributes.and_then(|a| a.lsp_go_to_definition.as_ref()),
                fields,
                is_accessor,
            ),
            scope: ScopeBuilder::new(
                input_name,
                paths,
                features_attributes.and_then(|a| a.scope.as_ref()),
                fields,
            ),
            accessor: AccessorBuilder::new(input_name, is_accessor, paths, fields),
            duplicate: CheckDuplicateBuilder::new(input_name, paths, helper_attributes, fields),
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
        self.lsp_go_to_definition.to_code_gen(codegen);
        self.scope.to_code_gen(codegen);
        self.accessor.to_code_gen(codegen);
        self.duplicate.to_code_gen(codegen);
    }
}
