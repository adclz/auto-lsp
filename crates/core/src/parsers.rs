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

use crate::core_build::parse::InvokeParserFn;
use parking_lot::RwLock;
use tree_sitter::{Language, Parser, Query};

/// List of parsing utilities.
///
/// Contains instances of both the [`tree_sitter`] parser and the AST parser.
pub struct Parsers {
    /// The underlying parser, protected by [`RwLock`] for safe concurrent access.
    pub parser: RwLock<Parser>,
    /// The language configuration for this parser.
    pub language: Language,
    /// The core query used to build the AST.
    pub core: Query,
    /// Function to invoke the AST parser.
    pub ast_parser: InvokeParserFn,
}

impl std::fmt::Debug for Parsers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("parsers")
            .field("language", &self.language)
            .field("core query", &self.core)
            .finish()
    }
}
