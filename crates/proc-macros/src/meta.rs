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

#![allow(unused)]
use darling::util::Flag;
use darling::{ast, util, FromDeriveInput, FromField, FromMeta};
use syn::{Ident, Type};

/// Struct input when `seq` macro is used
#[derive(Debug, FromDeriveInput)]
pub struct StructInput {
    pub data: ast::Data<util::Ignored, StructHelpers>,
}

#[derive(FromField, Debug)]
#[darling(attributes(ast))]
pub struct StructHelpers {
    pub ident: Option<Ident>,
    pub ty: Type,
}

#[derive(Debug, FromMeta)]
pub struct DarlingInput {
    /// The query name
    pub query: String,
    // Lsp
    #[darling(default)]
    pub declaration: Flag,
    #[darling(default)]
    pub definition: Flag,
    #[darling(default)]
    pub hover: Flag,
    #[darling(default)]
    pub document_symbols: Flag,
    #[darling(default)]
    pub code_actions: Flag,
    #[darling(default)]
    pub code_lenses: Flag,
    #[darling(default)]
    pub completions: Flag,
    #[darling(default)]
    pub triggered_completions: Flag,
    #[darling(default)]
    pub inlay_hints: Flag,
    #[darling(default)]
    pub semantic_tokens: Flag,
    // Special
    #[darling(default)]
    pub scope: Flag,
}
