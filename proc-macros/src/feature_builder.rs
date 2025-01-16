use crate::{
    features::{
        check::CheckBuilder, comment::CommentBuilder, lsp_code_lens::CodeLensBuilder,
        lsp_completion_item::CompletionItemsBuilder, lsp_document_symbol::DocumentSymbolBuilder,
        lsp_go_to_declaration::GoToDeclarationBuilder, lsp_go_to_definition::GotoDefinitionBuilder,
        lsp_hover_info::HoverInfoBuilder, lsp_inlay_hint::InlayHintsBuilder,
        lsp_semantic_token::SemanticTokensBuilder, reference::ReferenceBuilder,
        scope::ScopeBuilder,
    },
    utilities::extract_fields::StructFields,
    ReferenceFeatures, ReferenceOrSymbolFeatures, StructHelpers, SymbolFeatures,
};
use darling::{ast, util};
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;

pub trait FeaturesCodeGen {
    fn code_gen(&self, params: &SymbolFeatures) -> impl quote::ToTokens;
    fn code_gen_accessor(&self, params: &ReferenceFeatures) -> impl quote::ToTokens;
}

pub struct Features<'a> {
    pub features_attributes: &'a ReferenceOrSymbolFeatures<'a>,
    pub lsp_code_lens: CodeLensBuilder<'a>,
    pub lsp_completion_items: CompletionItemsBuilder<'a>,
    pub lsp_document_symbols: DocumentSymbolBuilder<'a>,
    pub lsp_hover_info: HoverInfoBuilder<'a>,
    pub lsp_inlay_hints: InlayHintsBuilder<'a>,
    pub lsp_semantic_tokens: SemanticTokensBuilder<'a>,
    pub lsp_go_to_definition: GotoDefinitionBuilder<'a>,
    pub lsp_go_to_declaration: GoToDeclarationBuilder<'a>,
    pub scope: ScopeBuilder<'a>,
    pub accessor: ReferenceBuilder<'a>,
    pub check: CheckBuilder<'a>,
    pub comment: CommentBuilder<'a>,
}

impl<'a> Features<'a> {
    pub fn new(
        features_attributes: &'a ReferenceOrSymbolFeatures<'a>,
        helper_attributes: &'a ast::Data<util::Ignored, StructHelpers>,
        input_name: &'a Ident,
        fields: &'a StructFields,
    ) -> Self {
        Self {
            features_attributes,
            lsp_code_lens: CodeLensBuilder::new(input_name, fields),
            lsp_completion_items: CompletionItemsBuilder::new(input_name, fields),
            lsp_document_symbols: DocumentSymbolBuilder::new(input_name, fields),
            lsp_hover_info: HoverInfoBuilder::new(input_name, fields),
            lsp_inlay_hints: InlayHintsBuilder::new(input_name, fields),
            lsp_semantic_tokens: SemanticTokensBuilder::new(input_name, fields),
            lsp_go_to_definition: GotoDefinitionBuilder::new(input_name, fields),
            lsp_go_to_declaration: GoToDeclarationBuilder::new(input_name, fields),
            scope: ScopeBuilder::new(input_name, fields),
            accessor: ReferenceBuilder::new(input_name, fields),
            check: CheckBuilder::new(input_name, helper_attributes, fields),
            comment: CommentBuilder::new(input_name, fields),
        }
    }
}

impl<'a> ToTokens for Features<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.features_attributes {
            ReferenceOrSymbolFeatures::Reference(reference) => {
                self.accessor.code_gen_accessor(reference).to_tokens(tokens);
                self.scope.code_gen_accessor(reference).to_tokens(tokens);
                self.check.code_gen_accessor(reference).to_tokens(tokens);
                self.comment.code_gen_accessor(reference).to_tokens(tokens);
                self.lsp_code_lens
                    .code_gen_accessor(reference)
                    .to_tokens(tokens);
                self.lsp_completion_items
                    .code_gen_accessor(reference)
                    .to_tokens(tokens);
                self.lsp_document_symbols
                    .code_gen_accessor(reference)
                    .to_tokens(tokens);
                self.lsp_hover_info
                    .code_gen_accessor(reference)
                    .to_tokens(tokens);
                self.lsp_inlay_hints
                    .code_gen_accessor(reference)
                    .to_tokens(tokens);
                self.lsp_semantic_tokens
                    .code_gen_accessor(reference)
                    .to_tokens(tokens);
                self.lsp_go_to_definition
                    .code_gen_accessor(reference)
                    .to_tokens(tokens);
                self.lsp_go_to_declaration
                    .code_gen_accessor(reference)
                    .to_tokens(tokens);
            }
            ReferenceOrSymbolFeatures::Symbol(symbol) => {
                self.accessor.code_gen(symbol).to_tokens(tokens);
                self.scope.code_gen(symbol).to_tokens(tokens);
                self.check.code_gen(symbol).to_tokens(tokens);
                self.comment.code_gen(symbol).to_tokens(tokens);
                self.lsp_code_lens.code_gen(symbol).to_tokens(tokens);
                self.lsp_completion_items.code_gen(symbol).to_tokens(tokens);
                self.lsp_document_symbols.code_gen(symbol).to_tokens(tokens);
                self.lsp_hover_info.code_gen(symbol).to_tokens(tokens);
                self.lsp_inlay_hints.code_gen(symbol).to_tokens(tokens);
                self.lsp_semantic_tokens.code_gen(symbol).to_tokens(tokens);
                self.lsp_go_to_definition.code_gen(symbol).to_tokens(tokens);
                self.lsp_go_to_declaration
                    .code_gen(symbol)
                    .to_tokens(tokens);
            }
        }
    }
}
