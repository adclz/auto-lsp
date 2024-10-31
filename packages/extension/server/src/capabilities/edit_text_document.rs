use std::any;

use lsp_types::DidChangeTextDocumentParams;

use crate::session::Session;

impl<'a> Session<'a> {
    pub fn edit_text_document(
        &mut self,
        params: DidChangeTextDocumentParams,
    ) -> anyhow::Result<()> {
        self.edit_document(params)?;

        Ok(())
    }
}
