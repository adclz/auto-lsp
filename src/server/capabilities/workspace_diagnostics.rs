use crate::server::session::Session;
use auto_lsp_core::salsa::{db::BaseDatabase, tracked::get_ast};
use lsp_types::{
    FullDocumentDiagnosticReport, WorkspaceDiagnosticParams, WorkspaceDiagnosticReport,
    WorkspaceDiagnosticReportResult, WorkspaceDocumentDiagnosticReport,
    WorkspaceFullDocumentDiagnosticReport,
};
use std::ops::Deref;

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
            let ast = get_ast(db, file).clone().into_inner();
            let errors = [ast.lexer_diagnostics.clone(), ast.ast_diagnostics.clone()]
                .into_iter()
                .flatten()
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
