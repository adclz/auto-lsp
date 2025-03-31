use crate::core::ast::GetHover;
use crate::server::session::Session;
use auto_lsp_core::salsa::{db::BaseDatabase, tracked::get_ast};
use lsp_types::{Hover, HoverParams};
use std::ops::Deref;

/// Request to get hover information for a symbol at a position
pub fn get_hover<Db: BaseDatabase>(db: &Db, params: HoverParams) -> anyhow::Result<Option<Hover>> {
    let uri = &params.text_document_position_params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let document = file.document(db).read();
    let root = get_ast(db, file).clone().into_inner();
    let position = params.text_document_position_params.position;

    let offset = document.offset_at(position).unwrap();
    let item = root.descendant_at(offset);

    match item {
        Some(item) => Ok(item.get_hover(&document)),
        None => Ok(None),
    }
}
