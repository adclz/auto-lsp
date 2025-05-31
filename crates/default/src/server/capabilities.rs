use lsp_types::{
    OneOf, SemanticTokensFullOptions, SemanticTokensLegend, SemanticTokensOptions,
    WorkspaceFoldersServerCapabilities, WorkspaceServerCapabilities,
};

pub static TEXT_DOCUMENT_SYNC: Option<lsp_types::TextDocumentSyncCapability> = Some(
    lsp_types::TextDocumentSyncCapability::Kind(lsp_types::TextDocumentSyncKind::INCREMENTAL),
);

pub static WORKSPACE_PROVIDER: Option<WorkspaceServerCapabilities> =
    Some(WorkspaceServerCapabilities {
        workspace_folders: Some(WorkspaceFoldersServerCapabilities {
            supported: Some(false),
            change_notifications: Some(OneOf::Left(true)),
        }),
        file_operations: None,
    });

pub fn semantic_tokens_provider(
    range: bool,
    token_types: Option<&'static [lsp_types::SemanticTokenType]>,
    token_modifiers: Option<&'static [lsp_types::SemanticTokenModifier]>,
) -> Option<lsp_types::SemanticTokensServerCapabilities> {
    Some(
        lsp_types::SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions {
            legend: SemanticTokensLegend {
                token_types: token_types.map(|types| types.to_vec()).unwrap_or_default(),
                token_modifiers: token_modifiers
                    .map(|modifiers| modifiers.to_vec())
                    .unwrap_or_default(),
            },
            range: Some(range),
            full: Some(SemanticTokensFullOptions::Bool(true)),
            ..Default::default()
        }),
    )
}
