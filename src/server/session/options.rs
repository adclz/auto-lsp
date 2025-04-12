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

use std::collections::HashMap;

use auto_lsp_core::{document::Document, parsers::Parsers};
use regex::{Match, Regex};
/// List of options for the LSP server capabilties ([`lsp_types::ServerCapabilities`])
///
/// Use `..Default::default()` to set the rest of the options to false
///
/// # Example
/// ```rust
/// # use auto_lsp::server::LspOptions;
/// let options = LspOptions {
///    document_symbols: true,
///    diagnostics: true,
///    ..Default::default()
/// };
/// ```
#[derive(Default)]
pub struct LspOptions {
    pub completions: Option<lsp_types::CompletionOptions>,
    pub diagnostics: bool,
    pub document_symbols: bool,
    pub definition_provider: bool,
    pub declaration_provider: bool,
    pub document_links: Option<RegexToDocumentLink>,
    pub folding_ranges: bool,
    pub hover_info: bool,
    pub references: bool,
    pub semantic_tokens: Option<SemanticTokensList>,
    pub selection_ranges: bool,
    pub workspace_symbols: bool,
    pub inlay_hints: bool,
    pub code_lens: bool,
}

/// Initialization options for the LSP server
pub struct InitOptions {
    pub parsers: &'static HashMap<&'static str, Parsers>,
    pub lsp_options: LspOptions,
}

/// Lists of semantic token types and modifiers
///
/// Usually you should define the lists with the [`crate::define_semantic_token_types`] and [`crate::define_semantic_token_modifiers`] macros.
#[derive(Default)]
pub struct SemanticTokensList {
    pub semantic_token_types: Option<&'static [lsp_types::SemanticTokenType]>,
    pub semantic_token_modifiers: Option<&'static [lsp_types::SemanticTokenModifier]>,
}

/// Regex used when the server is asked to provide document links
///
/// **to_document_link** receives the matches and pushes [`lsp_types::DocumentLink`] to the accumulator
///
/// # Example
///
/// ```rust
/// # use auto_lsp::server::{RegexToDocumentLink, Session};
/// # use auto_lsp_core::document::Document;
/// # use lsp_types::{DocumentLink, Url};
/// # use regex::Regex;
/// let regex = Regex::new(r"(\w+):(\d+)").unwrap();
///
/// fn to_document_link(m: regex::Match, line: usize, document: &Document, acc: &mut Vec<DocumentLink>) -> lsp_types::DocumentLink {
///    lsp_types::DocumentLink {
///         data: None,
///         tooltip: Some(m.as_str().to_string()),
///         target:None,
///         range: lsp_types::Range {
///                     start: lsp_types::Position {
///                         line: line as u32,
///                         character: m.start() as u32,
///                     },
///                     end: lsp_types::Position {
///                         line: line as u32,
///                         character: m.end() as u32,
///                     },
///                },
///          }
///    }    
///
/// RegexToDocumentLink {
///     regex,
///     to_document_link,
/// };
pub struct RegexToDocumentLink {
    pub regex: Regex,
    pub to_document_link: fn(
        _match: Match<'_>,
        line: usize,
        document: &Document,
        acc: &mut Vec<lsp_types::DocumentLink>,
    ) -> lsp_types::DocumentLink,
}
