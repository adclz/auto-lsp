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
use lsp_types::{OneOf, SemanticTokenModifier, SemanticTokenType, SemanticTokensFullOptions, SemanticTokensLegend, SemanticTokensOptions, ServerCapabilities, ServerInfo, WorkspaceFoldersServerCapabilities, WorkspaceServerCapabilities};
use regex::{Match, Regex};

/// Initialization options for the LSP server
pub struct InitOptions {
    pub parsers: &'static HashMap<&'static str, Parsers>,
    pub capabilities: ServerCapabilities,
    pub server_info: Option<ServerInfo>,
}

pub static TEXT_DOCUMENT_SYNC: Option<lsp_types::TextDocumentSyncCapability> =
    Some(lsp_types::TextDocumentSyncCapability::Kind(
            lsp_types::TextDocumentSyncKind::INCREMENTAL,
    ));

pub static WORKSPACE_PROVIDER: Option<WorkspaceServerCapabilities> =
    Some(WorkspaceServerCapabilities {
        workspace_folders: Some(WorkspaceFoldersServerCapabilities {
            supported: Some(true),
            change_notifications: Some(OneOf::Left(true)),
        }),
        file_operations: None,
    });

pub fn semantic_tokens_provider(
    range: bool,
    token_types: Option<&'static [lsp_types::SemanticTokenType]>,
    token_modifiers: Option<&'static [lsp_types::SemanticTokenModifier]>,
) -> Option<lsp_types::SemanticTokensServerCapabilities> {
    Some(lsp_types::SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions {
        legend: SemanticTokensLegend {
            token_types: token_types.map(|types| types.to_vec()).unwrap_or_default(),
            token_modifiers: token_modifiers
                .map(|modifiers| modifiers.to_vec())
                .unwrap_or_default(),
        },
        range: Some(range),
        full: Some(SemanticTokensFullOptions::Bool(true)),
        ..Default::default()
    }))
}
