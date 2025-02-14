use crate::server::session::{Session, WORKSPACES};
use lsp_types::{DocumentLink, DocumentLinkParams};

impl Session {
    /// Get document links for a document.
    ///
    /// To find a document link, we need the comment [`tree_sitter::Query`] to find all comments,
    /// then we use the regex from the [`crate::server::DocumentLinksOption`] to find the links,
    /// and finally we pass matches to the **to_document_link** function.
    pub fn get_document_links(
        &mut self,
        params: DocumentLinkParams,
    ) -> anyhow::Result<Vec<DocumentLink>> {
        let with_regex = &self
            .init_options
            .lsp_options
            .document_links
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Document links regex not found"))?;

        let re = &with_regex.regex;
        let to_document_link = &with_regex.to_document_link;
        let uri = &params.text_document.uri;
        let workspace = WORKSPACES.lock();

        let (workspace, document) = workspace
            .get(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        let mut results = vec![];
        let matches = workspace.find_all_with_regex(&document, &re);
        matches.into_iter().for_each(|(m, line)| {
            to_document_link(m, line, &document, &workspace, &mut results);
        });

        Ok(results)
    }
}
