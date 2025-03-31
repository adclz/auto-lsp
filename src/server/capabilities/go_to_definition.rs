use std::ops::Deref;
use crate::core::ast::GetGoToDefinition;
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse};
use auto_lsp_core::salsa::db::WorkspaceDatabase;
use crate::server::session::{Session};

impl<Db: WorkspaceDatabase> Session<Db> {
    /// Request to go to the definition of a symbol
    ///
    /// The trait [`crate::core::ast::GetGoToDefinition`] needs to be implemented otherwise this will return None.
    pub fn go_to_definition(
        &mut self,
        params: GotoDefinitionParams,
    ) -> anyhow::Result<Option<GotoDefinitionResponse>> {
        let uri = &params.text_document_position_params.text_document.uri;

        let db = &*self.db.lock();

        let file = db.get_file(&uri)
            .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

        let document = file.document(db.deref()).read();
        let root = file.get_ast(db.deref()).clone().into_inner();


        let position = params.text_document_position_params.position;

        let offset = document.offset_at(position).unwrap();
        let item = root.descendant_at(offset);

        match item {
            Some(item) => Ok(item.go_to_definition(&document)),
            None => Ok(None),
        }
    }
}
