use crate::server::session::{Session, WORKSPACE};
use auto_lsp_core::ast::BuildCodeActions;
use lsp_types::{CodeAction, CodeActionParams};

impl Session {
    /// Get code actions for a document.
    pub fn get_code_actions(
        &mut self,
        params: CodeActionParams,
    ) -> anyhow::Result<Option<Vec<CodeAction>>> {
        let mut results = vec![];

        let uri = params.text_document.uri;
        let workspace = WORKSPACE.lock();

        let (root, document) = workspace
            .roots
            .get(&uri)
            .ok_or(anyhow::anyhow!("Root not found"))?;

        if let Some(a) = root.ast.as_ref() {
            a.build_code_actions(document, &mut results)
        }

        Ok(Some(results))
    }
}
