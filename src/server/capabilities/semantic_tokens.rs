use crate::core::ast::BuildSemanticTokens;
use auto_lsp_core::salsa::{db::BaseDatabase, tracked::get_ast};
use auto_lsp_core::semantic_tokens_builder::SemanticTokensBuilder;
use lsp_types::{
    SemanticTokensParams, SemanticTokensRangeParams, SemanticTokensRangeResult,
    SemanticTokensResult,
};

/// Get all semantic tokens for a document.
pub fn get_semantic_tokens_full<Db: BaseDatabase>(
    db: &Db,
    params: SemanticTokensParams,
) -> anyhow::Result<Option<SemanticTokensResult>> {
    let uri = &params.text_document.uri;

    let file = db
        .get_file(uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let document = file.document(db).read();
    let ast = get_ast(db, file).to_symbol();

    let mut builder = SemanticTokensBuilder::new(0.to_string());

    if let Some(root) = ast {
        root.build_semantic_tokens(&document, &mut builder)?
    }

    Ok(Some(SemanticTokensResult::Tokens(builder.build())))
}

/// Get semantic tokens for a range in a document.
pub fn get_semantic_tokens_range<Db: BaseDatabase>(
    db: &Db,
    params: SemanticTokensRangeParams,
) -> anyhow::Result<Option<SemanticTokensRangeResult>> {
    let uri = &params.text_document.uri;

    let file = db
        .get_file(uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let document = file.document(db).read();
    let ast = get_ast(db, file).to_symbol();

    let mut builder = SemanticTokensBuilder::new(0.to_string());

    if let Some(root) = ast {
        root.build_semantic_tokens(&document, &mut builder)?
    }

    Ok(Some(SemanticTokensRangeResult::Tokens(builder.build())))
}
