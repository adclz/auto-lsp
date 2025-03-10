use lsp_types::{
    FullDocumentDiagnosticReport, WorkspaceDiagnosticParams, WorkspaceDiagnosticReport,
    WorkspaceDiagnosticReportResult, WorkspaceDocumentDiagnosticReport,
    WorkspaceFullDocumentDiagnosticReport,
};

use crate::server::session::{Session, WORKSPACE};

impl Session {
    /// Get diagnostics for all documents.
    pub fn get_workspace_diagnostics(
        &mut self,
        _params: WorkspaceDiagnosticParams,
    ) -> anyhow::Result<WorkspaceDiagnosticReportResult> {
        let lock = WORKSPACE.lock();

        let result: Vec<lsp_types::WorkspaceDocumentDiagnosticReport> = lock
            .roots
            .iter()
            .map(|(uri, (root, _))| {
                let errors = root.diagnostics.clone();
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

        Ok(WorkspaceDiagnosticReportResult::Report(
            WorkspaceDiagnosticReport { items: result },
        ))
    }
}
