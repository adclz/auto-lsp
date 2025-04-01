use auto_lsp_core::salsa::{
    db::BaseDatabase,
    tracked::{get_ast, DiagnosticAccumulator},
};
use lsp_types::{
    FullDocumentDiagnosticReport, WorkspaceDiagnosticParams, WorkspaceDiagnosticReport,
    WorkspaceDiagnosticReportResult, WorkspaceDocumentDiagnosticReport,
    WorkspaceFullDocumentDiagnosticReport,
};

/// Get diagnostics for all documents.
pub fn get_workspace_diagnostics<Db: BaseDatabase>(
    db: &Db,
    _params: WorkspaceDiagnosticParams,
) -> anyhow::Result<WorkspaceDiagnosticReportResult> {
    let result: Vec<lsp_types::WorkspaceDocumentDiagnosticReport> = db
        .get_files()
        .iter()
        .map(|file| {
            let file = *file;
            let errors = get_ast::accumulated::<DiagnosticAccumulator>(db, file)
                .into_iter()
                .map(|d| d.into())
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
