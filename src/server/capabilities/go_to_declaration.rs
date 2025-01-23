use lsp_types::request::{GotoDeclarationParams, GotoDeclarationResponse};

use crate::server::session::{Session, WORKSPACES};

impl Session {
    /// Request to go to the declaration of a symbol
    ///
    /// The trait [`crate::core::ast::GetGoToDeclaration`] needs to be implemented otherwise this will return None.
    pub fn go_to_declaration(
        &mut self,
        params: GotoDeclarationParams,
    ) -> anyhow::Result<Option<GotoDeclarationResponse>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let workspace = WORKSPACES.lock();

        let (workspace, document) = workspace
            .get(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        let position = params.text_document_position_params.position;

        let offset = document.offset_at(position).unwrap();
        let item = workspace
            .ast
            .iter()
            .find_map(|symbol| symbol.read().find_at_offset(offset));

        match item {
            Some(item) => Ok(item.read().go_to_declaration(document)),
            None => Ok(None),
        }
    }
}
