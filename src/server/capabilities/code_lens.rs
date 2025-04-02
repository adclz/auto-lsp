use crate::core::ast::BuildCodeLenses;
use auto_lsp_core::salsa::{db::BaseDatabase, tracked::get_ast};
use lsp_types::{CodeLens, CodeLensParams};

pub fn get_code_lenses<Db: BaseDatabase>(
    db: &Db,
    params: CodeLensParams,
) -> anyhow::Result<Option<Vec<CodeLens>>> {
    let mut results = vec![];

    let uri = params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let document = file.document(db).read();
    let root = get_ast(db, file).to_symbol();

    if let Some(root) = root { root.build_code_lenses(&document, &mut results) }

    Ok(Some(results))
}
