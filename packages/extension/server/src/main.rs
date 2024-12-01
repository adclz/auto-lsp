use std::collections::HashMap;
use std::error::Error;
#[cfg(target_arch = "wasm32")]
use std::fs;

use auto_lsp::builders::{Builder, BuilderFn};
use lsp_types::notification::DidOpenTextDocument;
use lsp_types::request::{
    CodeLensRequest, Completion, DocumentLinkRequest, DocumentSymbolRequest, FoldingRangeRequest,
    HoverRequest, InlayHintRequest, SelectionRangeRequest, SemanticTokensFullRequest,
    SemanticTokensRangeRequest, WorkspaceDiagnosticRequest, WorkspaceSymbolRequest,
};
use lsp_types::{
    CodeLensOptions, DocumentLinkOptions, SelectionRangeProviderCapability,
    SemanticTokensFullOptions, SemanticTokensLegend, SemanticTokensOptions,
    WorkspaceFoldersServerCapabilities, WorkspaceServerCapabilities,
};
use session::cst_parser::CstParser;
use session::dispatchers::{NotificationDispatcher, RequestDispatcher};
use session::Session;

use crossbeam_channel::select;
use lazy_static::lazy_static;
use lsp_server::{Connection, Message};
use lsp_types::{
    notification::{DidChangeTextDocument, DidChangeWatchedFiles},
    request::DocumentDiagnosticRequest,
    DiagnosticOptions, DiagnosticServerCapabilities, OneOf, ServerCapabilities,
};
use symbols::symbols::SourceFileBuilder;

mod capabilities;
mod session;
mod symbols;

//******** <Configuration> *********

///// Parsers and Builders

// CST_PARSERS store all tree_sitter parsers in a HashMap.
// A Parser can be added using the create_parser! macro and the name of the corresponding crate.
// The client is responsible for setting file extensions and which parser to use for each extension.
lazy_static! {
    pub static ref CST_PARSERS: HashMap<String, CstParser> = {
        HashMap::from([(
            "iec-61131-2".into(),
            crate::create_parser!(tree_sitter_iec61131_3_2),
        )])
    };
}

// AST_BUILDERS store all AST builders in a HashMap.
// An AST builder can be added using the create_builder! macro and the name of the corresponding crate.
// When the CST is built, the LSP will try to build the AST using the corresponding builder.
// Since all symbols implement the AstItem trait, a node from a specific ast can hold a reference to another symbol located in a different ast.
lazy_static! {
    pub static ref AST_BUILDERS: HashMap<String, BuilderFn> = HashMap::from([(
        "iec-61131-2".to_string(),
        SourceFileBuilder::builder as BuilderFn
    )]);
}

///// Semantics

// Semantic tokens are stored in phf maps for O(1) access.
// These macros will generated 2 static maps:
// - SUPPORTED_*: list of supported tokens sent to the client.
// - TOKEN_*: list the server uses to generate the tokens.

// List of semantic token types
define_semantic_token_types![standard {
    "function" => FUNCTION,
    "variable" => VARIABLE,
    "keyword" => KEYWORD,
    "number" => NUMBER
}];

// List of semantic token modifiers
define_semantic_token_modifiers![standard {
    "declaration" => DECLARATION,
    "static" => STATIC,
    "readonly" => READONLY,
    "deprecated" => DEPRECATED,
    "defaultLibrary" => DEFAULT_LIBRARY
}];

//******** </Configuration> *********

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // Note that  we must have our logging only write out to stderr since the communication with the client
    // is done via stdin/stdout. If we write to stdout, we will corrupt the communication.
    eprintln!("Starting WASM based LSP server");

    // This is a workaround for a deadlock issue in WASI libc.
    // See https://github.com/WebAssembly/wasi-libc/pull/491
    #[cfg(target_arch = "wasm32")]
    fs::metadata("/workspace").unwrap();

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();

    // Server capabilities.
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
                        token_types: SUPPORTED_TYPES.to_vec(),
                        token_modifiers: SUPPORTED_MODIFIERS.to_vec(),
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
        ..Default::default()
    })
    .unwrap();

    let initialization_params = connection.initialize(server_capabilities)?;

    let mut session = Session::new(connection);

    // Initialize the session with the client's initialization options.
    // This will also add all documents, parse and send diagnostics.
    session.init(initialization_params)?;

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    session.main_loop()?;

    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("Shutting down server");
    Ok(())
}

impl Session {
    pub fn main_loop(&mut self) -> anyhow::Result<()> {
        loop {
            select! {
                recv(self.connection.receiver) -> msg => {
                    match msg? {
                        Message::Request(req) => {
                            if self.connection.handle_shutdown(&req)? {
                                return Ok(());
                            };
                            RequestDispatcher::new(self, req)
                                .on::<DocumentDiagnosticRequest, _>(Self::get_diagnostics)?
                                .on::<DocumentLinkRequest, _>(Self::get_document_link)?
                                .on::<DocumentSymbolRequest, _>(Self::get_document_symbols)?
                                .on::<FoldingRangeRequest, _>(Self::get_folding_ranges)?
                                .on::<HoverRequest, _>(Self::get_hover_info)?
                                .on::<SemanticTokensFullRequest, _>(Self::get_semantic_tokens_full)?
                                .on::<SemanticTokensRangeRequest, _>(Self::get_semantic_tokens_range)?
                                .on::<SelectionRangeRequest, _>(Self::get_selection_ranges)?
                                .on::<WorkspaceSymbolRequest, _>(Self::get_workspace_symbols)?
                                .on::<WorkspaceDiagnosticRequest, _>(Self::get_workspace_diagnostics)?
                                .on::<InlayHintRequest, _>(Self::get_inlay_hint)?
                                .on::<CodeLensRequest, _>(Self::get_code_lens)?
                                .on::<Completion, _>(Self::get_completion_items)?;
                        }
                        Message::Notification(not) => {
                            NotificationDispatcher::new(self, not)
                                .on::<DidOpenTextDocument>(Self::open_text_document)?
                                .on::<DidChangeTextDocument>(Self::edit_text_document)?
                                .on::<DidChangeWatchedFiles>(Self::changed_watched_files)?;
                        }
                        Message::Response(_) => {}
                    }
                }
            }
        }
    }
}
