use super::Workspace;
use crate::document::Document;
use streaming_iterator::StreamingIterator;

impl Workspace {
    /// Sets comments in the document based on the [`tree_sitter::Query`] for comments
    ///
    /// This function identifies comments in the [`tree_sitter::Tree`] of the [`Document`] and then
    /// attempts to associate these comments with corresponding nodes in the AST.
    ///
    /// ### Process:
    /// 1. Searches for comments using the provided `comments_query`.
    /// 2. For each comment node, checks for the existence of a named sibling node.
    /// 3. Attempts to find the AST symbol corresponding to the named sibling node.
    /// 4. If the AST node or its parent is identified as a comment, sets the comment range for that node.
    pub fn set_comments(&mut self, document: &Document) -> &mut Self {
        let comments_query = match self.parsers.tree_sitter.queries.comments {
            Some(ref query) => query,
            None => return self,
        };

        let source_code = document.texter.text.as_bytes();
        let cst = &document.tree;
        let ast = match self.ast.as_ref() {
            Some(ast) => ast,
            None => return self,
        };

        let mut cursor = tree_sitter::QueryCursor::new();
        let mut captures = cursor.captures(comments_query, cst.root_node(), source_code);

        while let Some((m, capture_index)) = captures.next() {
            let capture = m.captures[*capture_index];

            // Look if a named sibiling exists after the comment
            if let Some(next_named_sibling) = capture.node.next_named_sibling() {
                // Find the AST symbol
                if let Some(ast_node) = ast.read().descendant_at(next_named_sibling.start_byte()) {
                    let range = std::ops::Range {
                        start: capture.node.range().start_byte,
                        end: capture.node.range().end_byte,
                    };

                    // Check if the AST node is a comment
                    if ast_node.read().is_comment() {
                        ast_node.write().set_comment(Some(range));
                    } else if let Some(parent) = ast_node.read().get_parent() {
                        if let Some(parent) = parent.to_dyn() {
                            if parent.read().get_range().start == ast_node.read().get_range().start
                                && parent.read().is_comment()
                            {
                                parent.write().set_comment(Some(range));
                            }
                        }
                    }
                }
            }
        }
        self
    }
}
