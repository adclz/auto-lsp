use lsp_types::DidOpenTextDocumentParams;

use crate::session::Session;

impl<'a> Session<'a> {
    pub fn open_text_document(&mut self, _params: DidOpenTextDocumentParams) -> anyhow::Result<()> {
        // Todo ?
        Ok(())
    }
}
