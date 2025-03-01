use crate::core::ast::GetHover;
use lsp_types::{Hover, HoverParams};

use crate::server::session::{Session, WORKSPACE};

impl Session {
    /// Request to get hover information for a symbol at a position
    pub fn get_hover(&mut self, params: HoverParams) -> anyhow::Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;

        let workspace = WORKSPACE.lock();

        let (root, document) = workspace.roots
            .get(uri)
            .ok_or(anyhow::anyhow!("Root not found"))?;

        let position = params.text_document_position_params.position;

        let offset = document.offset_at(position).unwrap();
        let item = root.descendant_at(offset);

        match item {
            Some(item) => Ok(item.get_hover(document)),
            None => Ok(None),
        }
    }
}
