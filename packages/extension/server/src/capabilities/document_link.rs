use std::str::FromStr;

use lsp_types::{DocumentLink, DocumentLinkParams, Range, Url};
use regex::Regex;
use streaming_iterator::StreamingIterator;

use crate::session::Session;

impl<'a> Session<'a> {
    pub fn get_document_link(
        &mut self,
        params: DocumentLinkParams,
    ) -> anyhow::Result<Vec<DocumentLink>> {
        let uri = &params.text_document.uri;
        let workspace = self.workspaces.get(uri).unwrap();
        let root_node = workspace.cst.root_node();
        let source = workspace.document.get_content(None);
        let query = &workspace.provider.queries.comments;
        let re = Regex::new(r"\s+source:\/\/(\w+.\w+):(\d+)/g").unwrap();

        let mut query_cursor = tree_sitter::QueryCursor::new();
        let mut captures = query_cursor.captures(query, root_node, source.as_bytes());

        let mut results = vec![];

        while let Some((m, capture_index)) = captures.next() {
            let capture = m.captures[*capture_index];
            let comment_text = capture.node.utf8_text(source.as_bytes()).unwrap();

            for _match in re.find_iter(comment_text) {
                let link_start = _match.range().start;
                let link_end = _match.range().end;

                let url = _match.as_str().split(":").collect::<Vec<&str>>();

                results.push(DocumentLink {
                    range: Range {
                        start: workspace
                            .document
                            .position_at((capture.node.start_byte() + link_start) as u32),
                        end: workspace
                            .document
                            .position_at((capture.node.start_byte() + link_end) as u32),
                    },
                    target: Some(
                        Url::from_str(&format!("workspace/{}#L{}", url[0], url[1])).unwrap(),
                    ),
                    tooltip: None,
                    data: None,
                });
            }
        }

        Ok(results)
    }
}
