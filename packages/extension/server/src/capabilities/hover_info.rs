use lsp_types::{Hover, HoverParams};

use auto_lsp::traits::ast_item::AstItem;

use crate::session::Session;

impl<'a> Session<'a> {
    pub fn get_hover_info(&mut self, params: HoverParams) -> anyhow::Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let workspace = self.workspaces.get(uri).unwrap();
        let position = params.text_document_position_params.position;
        let doc = &workspace.document;

        let offset = doc.offset_at(position) as usize;
        let item = workspace
            .ast
            .iter()
            .find_map(|symbol| symbol.find_at_offset(&offset));

        match item {
            Some(item) => Ok(item.read().unwrap().get_hover(doc)),
            None => Ok(None),
        }
    }
}
