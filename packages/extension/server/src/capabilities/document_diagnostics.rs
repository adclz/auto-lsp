use lsp_types::{
    DocumentDiagnosticParams, DocumentDiagnosticReport, DocumentDiagnosticReportResult,
    FullDocumentDiagnosticReport, RelatedFullDocumentDiagnosticReport,
};

use crate::session::Session;

impl<'a> Session<'a> {
    pub fn get_diagnostics(
        &mut self,
        params: DocumentDiagnosticParams,
    ) -> anyhow::Result<DocumentDiagnosticReportResult> {
        let uri = params.text_document.uri;
        let workspace = self
            .workspaces
            .get_mut(&uri)
            .ok_or(anyhow::format_err!("Workspace not found"))?;

        eprintln!("ERRORS LEN: {}", workspace.errors.len());

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
