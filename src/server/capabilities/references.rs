use auto_lsp_core::salsa::db::BaseDatabase;
use lsp_types::{Location, ReferenceParams};

/// Request to get references of a symbol
///
/// To get the references, the server will look for the symbol at the given position,
/// then read `get_referrers` from the symbol and return the references.
pub fn get_references<Db: BaseDatabase>(
    db: &Db,
    params: ReferenceParams,
) -> anyhow::Result<Option<Vec<Location>>> {
    let uri = &params.text_document_position.text_document.uri;

    let file = db
        .get_file(uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let document = file.document(db).read();
    let position = params.text_document_position.position;

    // todo
    let offset = document.offset_at(position).unwrap();
    Ok(None)
}
