use lsp_types::DidChangeTextDocumentParams;

use crate::server::session::Session;

use super::WORKSPACE;

impl Session {
    /// Edit a document in [`WORKSPACE`].
    ///
    /// Edits are incremental, meaning that the entire document is not parsed.
    pub(crate) fn edit_document(
        &mut self,
        params: DidChangeTextDocumentParams,
    ) -> anyhow::Result<()> {
        let uri = &params.text_document.uri;

        let mut workspace = WORKSPACE.lock();
        let (root, document) = workspace
            .roots
            .get_mut(uri)
            .ok_or(anyhow::anyhow!("Root not found"))?;

        document.update(
            &mut root.parsers.tree_sitter.parser.write(),
            &params.content_changes,
        )?;

        // Update AST
        root.parse(document);
        workspace.resolve_checks();
        Ok(())
    }
}
