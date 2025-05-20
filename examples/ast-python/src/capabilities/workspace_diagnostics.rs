/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/
use auto_lsp::core::salsa::db::BaseDatabase;
use auto_lsp::{anyhow, lsp_types};
use auto_lsp::core::errors::ParseErrorAccumulator;
use auto_lsp::core::salsa::tracked::get_ast;
use auto_lsp::lsp_types::{FullDocumentDiagnosticReport, WorkspaceDiagnosticParams, WorkspaceDiagnosticReport, WorkspaceDiagnosticReportResult, WorkspaceDocumentDiagnosticReport, WorkspaceFullDocumentDiagnosticReport};

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
