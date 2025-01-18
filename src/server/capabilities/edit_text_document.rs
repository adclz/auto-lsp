use lsp_types::DidChangeTextDocumentParams;

use crate::server::session::Session;

impl Session {
    /// Request when a document is changed
    ///
    /// Calls [Session::edit_document] to update the document in the workspace.
    pub fn edit_text_document(
        &mut self,
        params: DidChangeTextDocumentParams,
    ) -> anyhow::Result<()> {
        eprintln!("CHANGED EDITED FILE");
        self.edit_document(params)?;

        Ok(())
    }
}
