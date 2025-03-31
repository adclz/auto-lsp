use crate::server::session::Session;
use auto_lsp_core::salsa::db::WorkspaceDatabase;
use lsp_types::{
    FullDocumentDiagnosticReport, WorkspaceDiagnosticParams, WorkspaceDiagnosticReport,
    WorkspaceDiagnosticReportResult, WorkspaceDocumentDiagnosticReport,
    WorkspaceFullDocumentDiagnosticReport,
};
use std::ops::Deref;

impl<Db: WorkspaceDatabase> Session<Db> {
    /// Get diagnostics for all documents.
    pub fn get_workspace_diagnostics(
        &mut self,
        _params: WorkspaceDiagnosticParams,
    ) -> anyhow::Result<WorkspaceDiagnosticReportResult> {
        let result: Vec<lsp_types::WorkspaceDocumentDiagnosticReport> = self
            .db
            .get_files()
            .iter()
            .map(|file| {
                let file = *file;
                let ast = file.get_ast(&self.db).clone().into_inner();
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
                    uri: file.url(&self.db).clone(),
                })
            })
            .collect();

        Ok(WorkspaceDiagnosticReportResult::Report(
            WorkspaceDiagnosticReport { items: result },
        ))
    }
}
