use auto_lsp_core::root::Root;

use lsp_types::{DidChangeTextDocumentParams, Url};

use crate::server::session::Session;

use super::WORKSPACE;

impl Session {
    /// Add a new document to roots
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

        let (root, document) = Root::from_texter(parsers, uri.clone(), text)?;

        WORKSPACE
            .lock()
            .roots
            .insert(uri.to_owned(), (root, document));

        Ok(())
    }

    /// Edit a document in roots
    ///
    /// Edits are incremental, meaning that the entire document is not re-parsed.
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
        Ok(())
    }
}
