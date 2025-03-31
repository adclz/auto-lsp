use std::ops::Deref;
use crate::core::ast::BuildInlayHints;
use lsp_types::{InlayHint, InlayHintParams};
use auto_lsp_core::salsa::db::WorkspaceDatabase;
use crate::server::session::{Session};

impl<Db: WorkspaceDatabase> Session<Db> {
    /// Get inlay hints for a document.
    pub fn get_inlay_hints(
        &mut self,
        params: InlayHintParams,
    ) -> anyhow::Result<Option<Vec<InlayHint>>> {
        let mut results = vec![];

        let uri = params.text_document.uri;
        let db = &*self.db.lock();

        let file = db.get_file(&uri)
            .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

        let document = file.document(db.deref()).read();
        let root = file.get_ast(db.deref()).clone().into_inner();

        root.ast.iter().for_each(|ast| {
            ast.build_inlay_hints(&document, &mut results);
        });

        Ok(Some(results))
    }
}
