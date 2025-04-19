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

use salsa::Accumulator;

use super::db::{BaseDatabase, File};
use crate::ast::DynSymbol;
use crate::core_build::lexer::get_tree_sitter_errors;
use crate::errors::ParseErrorAccumulator;
use std::fmt::Formatter;
use std::sync::Arc;

#[salsa::tracked(no_eq, return_ref)]
pub fn get_ast<'db>(db: &'db dyn BaseDatabase, file: File) -> ParsedAst {
    let parsers = file.parsers(db);
    let doc = file.document(db).read();

    if doc.texter.text.is_empty() {
        return ParsedAst::default();
    }

    let node = doc.tree.root_node();
    let source_code = doc.texter.text.as_bytes();

    get_tree_sitter_errors(db, &node, source_code);

    match (parsers.ast_parser)(db, parsers, &doc) {
        Ok(ast) => ParsedAst::new(ast),
        Err(e) => {
            ParseErrorAccumulator::accumulate(e.clone().into(), db);
            ParsedAst::default()
        }
    }
}

/// Cheap cloneable wrapper around a parsed AST
#[derive(Default, Clone)]
pub struct ParsedAst {
    inner: Arc<Option<DynSymbol>>,
}

impl std::fmt::Debug for ParsedAst {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ParsedAst").field(&self.inner).finish()
    }
}

impl PartialEq for ParsedAst {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for ParsedAst {}

impl ParsedAst {
    fn new(ast: DynSymbol) -> Self {
        Self {
            inner: Arc::new(Some(ast)),
        }
    }

    pub fn to_symbol(&self) -> Option<&DynSymbol> {
        self.inner.as_ref().as_ref()
    }
}

impl<'a> From<&'a ParsedAst> for Option<&'a DynSymbol> {
    fn from(parsed_ast: &'a ParsedAst) -> Option<&'a DynSymbol> {
        parsed_ast.inner.as_ref().as_ref()
    }
}
