use lsp_types::DidOpenTextDocumentParams;

use crate::server::session::Session;

impl Session {
    /// Request when a document is opened
    ///
    /// Since auto_lsp already reads all files at initialization and uses [`lsp_types::notification::DidChangeWatchedFiles`] to track changes,
    /// this request is not necessary.
    pub fn open_text_document(&mut self, _params: DidOpenTextDocumentParams) -> anyhow::Result<()> {
        // Todo ?
        Ok(())
    }
}
