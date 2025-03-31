use crate::server::session::Session;
use auto_lsp_core::salsa::{db::BaseDatabase, tracked::get_ast};
use lsp_types::{
    DocumentDiagnosticParams, DocumentDiagnosticReport, DocumentDiagnosticReportResult,
    FullDocumentDiagnosticReport, RelatedFullDocumentDiagnosticReport,
};
use std::ops::Deref;

pub fn get_diagnostics<Db: BaseDatabase>(
    db: &Db,
    params: DocumentDiagnosticParams,
) -> anyhow::Result<DocumentDiagnosticReportResult> {
    let uri = params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let root = get_ast(db, file).clone().into_inner();

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
