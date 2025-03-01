use auto_lsp_core::workspace::Workspace;

use lsp_types::{DidChangeTextDocumentParams, Url};

use crate::server::session::Session;

use super::WORKSPACES;

impl Session {
    /// Add a new document to workspaces
    ///
    /// This will first try to find the correct parser for the language id,
    /// then parse the source code with the tree sitter parser,
    /// and finally build the AST with the core [`tree_sitter::Query`] and root symbol.
    pub(crate) fn add_document(
        &mut self,
        uri: &Url,
        language_id: &str,
        source_code: &str,
    ) -> anyhow::Result<()> {
        let text = (self.text_fn)(source_code.to_string());
        let extension = match self.extensions.get(language_id) {
            Some(extension) => extension,
            None => {
                return Err(anyhow::format_err!(
                    "Extension {} is not registered",
                    language_id
                ))
            }
        };

        let parsers = self
            .init_options
            .parsers
            .get(extension.as_str())
            .ok_or(anyhow::format_err!("No parser available for {}", extension))?;

        let (workspace, document) = Workspace::from_texter(parsers, uri.clone(), text)?;

        WORKSPACES
            .lock()
            .insert(uri.to_owned(), (workspace, document));

        Ok(())
    }

    /// Edit a document in workspaces
    ///
    /// Edits are incremental, meaning that the entire document is not re-parsed.
    pub(crate) fn edit_document(
        &mut self,
        params: DidChangeTextDocumentParams,
    ) -> anyhow::Result<()> {
        let uri = &params.text_document.uri;

        let mut workspaces = WORKSPACES.lock();
        let (workspace, document) = workspaces
            .get_mut(uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        document.update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &params.content_changes,
        )?;

        // Update AST
        workspace.parse(document);
        Ok(())
    }
}
