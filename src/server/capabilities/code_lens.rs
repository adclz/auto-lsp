use std::ops::Deref;
use crate::core::ast::BuildCodeLenses;
use crate::server::session::{Session};
use lsp_types::{CodeLens, CodeLensParams};
use auto_lsp_core::salsa::db::WorkspaceDatabase;

impl<Db: WorkspaceDatabase> Session<Db> {
    /// Get code lenses for a document.
    pub fn get_code_lenses(
        &mut self,
        params: CodeLensParams,
    ) -> anyhow::Result<Option<Vec<CodeLens>>> {
        let mut results = vec![];

        let uri = params.text_document.uri;
        let db = &*self.db.lock();

        let file = db.get_file(&uri)
            .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;


        let document = file.document(db.deref()).read();
        let root = file.get_ast(db.deref()).clone().into_inner();

        if let Some(a) = root.ast.as_ref() {
            a.build_code_lenses(&document, &mut results)
        }

        Ok(Some(results))
    }
}
