use std::str::FromStr;

use lsp_types::{DocumentLink, DocumentLinkParams, Range, Url};
use regex::Regex;
use streaming_iterator::StreamingIterator;

use crate::session::{Session, WORKSPACES};

impl Session {
    pub fn get_document_link(
        &mut self,
        params: DocumentLinkParams,
    ) -> anyhow::Result<Vec<DocumentLink>> {
        let uri = &params.text_document.uri;
        let workspace = WORKSPACES.lock();

        let workspace = workspace
            .get(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        let query = match workspace.parsers.cst_parser.queries.comments {
            Some(ref query) => query,
            None => return Ok(vec![]),
        };

        let root_node = workspace.document.cst.root_node();
        let source = workspace.document.document.text.as_str();
        let re = Regex::new(r"\s+source:(\w+\.\w+):(\d+)").unwrap();

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

                let start = match workspace
                    .document
                    .position_at(capture.node.start_byte() + link_start)
                {
                    Some(start) => start,
                    None => continue,
                };

                let end = match workspace
                    .document
                    .position_at(capture.node.start_byte() + link_end)
                {
                    Some(end) => end,
                    None => continue,
                };

                results.push(DocumentLink {
                    range: Range { start, end },
                    target: Some(
                        Url::from_str(&format!("file:///workspace/{}#L{}", url[1], url[2]))
                            .unwrap(),
                    ),
                    tooltip: None,
                    data: None,
                });
            }
        }

        Ok(results)
    }
}
