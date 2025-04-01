use auto_lsp_core::salsa::{
    db::BaseDatabase,
    tracked::{get_ast, DiagnosticAccumulator},
};
use lsp_types::{
    DocumentDiagnosticParams, DocumentDiagnosticReport, DocumentDiagnosticReportResult,
    FullDocumentDiagnosticReport, RelatedFullDocumentDiagnosticReport,
};

pub fn get_diagnostics<Db: BaseDatabase>(
    db: &Db,
    params: DocumentDiagnosticParams,
) -> anyhow::Result<DocumentDiagnosticReportResult> {
    let uri = params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    Ok(DocumentDiagnosticReportResult::Report(
        DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
            related_documents: None,
            full_document_diagnostic_report: FullDocumentDiagnosticReport {
                result_id: None,
                items: get_ast::accumulated::<DiagnosticAccumulator>(db, file)
                    .into_iter()
                    .map(|d| d.into())
                    .collect(),
            },
        }),
    ))
}
