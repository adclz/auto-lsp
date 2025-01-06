use auto_lsp_core::{
    symbol::{Locator, SymbolData},
    workspace,
};
use lsp_types::Url;
use streaming_iterator::StreamingIterator;

use super::{workspace::Workspace, Session};

impl Session {
    pub fn add_comments(&self, uri: &Url) -> anyhow::Result<()> {
        let workspace = self
            .workspaces
            .get(uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        let comments_query = &workspace.cst_parser.queries.comments;

        let source_code = workspace.document.get_content(None).as_bytes();
        let cst = &workspace.cst;
        let ast = match workspace.ast.as_ref() {
            Some(ast) => ast,
            None => return Ok(()),
        };

        let mut cursor = tree_sitter::QueryCursor::new();
        let mut captures = cursor.captures(comments_query, cst.root_node(), source_code);

        let range = workspace.ast.as_ref().unwrap().read().get_range();
        eprintln!("WHOLE RANGE: {:?}", range);

        while let Some((m, capture_index)) = captures.next() {
            let capture = m.captures[*capture_index];
            // Since a comment is not within a query, we look for the next named sibling

            let next_named_sibling = match capture.node.next_named_sibling() {
                Some(node) => node,
                None => continue,
            };

            // We then look if this next sibling exists in the ast

            let node = ast.find_at_offset(next_named_sibling.start_byte());

            if let Some(node) = node {
                let range = capture.node.range();
                if node.read().is_comment() {
                    node.write().set_comment(Some(std::ops::Range {
                        start: range.start_byte,
                        end: range.end_byte,
                    }));
                    eprintln!("COMMENT SET");
                }
            };
        }
        Ok(())
    }
}
