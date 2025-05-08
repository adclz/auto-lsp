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
use super::db::{BaseDatabase, File};
use super::lexer::get_tree_sitter_errors;
use crate::ast::AstNode;
use crate::errors::ParseErrorAccumulator;
use salsa::Accumulator;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::Arc;

#[salsa::tracked(no_eq, return_ref)]
pub fn get_ast<'db>(db: &'db dyn BaseDatabase, file: File) -> ParsedAst2 {
    let parsers = file.parsers(db);
    let doc = file.document(db).read();

    if doc.texter.text.is_empty() {
        return ParsedAst2::default();
    }

    let node = doc.tree.root_node();
    let source_code = doc.texter.text.as_bytes();

    get_tree_sitter_errors(db, &node, source_code);

    match (parsers.ast_parser)(db, &doc) {
        Ok(mut nodes) => {
            nodes.sort_unstable();
            ParsedAst2::new(nodes)
        }
        Err(e) => {
            ParseErrorAccumulator::accumulate(e.clone().into(), db);
            ParsedAst2::default()
        }
    }
}

/// Cheap cloneable wrapper around a parsed AST
#[derive(Debug, Default, Clone)]
pub struct ParsedAst2 {
    pub nodes: Vec<Arc<dyn AstNode>>,
}

impl ParsedAst2 {
    pub(crate) fn new(nodes: Vec<Arc<dyn AstNode>>) -> Self {
        Self { nodes }
    }

    pub fn get_root(&self) -> Option<&Arc<dyn AstNode>> {
        self.nodes.get(0)
    }

    pub fn descendant_at(&self, offset: usize) -> Option<&Arc<dyn AstNode>> {
        let mut best: Option<&Arc<dyn AstNode>> = None;

        for node in self.nodes.iter() {
            let range = node.get_range();

            if range.start_byte > offset {
                break;
            }

            if range.start_byte <= offset && offset <= range.end_byte {
                match best {
                    Some(existing) => {
                        let existing_range = existing.get_range();
                        let current_span = range.end_byte - range.start_byte;
                        let existing_span = existing_range.end_byte - existing_range.start_byte;

                        // Prefer narrower (more specific) nodes
                        if current_span < existing_span {
                            best = Some(node);
                        }
                    }
                    None => {
                        best = Some(node);
                    }
                }
            }
        }

        best
    }
}

impl Deref for ParsedAst2 {
    type Target = Vec<Arc<dyn AstNode>>;

    fn deref(&self) -> &Self::Target {
        &self.nodes
    }
}

impl PartialEq for ParsedAst2 {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for ParsedAst2 {}
