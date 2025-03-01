use crate::core::ast::{BuildCompletionItems, BuildTriggeredCompletionItems};
use lsp_types::{CompletionParams, CompletionResponse, CompletionTriggerKind};

use crate::server::session::{Session, WORKSPACE};

impl Session {
    /// Get completion items for a document.
    pub fn get_completion_items(
        &mut self,
        params: CompletionParams,
    ) -> anyhow::Result<Option<CompletionResponse>> {
        let mut results = vec![];
        let uri = &params.text_document_position.text_document.uri;

        let lock = WORKSPACE.lock();
        let (root, document) = lock
            .roots
            .get(uri)
            .ok_or(anyhow::anyhow!("Root not found"))?;

        match params.context {
            Some(context) => match context.trigger_kind {
                CompletionTriggerKind::INVOKED => {
                    let offset = match document.offset_at(lsp_types::Position {
                        line: params.text_document_position.position.line,
                        character: params.text_document_position.position.character - 1,
                    }) {
                        Some(offset) => offset,
                        None => return Ok(None),
                    };

                    let item = match root.descendant_at(offset) {
                        Some(item) => {
                            if let Some(node) = item.read().get_parent_scope() {
                                node
                            } else {
                                return Ok(None);
                            }
                        }
                        None => return Ok(None),
                    };

                    item.build_completion_items(document, &mut results)
                }
                CompletionTriggerKind::TRIGGER_CHARACTER => {
                    let trigger_character = context.trigger_character.unwrap();
                    let offset = match document.offset_at(lsp_types::Position {
                        line: params.text_document_position.position.line,
                        character: params.text_document_position.position.character - 1,
                    }) {
                        Some(offset) => offset,
                        None => return Ok(None),
                    };

                    let item = match root.descendant_at(offset) {
                        Some(item) => item,
                        None => return Ok(None),
                    };
                    item.build_triggered_completion_items(
                        &trigger_character,
                        document,
                        &mut results,
                    )
                }
                _ => (),
            },
            None => return Ok(None),
        };
        Ok(Some(results.into()))
    }
}
