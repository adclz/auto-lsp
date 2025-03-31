use crate::server::session::Session;
use auto_lsp_core::salsa::db::WorkspaceDatabase;
use lsp_types::DidChangeTextDocumentParams;
use texter::updateables::Updateable;

impl<Db: WorkspaceDatabase> Session<Db> {
    /// Edit a document in [`WORKSPACE`].
    ///
    /// Edits are incremental, meaning that the entire document is not parsed.
    pub(crate) fn edit_document(
        &mut self,
        params: DidChangeTextDocumentParams,
    ) -> anyhow::Result<()> {
        let uri = &params.text_document.uri;

        self.db.update(uri, &params.content_changes)
    }
}
