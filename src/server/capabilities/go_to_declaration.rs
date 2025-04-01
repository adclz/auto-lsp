use crate::core::ast::GetGoToDeclaration;
use auto_lsp_core::salsa::{db::BaseDatabase, tracked::get_ast};
use lsp_types::request::{GotoDeclarationParams, GotoDeclarationResponse};

/// Request to go to the declaration of a symbol
///
/// The trait [`crate::core::ast::GetGoToDeclaration`] needs to be implemented otherwise this will return None.
pub fn go_to_declaration<Db: BaseDatabase>(
    db: &Db,
    params: GotoDeclarationParams,
) -> anyhow::Result<Option<GotoDeclarationResponse>> {
    let uri = params.text_document_position_params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let document = file.document(db).read();
    let root = get_ast(db, file).clone().into_inner();

    let position = params.text_document_position_params.position;

    let offset = document.offset_at(position).unwrap();
    let item = root.descendant_at(offset);

    match item {
        Some(item) => Ok(item.go_to_declaration(&document)),
        None => Ok(None),
    }
}
