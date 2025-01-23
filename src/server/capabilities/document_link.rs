use lsp_types::{DocumentLink, DocumentLinkParams};
use streaming_iterator::StreamingIterator;

use crate::server::session::{Session, WORKSPACES};

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
            .ok_or_else(|| anyhow::anyhow!("Document links regex not found"))?
            .with_regex;

        let re = &with_regex.regex;
        let to_document_lnik = &with_regex.to_document_link;
        let uri = &params.text_document.uri;
        let workspace = WORKSPACES.lock();

        let (workspace, document) = workspace
            .get(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        let query = match workspace.parsers.tree_sitter.queries.comments {
            Some(ref query) => query,
            None => return Ok(vec![]),
        };

        let root_node = document.tree.root_node();
        let source = document.texter.text.as_str();

        let mut query_cursor = tree_sitter::QueryCursor::new();
        let mut captures = query_cursor.captures(query, root_node, source.as_bytes());

        let mut results = vec![];

        while let Some((m, capture_index)) = captures.next() {
            let capture = m.captures[*capture_index];
            let comment_text = capture.node.utf8_text(source.as_bytes()).unwrap();

            for _match in re.find_iter(comment_text) {
                to_document_lnik(_match, &mut results);
            }
        }

        Ok(results)
    }
}
