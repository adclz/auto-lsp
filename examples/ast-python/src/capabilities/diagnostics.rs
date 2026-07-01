use auto_lsp::anyhow;
use auto_lsp::core::errors::ParseErrorAccumulator;
use auto_lsp::default::db::BaseDatabase;
use auto_lsp::default::db::tracked::get_ast;
use auto_lsp::lsp_types::{
    DocumentDiagnosticParams, DocumentDiagnosticReport, DocumentDiagnosticReportResult,
    FullDocumentDiagnosticReport, RelatedFullDocumentDiagnosticReport,
};

pub fn diagnostics(
    db: &impl BaseDatabase,
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
                items: get_ast::accumulated::<ParseErrorAccumulator>(db, file)
                    .into_iter()
                    .map(|d| d.to_lsp_diagnostic(file.document(db)).unwrap())
                    .collect(),
            },
        }),
    ))
}
