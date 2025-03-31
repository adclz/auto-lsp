use lsp_types::DidChangeTextDocumentParams;
use texter::updateables::Updateable;
use auto_lsp_core::salsa::db::WorkspaceDatabase;
use crate::server::session::Session;


impl<Db: WorkspaceDatabase> Session<Db> {
    /// Edit a document in [`WORKSPACE`].
    ///
    /// Edits are incremental, meaning that the entire document is not parsed.
    pub(crate) fn edit_document(
        &mut self,
        params: DidChangeTextDocumentParams,
    ) -> anyhow::Result<()> {
        let uri = &params.text_document.uri;

        let mut workspace = self.db.lock();
        workspace.update(uri, &params.content_changes)
    }
}
