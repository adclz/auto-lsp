use crate::server::session::Session;
use auto_lsp_core::salsa::db::BaseDatabase;
use auto_lsp_core::{ast::BuildCodeActions, salsa::tracked::get_ast};
use lsp_types::{CodeActionOrCommand, CodeActionParams};
use std::ops::Deref;

pub fn get_code_actions<Db: BaseDatabase>(
    db: &Db,
    params: CodeActionParams,
) -> anyhow::Result<Option<Vec<CodeActionOrCommand>>> {
    let mut results = vec![];

    let uri = params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let document = file.document(db).read();
    let root = get_ast(db, file).clone().into_inner();

    if let Some(a) = root.ast.as_ref() {
        a.build_code_actions(&document, &mut results)
    }

    Ok(Some(results))
}
