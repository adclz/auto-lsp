use crate::core::ast::GetHover;
use auto_lsp_core::{
    ast::Traverse,
    salsa::{db::BaseDatabase, tracked::get_ast},
};
use lsp_types::{Hover, HoverParams};

/// Request to get hover information for a symbol at a position
pub fn get_hover<Db: BaseDatabase>(db: &Db, params: HoverParams) -> anyhow::Result<Option<Hover>> {
    let uri = &params.text_document_position_params.text_document.uri;

    let file = db
        .get_file(uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let document = file.document(db).read();
    let root = match get_ast(db, file).to_symbol() {
        Some(item) => item,
        None => return Ok(None),
    };
    let position = params.text_document_position_params.position;

    let offset = document.offset_at(position).unwrap();
    let item = root.descendant_at(offset);

    match item {
        Some(item) => item.get_hover(&document),
        None => Ok(None),
    }
}
