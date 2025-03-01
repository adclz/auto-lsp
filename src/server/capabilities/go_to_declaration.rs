use crate::core::ast::GetGoToDeclaration;
use lsp_types::request::{GotoDeclarationParams, GotoDeclarationResponse};

use crate::server::session::{Session, WORKSPACE};

impl Session {
    /// Request to go to the declaration of a symbol
    ///
    /// The trait [`crate::core::ast::GetGoToDeclaration`] needs to be implemented otherwise this will return None.
    pub fn go_to_declaration(
        &mut self,
        params: GotoDeclarationParams,
    ) -> anyhow::Result<Option<GotoDeclarationResponse>> {
        let uri = &params.text_document_position_params.text_document.uri;

        let workspace = WORKSPACE.lock();

        let (root, document) = workspace.roots
            .get(uri)
            .ok_or(anyhow::anyhow!("Root not found"))?;

        let position = params.text_document_position_params.position;

        let offset = document.offset_at(position).unwrap();
        let item = root.descendant_at(offset);

        match item {
            Some(item) => Ok(item.go_to_declaration(document)),
            None => Ok(None),
        }
    }
}
