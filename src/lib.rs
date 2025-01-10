#[cfg(target_arch = "wasm32")]
use std::fs;

use lsp_types::{
    CodeLensOptions, DeclarationCapability, DocumentLinkOptions, SelectionRangeProviderCapability,
    SemanticTokensFullOptions, SemanticTokensLegend, SemanticTokensOptions,
    WorkspaceFoldersServerCapabilities, WorkspaceServerCapabilities,
};
use session::{InitOptions, InitResult, Session};

use lsp_server::Connection;
use lsp_types::{DiagnosticOptions, DiagnosticServerCapabilities, OneOf, ServerCapabilities};

pub mod capabilities;
pub mod session;

pub extern crate auto_lsp_core;
pub extern crate auto_lsp_macros;
pub extern crate constcat;
pub extern crate lsp_textdocument;
pub extern crate lsp_types;
pub extern crate self as auto_lsp;
pub extern crate tree_sitter;

pub fn create_session(init_options: InitOptions) -> anyhow::Result<InitResult> {
    // This is a workaround for a deadlock issue in WASI libc.
    // See https://github.com/WebAssembly/wasi-libc/pull/491
    #[cfg(target_arch = "wasm32")]
    fs::metadata("/workspace").unwrap();

    // Note that  we must have our logging only write out to stderr since the communication with the client
    // is done via stdin/stdout. If we write to stdout, we will corrupt the communication.
    stderrlog::new()
        .modules(vec![module_path!(), "auto_lsp_core"])
        .timestamp(stderrlog::Timestamp::Second)
        .verbosity(3)
        .init()
        .unwrap();

    log::info!("Starting LSP server");
    log::info!("");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Kind(
            lsp_types::TextDocumentSyncKind::INCREMENTAL,
        )),
        diagnostic_provider: Some(DiagnosticServerCapabilities::Options(DiagnosticOptions {
            inter_file_dependencies: true,
            workspace_diagnostics: true,
            ..Default::default()
        })),
        document_symbol_provider: Some(OneOf::Left(true)),
        folding_range_provider: Some(lsp_types::FoldingRangeProviderCapability::Simple(true)),
        semantic_tokens_provider: Some(
            lsp_types::SemanticTokensServerCapabilities::SemanticTokensOptions(
                SemanticTokensOptions {
                    legend: SemanticTokensLegend {
                        token_types: init_options.semantic_token_types.to_vec(),
                        token_modifiers: init_options.semantic_token_modifiers.to_vec(),
                    },
                    range: Some(true),
                    full: Some(SemanticTokensFullOptions::Bool(true)),
                    ..Default::default()
                },
            ),
        ),
        hover_provider: Some(lsp_types::HoverProviderCapability::Simple(true)),
        workspace_symbol_provider: Some(OneOf::Left(true)),
        document_link_provider: Some(DocumentLinkOptions {
            resolve_provider: Some(false),
            work_done_progress_options: Default::default(),
        }),
        selection_range_provider: Some(SelectionRangeProviderCapability::Simple(true)),
        workspace: Some(WorkspaceServerCapabilities {
            workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                supported: Some(true),
                change_notifications: Some(OneOf::Left(true)),
            }),
            ..Default::default()
        }),
        inlay_hint_provider: Some(OneOf::Left(true)),
        code_lens_provider: Some(CodeLensOptions {
            resolve_provider: Some(false),
        }),
        completion_provider: Some(lsp_types::CompletionOptions {
            trigger_characters: None,
            resolve_provider: Some(false),
            ..Default::default()
        }),
        definition_provider: Some(OneOf::Left(true)),
        declaration_provider: Some(DeclarationCapability::Simple(true)),
        references_provider: Some(OneOf::Left(true)),
        ..Default::default()
    })
    .unwrap();

    let initialization_params = connection.initialize(server_capabilities)?;

    let mut session = Session::new(init_options, connection);

    // Initialize the session with the client's initialization options.
    // This will also add all documents, parse and send diagnostics.
    session.init(initialization_params)?;

    Ok(InitResult {
        session,
        io_threads,
    })
}
