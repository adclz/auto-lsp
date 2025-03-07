use crate::core::ast::BuildSemanticTokens;
use auto_lsp_core::semantic_tokens_builder::SemanticTokensBuilder;
use lsp_types::{SemanticTokensParams, SemanticTokensRangeParams, SemanticTokensResult};

use crate::server::session::{Session, WORKSPACE};

impl Session {
    /// Get all semantic tokens for a document.
    pub fn get_semantic_tokens_full(
        &mut self,
        params: SemanticTokensParams,
    ) -> anyhow::Result<SemanticTokensResult> {
        let uri = &params.text_document.uri;

        let workspace = WORKSPACE.lock();

        let (root, document) = workspace
            .roots
            .get(uri)
            .ok_or(anyhow::anyhow!("Root not found"))?;

        let mut builder = SemanticTokensBuilder::new(0.to_string());

        root.ast
            .iter()
            .for_each(|p| p.build_semantic_tokens(document, &mut builder));

        Ok(SemanticTokensResult::Tokens(builder.build()))
    }

    /// Get semantic tokens for a range in a document.
    pub fn get_semantic_tokens_range(
        &mut self,
        params: SemanticTokensRangeParams,
    ) -> anyhow::Result<SemanticTokensResult> {
        let uri = &params.text_document.uri;

        let workspace = WORKSPACE.lock();

        let (root, document) = workspace
            .roots
            .get(uri)
            .ok_or(anyhow::anyhow!("Root not found"))?;

        let mut builder = SemanticTokensBuilder::new(0.to_string());

        root.ast
            .iter()
            .for_each(|p| p.build_semantic_tokens(document, &mut builder));

        Ok(SemanticTokensResult::Tokens(builder.build()))
    }
}
