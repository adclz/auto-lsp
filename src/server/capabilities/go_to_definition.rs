use crate::core::ast::GetGoToDefinition;
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse};

use crate::server::session::{Session, WORKSPACE};

impl Session {
    /// Request to go to the definition of a symbol
    ///
    /// The trait [`crate::core::ast::GetGoToDefinition`] needs to be implemented otherwise this will return None.
    pub fn go_to_definition(
        &mut self,
        params: GotoDefinitionParams,
    ) -> anyhow::Result<Option<GotoDefinitionResponse>> {
        let uri = &params.text_document_position_params.text_document.uri;

        let workspace = WORKSPACE.lock();

        let (root, document) = workspace.roots
            .get(uri)
            .ok_or(anyhow::anyhow!("Root not found"))?;

        let position = params.text_document_position_params.position;

        let offset = document.offset_at(position).unwrap();
        let item = root.descendant_at(offset);

        match item {
            Some(item) => Ok(item.go_to_definition(document)),
            None => Ok(None),
        }
    }
}
