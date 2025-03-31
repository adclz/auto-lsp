use crate::core::ast::BuildCodeLenses;
use crate::server::session::Session;
use auto_lsp_core::salsa::db::WorkspaceDatabase;
use lsp_types::{CodeLens, CodeLensParams};

impl<Db: WorkspaceDatabase> Session<Db> {
    /// Get code lenses for a document.
    pub fn get_code_lenses(
        &mut self,
        params: CodeLensParams,
    ) -> anyhow::Result<Option<Vec<CodeLens>>> {
        let mut results = vec![];

        let uri = params.text_document.uri;

        let file = self
            .db
            .get_file(&uri)
            .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

        let document = file.document(&self.db).read();
        let root = file.get_ast(&self.db).clone().into_inner();

        if let Some(a) = root.ast.as_ref() {
            a.build_code_lenses(&document, &mut results)
        }

        Ok(Some(results))
    }
}
