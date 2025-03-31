use lsp_types::DidChangeTextDocumentParams;
use auto_lsp_core::salsa::db::WorkspaceDatabase;
use crate::server::session::Session;

impl<Db: WorkspaceDatabase> Session<Db> {
    /// Request when a document is changed
    ///
    /// Calls [Session::edit_document] to update the document in the root.
    pub fn edit_text_document(
        &mut self,
        params: DidChangeTextDocumentParams,
    ) -> anyhow::Result<()> {
        self.edit_document(params)
    }
}
