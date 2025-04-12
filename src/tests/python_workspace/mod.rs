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

use crate::{self as auto_lsp};
use auto_lsp::configure_parsers;

pub mod ast;
pub mod check;
pub mod code_actions;
pub mod code_lenses;
pub mod completion_items;
pub mod document_symbols;
pub mod hover;
pub mod inlay_hints;
pub mod semantic_tokens;

use ast::{Module, CORE_QUERY};

configure_parsers!(
    PYTHON_PARSERS,
    "python" => {
        language: tree_sitter_python::LANGUAGE,
        core: CORE_QUERY,
        ast_root: Module
    }
);
