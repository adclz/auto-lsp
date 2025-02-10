use crate::core::ast::BuildCodeLenses;
use crate::server::session::{Session, WORKSPACES};
use lsp_types::{CodeLens, CodeLensParams};

impl Session {
    /// Get code lens for a document.
    pub fn get_code_lens(
        &mut self,
        params: CodeLensParams,
    ) -> anyhow::Result<Option<Vec<CodeLens>>> {
        let mut results = vec![];

        let uri = params.text_document.uri;
        let workspace = WORKSPACES.lock();

        let (workspace, document) = workspace
            .get(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        workspace
            .ast
            .as_ref()
            .map(|a| a.build_code_lens(document, &mut results));

        Ok(Some(results))
    }
}
