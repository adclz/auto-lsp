use crate::core::ast::BuildSemanticTokens;
use crate::server::session::Session;
use auto_lsp_core::salsa::db::WorkspaceDatabase;
use auto_lsp_core::semantic_tokens_builder::SemanticTokensBuilder;
use lsp_types::{
    SemanticTokensParams, SemanticTokensRangeParams, SemanticTokensRangeResult,
    SemanticTokensResult,
};
use std::ops::Deref;

/// Get all semantic tokens for a document.
pub fn get_semantic_tokens_full<Db: WorkspaceDatabase>(
    db: &Db,
    params: SemanticTokensParams,
) -> anyhow::Result<Option<SemanticTokensResult>> {
    let uri = &params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let document = file.document(db).read();
    let root = file.get_ast(db).clone().into_inner();

    let mut builder = SemanticTokensBuilder::new(0.to_string());

    root.ast
        .iter()
        .for_each(|p| p.build_semantic_tokens(&document, &mut builder));

    Ok(Some(SemanticTokensResult::Tokens(builder.build())))
}

/// Get semantic tokens for a range in a document.
pub fn get_semantic_tokens_range<Db: WorkspaceDatabase>(
    db: &Db,
    params: SemanticTokensRangeParams,
) -> anyhow::Result<Option<SemanticTokensRangeResult>> {
    let uri = &params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let document = file.document(db).read();
    let root = file.get_ast(db).clone().into_inner();

    let mut builder = SemanticTokensBuilder::new(0.to_string());

    root.ast
        .iter()
        .for_each(|p| p.build_semantic_tokens(&document, &mut builder));

    Ok(Some(SemanticTokensRangeResult::Tokens(builder.build())))
}
