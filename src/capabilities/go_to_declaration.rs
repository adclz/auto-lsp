use lsp_types::request::{GotoDeclarationParams, GotoDeclarationResponse};

use crate::session::Session;

impl Session {
    pub fn go_to_declaration(
        &mut self,
        params: GotoDeclarationParams,
    ) -> anyhow::Result<Option<GotoDeclarationResponse>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let workspace = self.workspaces.get(uri).unwrap();
        let position = params.text_document_position_params.position;
        let doc = &workspace.document;

        let offset = doc.offset_at(position) as usize;
        let item = workspace
            .ast
            .iter()
            .find_map(|symbol| symbol.read().find_at_offset(offset));

        match item {
            Some(item) => Ok(item.read().go_to_declaration(doc)),
            None => Ok(None),
        }
    }
}
