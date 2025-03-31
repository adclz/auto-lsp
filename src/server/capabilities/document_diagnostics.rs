use std::ops::Deref;
use lsp_types::{
    DocumentDiagnosticParams, DocumentDiagnosticReport, DocumentDiagnosticReportResult,
    FullDocumentDiagnosticReport, RelatedFullDocumentDiagnosticReport,
};
use auto_lsp_core::salsa::db::WorkspaceDatabase;
use crate::server::session::{Session};

impl<Db: WorkspaceDatabase> Session<Db> {
    /// Get diagnostics for a document.
    pub fn get_diagnostics(
        &mut self,
        params: DocumentDiagnosticParams,
    ) -> anyhow::Result<DocumentDiagnosticReportResult> {
        let uri = params.text_document.uri;
        let db = &*self.db.lock();

        let file = db.get_file(&uri)
            .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

        let root = file.get_ast(db.deref()).clone().into_inner();

        Ok(DocumentDiagnosticReportResult::Report(
            DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
                related_documents: None,
                full_document_diagnostic_report: FullDocumentDiagnosticReport {
                    result_id: None,
                    items: [root.lexer_diagnostics.clone(), root.ast_diagnostics.clone()]
                        .into_iter()
                        .flatten()
                        .collect(),
                },
            }),
        ))
    }
}
