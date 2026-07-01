use auto_lsp::core::errors::ParseErrorAccumulator;
use auto_lsp::default::db::BaseDatabase;
use auto_lsp::default::db::tracked::get_ast;
use auto_lsp::lsp_types::{
    FullDocumentDiagnosticReport, WorkspaceDiagnosticParams, WorkspaceDiagnosticReport,
    WorkspaceDiagnosticReportResult, WorkspaceDocumentDiagnosticReport,
    WorkspaceFullDocumentDiagnosticReport,
};
use auto_lsp::{anyhow, lsp_types};

/// Get diagnostics for all documents.
pub fn workspace_diagnostics(
    db: &impl BaseDatabase,
    _params: WorkspaceDiagnosticParams,
) -> anyhow::Result<WorkspaceDiagnosticReportResult> {
    let result: Vec<lsp_types::WorkspaceDocumentDiagnosticReport> = db
        .get_files()
        .iter()
        .map(|file| {
            let file = *file;
            let errors = get_ast::accumulated::<ParseErrorAccumulator>(db, file)
                .into_iter()
                .map(|d| d.to_lsp_diagnostic(file.document(db)).unwrap())
                .collect();
            WorkspaceDocumentDiagnosticReport::Full(WorkspaceFullDocumentDiagnosticReport {
                version: None,
                full_document_diagnostic_report: FullDocumentDiagnosticReport {
                    result_id: None,
                    items: errors,
                },
                uri: file.url(db).clone(),
            })
        })
        .collect();

    Ok(WorkspaceDiagnosticReportResult::Report(
        WorkspaceDiagnosticReport { items: result },
    ))
}
