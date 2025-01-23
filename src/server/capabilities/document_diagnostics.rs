use lsp_types::{
    DocumentDiagnosticParams, DocumentDiagnosticReport, DocumentDiagnosticReportResult,
    FullDocumentDiagnosticReport, RelatedFullDocumentDiagnosticReport,
};

use crate::server::session::{Session, WORKSPACES};

impl Session {
    /// Get diagnostics for a document.
    ///
    /// Diagnostics are kept in memory since the last time the document was added or updated.
    pub fn get_diagnostics(
        &mut self,
        params: DocumentDiagnosticParams,
    ) -> anyhow::Result<DocumentDiagnosticReportResult> {
        let uri = params.text_document.uri;
        let workspace = WORKSPACES.lock();

        let (workspace, _) = workspace
            .get(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;
        Ok(DocumentDiagnosticReportResult::Report(
            DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
                related_documents: None,
                full_document_diagnostic_report: FullDocumentDiagnosticReport {
                    result_id: None,
                    items: workspace.diagnostics.clone(),
                },
            }),
        ))
    }
}
