use crate::server::session::{Session, WORKSPACES};
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
        let workspace = WORKSPACES.lock();

        let (workspace, document) = workspace
            .get(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        if let Some(a) = workspace.ast.as_ref() {
            a.build_code_actions(document, &mut results)
        }

        Ok(Some(results))
    }
}
