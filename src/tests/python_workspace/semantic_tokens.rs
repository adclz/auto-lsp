use super::ast::{Function, Module};
use crate::{self as auto_lsp};
use auto_lsp::core::ast::{AstSymbol, BuildSemanticTokens};
use auto_lsp::define_semantic_token_types;
use auto_lsp_core::document::Document;
use auto_lsp_core::semantic_tokens_builder::SemanticTokensBuilder;

define_semantic_token_types!(standard {
    "Function" => FUNCTION,
});

impl BuildSemanticTokens for Module {
    fn build_semantic_tokens(
        &self,
        doc: &Document,
        builder: &mut auto_lsp_core::semantic_tokens_builder::SemanticTokensBuilder,
    ) {
        for statement in &self.statements {
            statement.read().build_semantic_tokens(doc, builder);
        }
    }
}

impl BuildSemanticTokens for Function {
    fn build_semantic_tokens(&self, doc: &Document, builder: &mut SemanticTokensBuilder) {
        builder.push(
            self.name.read().get_lsp_range(doc),
            TOKEN_TYPES.get_index("Function").unwrap() as u32,
            0,
        );
    }
}
