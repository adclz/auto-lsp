use super::Workspace;
use crate::document::Document;
use streaming_iterator::StreamingIterator;

impl Workspace {
    pub fn set_comments(&self, document: &Document) -> anyhow::Result<()> {
        let comments_query = match self.parsers.tree_sitter.queries.comments {
            Some(ref query) => query,
            None => return Ok(()),
        };

        let source_code = document.texter.text.as_bytes();
        let cst = &document.tree;
        let ast = match self.ast.as_ref() {
            Some(ast) => ast,
            None => return Ok(()),
        };

        let mut cursor = tree_sitter::QueryCursor::new();
        let mut captures = cursor.captures(comments_query, cst.root_node(), source_code);

        while let Some((m, capture_index)) = captures.next() {
            let capture = m.captures[*capture_index];
            // Since a comment is not within a query, we look for the next named sibling

            let next_named_sibling = match capture.node.next_named_sibling() {
                Some(node) => node,
                None => continue,
            };

            // We then look if this next sibling exists in the ast

            let node = ast.read().find_at_offset(next_named_sibling.start_byte());

            if let Some(node) = node {
                let range = capture.node.range();
                if node.read().is_comment() {
                    node.write().set_comment(Some(std::ops::Range {
                        start: range.start_byte,
                        end: range.end_byte,
                    }));
                } else {
                    match node.read().get_parent() {
                        Some(parent) => {
                            let parent = parent.to_dyn().unwrap();
                            if parent.read().get_range().start == node.read().get_range().start {
                                if parent.read().is_comment() {
                                    parent.write().set_comment(Some(std::ops::Range {
                                        start: range.start_byte,
                                        end: range.end_byte,
                                    }));
                                }
                            }
                        }
                        None => {}
                    }
                }
            };
        }
        Ok(())
    }
}
