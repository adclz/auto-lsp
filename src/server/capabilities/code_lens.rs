use crate::core::ast::BuildCodeLenses;
use crate::server::session::{Session, WORKSPACES};
use lsp_types::{CodeLens, CodeLensParams};

impl Session {
    /// Get code lenses for a document.
    pub fn get_code_lenses(
        &mut self,
        params: CodeLensParams,
    ) -> anyhow::Result<Option<Vec<CodeLens>>> {
        let mut results = vec![];

        let uri = params.text_document.uri;
        let workspace = WORKSPACES.lock();

        let (workspace, document) = workspace
            .get(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        if let Some(a) = workspace.ast.as_ref() {
            a.build_code_lenses(document, &mut results)
        }

        Ok(Some(results))
    }
}
