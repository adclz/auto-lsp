use lsp_types::{
    DocumentDiagnosticParams, DocumentDiagnosticReport, DocumentDiagnosticReportResult,
    FullDocumentDiagnosticReport, RelatedFullDocumentDiagnosticReport,
};

use crate::session::{Session, WORKSPACES};

impl Session {
    pub fn get_diagnostics(
        &mut self,
        params: DocumentDiagnosticParams,
    ) -> anyhow::Result<DocumentDiagnosticReportResult> {
        let uri = params.text_document.uri;
        let workspace = WORKSPACES.lock();

        let workspace = workspace
            .get(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;
        Ok(DocumentDiagnosticReportResult::Report(
            DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
                related_documents: None,
                full_document_diagnostic_report: FullDocumentDiagnosticReport {
                    result_id: None,
                    items: workspace.errors.clone(),
                },
            }),
        ))
    }
}
