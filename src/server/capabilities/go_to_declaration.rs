use crate::core::ast::GetGoToDeclaration;
use crate::server::session::Session;
use auto_lsp_core::salsa::db::WorkspaceDatabase;
use lsp_types::request::{GotoDeclarationParams, GotoDeclarationResponse};
use std::ops::Deref;

impl<Db: WorkspaceDatabase> Session<Db> {
    /// Request to go to the declaration of a symbol
    ///
    /// The trait [`crate::core::ast::GetGoToDeclaration`] needs to be implemented otherwise this will return None.
    pub fn go_to_declaration(
        &mut self,
        params: GotoDeclarationParams,
    ) -> anyhow::Result<Option<GotoDeclarationResponse>> {
        let uri = params.text_document_position_params.text_document.uri;

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
            Some(item) => Ok(item.go_to_declaration(&document)),
            None => Ok(None),
        }
    }
}
