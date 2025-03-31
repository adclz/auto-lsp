use crate::core::ast::BuildInlayHints;
use crate::server::session::Session;
use auto_lsp_core::salsa::db::WorkspaceDatabase;
use lsp_types::{InlayHint, InlayHintParams};
use std::ops::Deref;

/// Get inlay hints for a document.
pub fn get_inlay_hints<Db: WorkspaceDatabase>(
    db: &Db,
    params: InlayHintParams,
) -> anyhow::Result<Option<Vec<InlayHint>>> {
    let mut results = vec![];

    let uri = params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let document = file.document(db).read();
    let root = file.get_ast(db).clone().into_inner();

    root.ast.iter().for_each(|ast| {
        ast.build_inlay_hints(&document, &mut results);
    });

    Ok(Some(results))
}
