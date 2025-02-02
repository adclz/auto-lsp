use crate::core::ast::{BuildCompletionItems, BuildInvokedCompletionItems};
use lsp_types::{CompletionParams, CompletionResponse, CompletionTriggerKind};

use crate::server::session::{Session, WORKSPACES};

impl Session {
    /// Get completion items for a document.
    pub fn get_completion_items(
        &mut self,
        params: CompletionParams,
    ) -> anyhow::Result<Option<CompletionResponse>> {
        let mut results = vec![];
        let uri = &params.text_document_position.text_document.uri;
        let workspace = WORKSPACES.lock();

        let (workspace, document) = workspace
            .get(uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        let offset = document
            .offset_at(params.text_document_position.position)
            .unwrap();

        let item = match workspace.descendant_at(offset) {
            Some(item) => item,
            None => {
                return Ok(None);
            }
        };

        match params.context {
            Some(context) => match context.trigger_kind {
                CompletionTriggerKind::INVOKED => {
                    item.build_completion_items(document, &mut results)
                }
                CompletionTriggerKind::TRIGGER_CHARACTER => {
                    let trigger_character = context.trigger_character.unwrap();
                    item.build_invoked_completion_items(&trigger_character, document, &mut results)
                }
                _ => (),
            },
            None => return Ok(None),
        };
        Ok(Some(results.into()))
    }
}
