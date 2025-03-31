use crate::server::session::Session;
use auto_lsp_core::salsa::db::WorkspaceDatabase;
use lsp_types::{DocumentLink, DocumentLinkParams};
use std::ops::Deref;

impl<Db: WorkspaceDatabase> Session<Db> {
    /// Get document links for a document.
    ///
    /// To find a document link, we need the comment [`tree_sitter::Query`] to find all comments,
    /// then we use the regex from the [`crate::server::RegexToDocumentLink`] to find the links,
    /// and finally we pass matches to the **to_document_link** function.
    pub fn get_document_links(
        &mut self,
        params: DocumentLinkParams,
    ) -> anyhow::Result<Option<Vec<DocumentLink>>> {
        let with_regex = &self
            .init_options
            .lsp_options
            .document_links
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Document links regex not found"))?;

        let re = &with_regex.regex;
        let to_document_link = &with_regex.to_document_link;
        let uri = params.text_document.uri;

        let file = self
            .db
            .get_file(&uri)
            .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

        let document = file.document(&self.db).read();
        let root = file.get_ast(&self.db).clone().into_inner();

        let mut results = vec![];
        let matches = root.find_all_with_regex(&document, re);
        matches.into_iter().for_each(|(m, line)| {
            to_document_link(m, line, &document, &root, &mut results);
        });

        Ok(Some(results))
    }
}
