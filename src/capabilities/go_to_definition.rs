use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverParams};

use auto_lsp_core::symbol::AstSymbol;

use crate::session::Session;

impl Session {
    pub fn go_to_definition(
        &mut self,
        params: GotoDefinitionParams,
    ) -> anyhow::Result<Option<GotoDefinitionResponse>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let workspace = self.workspaces.get(uri).unwrap();
        let position = params.text_document_position_params.position;
        let doc = &workspace.document;

        let offset = doc.offset_at(position).unwrap();
        let item = workspace
            .ast
            .iter()
            .find_map(|symbol| symbol.read().find_at_offset(offset));

        match item {
            Some(item) => Ok(item.read().go_to_definition(doc)),
            None => Ok(None),
        }
    }
}
