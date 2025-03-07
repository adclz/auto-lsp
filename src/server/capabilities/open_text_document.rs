use lsp_types::DidOpenTextDocumentParams;

use crate::server::session::{Session, WORKSPACE};

impl Session {
    pub fn open_text_document(&mut self, params: DidOpenTextDocumentParams) -> anyhow::Result<()> {
        let uri = &params.text_document.uri;
        let mut workspace = WORKSPACE.lock();

        if workspace.roots.contains_key(uri) {
            // The file is already in the root
            // We can ignore this change
            return Ok(());
        };
        let file_path = uri
            .to_file_path()
            .map_err(|_| anyhow::anyhow!("Failed to read file {}", uri.to_string()))?;

        let (_url, root, document) = self.file_to_root(&file_path)?;
        workspace.roots.insert(uri.clone(), (root, document));
        Ok(())
    }
}
