use crate::core::ast::GetHover;
use crate::server::session::Session;
use auto_lsp_core::salsa::db::WorkspaceDatabase;
use lsp_types::{Hover, HoverParams};
use std::ops::Deref;

impl<Db: WorkspaceDatabase> Session<Db> {
    /// Request to get hover information for a symbol at a position
    pub fn get_hover(&mut self, params: HoverParams) -> anyhow::Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;

        let file = self
            .db
            .get_file(&uri)
            .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

        let document = file.document(&self.db).read();
        let root = file.get_ast(&self.db).clone().into_inner();
        let position = params.text_document_position_params.position;

        let offset = document.offset_at(position).unwrap();
        let item = root.descendant_at(offset);

        match item {
            Some(item) => Ok(item.get_hover(&document)),
            None => Ok(None),
        }
    }
}
