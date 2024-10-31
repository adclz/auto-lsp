use lsp_types::{Diagnostic, DocumentDiagnosticParams};

use crate::session::Session;

impl<'a> Session<'a> {
    pub fn get_diagnostics(
        &mut self,
        params: DocumentDiagnosticParams,
    ) -> anyhow::Result<Vec<Diagnostic>> {
        let uri = params.text_document.uri;
        let workspace = self
            .workspaces
            .get_mut(&uri)
            .ok_or(anyhow::format_err!("Workspace not found"))?;

        Ok(workspace.errors.clone())
    }
}
