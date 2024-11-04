use auto_lsp::traits::ast_item::AstItem;
use lsp_types::{CompletionParams, CompletionResponse};

use crate::session::Session;

impl<'a> Session<'a> {
    pub fn get_completion_items(
        &mut self,
        params: CompletionParams,
    ) -> anyhow::Result<Option<CompletionResponse>> {
        let uri = &params.text_document_position.text_document.uri;
        let workspace = self.workspaces.get(uri).unwrap();
        let position = params.text_document_position.position;
        let doc = &workspace.document;

        let offset = doc.offset_at(position) as usize;
        let item = workspace
            .ast
            .iter()
            .find_map(|symbol| symbol.find_at_offset(&offset));

        // Todo:  must resolve request before accessing items
        match item {
            Some(item) => {
                let mut results = vec![];
                item.read().unwrap().build_completion_items(&mut results);
                Ok(Some(results.into()))
            }
            None => Ok(None),
        }
    }
}
