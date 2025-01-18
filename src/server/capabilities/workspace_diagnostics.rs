use lsp_types::{
    FullDocumentDiagnosticReport, WorkspaceDiagnosticParams, WorkspaceDiagnosticReport,
    WorkspaceDocumentDiagnosticReport, WorkspaceFullDocumentDiagnosticReport,
};

use crate::server::session::{Session, WORKSPACES};

impl Session {
    /// Get diagnostics for all documents.
    pub fn get_workspace_diagnostics(
        &mut self,
        _params: WorkspaceDiagnosticParams,
    ) -> anyhow::Result<WorkspaceDiagnosticReport> {
        let workspaces = WORKSPACES.lock();

        let result: Vec<lsp_types::WorkspaceDocumentDiagnosticReport> = workspaces
            .iter()
            .map(|(uri, workspace)| {
                let errors = workspace.errors.clone();
                WorkspaceDocumentDiagnosticReport::Full(WorkspaceFullDocumentDiagnosticReport {
                    version: None,
                    full_document_diagnostic_report: FullDocumentDiagnosticReport {
                        result_id: None,
                        items: errors,
                    },
                    uri: uri.clone(),
                })
            })
            .collect();

        Ok(WorkspaceDiagnosticReport { items: result })
    }
}
