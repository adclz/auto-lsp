/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/
use super::lexer::get_tree_sitter_errors;
use super::{BaseDatabase, File};
use auto_lsp_core::ast::AstNode;
use auto_lsp_core::errors::ParseErrorAccumulator;
use fastrace::prelude::*;
use salsa::Accumulator;
use std::ops::Deref;
use std::sync::Arc;

/// Query that returns the AST of a file.
///
/// This query will also sort the nodes by their id.
#[salsa::tracked(no_eq, return_ref)]
pub fn get_ast<'db>(db: &'db dyn BaseDatabase, file: File) -> ParsedAst {
    let parsers = file.parsers(db);
    let doc = file.document(db);
    let url = file.url(db);

    if doc.texter.text.is_empty() {
        return ParsedAst::default();
    }

    let root =
        Span::root("build ast", SpanContext::random()).with_property(|| ("file", url.to_string()));
    let _guard = root.set_local_parent();

    let node = doc.tree.root_node();
    let source_code = doc.texter.text.as_bytes();

    get_tree_sitter_errors(db, &node, source_code);

    match (parsers.ast_parser)(db, &doc) {
        Ok(mut nodes) => {
            {
                let _ = Span::enter_with_parent("sort unstable", &root);
                nodes.sort_unstable();
            }
            root.add_event(Event::new(format!("total nodes: {}", nodes.len())));
            fastrace::flush();
            ParsedAst::new(nodes)
        }
        Err(e) => {
            ParseErrorAccumulator::accumulate(e.clone().into(), db);
            ParsedAst::default()
        }
    }
}

/// Cheap cloneable wrapper around a parsed AST.
///
/// The first node of the list is always the root node.
#[derive(Debug, Default, Clone, Eq)]
pub struct ParsedAst {
    pub nodes: Arc<Vec<Arc<dyn AstNode>>>,
}

impl PartialEq for ParsedAst {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.nodes, &other.nodes)
    }
}

impl Deref for ParsedAst {
    type Target = Vec<Arc<dyn AstNode>>;

    fn deref(&self) -> &Self::Target {
        &self.nodes
    }
}

impl ParsedAst {
    pub(crate) fn new(nodes: Vec<Arc<dyn AstNode>>) -> Self {
        Self {
            nodes: Arc::new(nodes),
        }
    }

    /// Returns the root node of the AST.
    pub fn get_root(&self) -> Option<&Arc<dyn AstNode>> {
        self.nodes.first()
    }

    /// Returns the first node that contains the given offset.
    pub fn descendant_at(&self, offset: usize) -> Option<&Arc<dyn AstNode>> {
        let mut result = None;
        for node in self.nodes.iter() {
            let range = node.get_range();

            if range.start_byte > offset {
                // If the start byte is greater than the offset, we can stop searching
                continue;
            }

            if range.start_byte <= offset && offset <= range.end_byte {
                result = Some(node);
            } else {
                continue;
            }
        }
        result
    }
}
