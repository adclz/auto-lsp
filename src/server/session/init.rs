use std::collections::HashMap;

#[cfg(target_arch = "wasm32")]
use std::fs;

use auto_lsp_core::{
    document::Document,
    workspace::{Parsers, Workspace},
};
use lsp_server::{Connection, IoThreads};
use lsp_types::{
    CodeLensOptions, DiagnosticOptions, DiagnosticServerCapabilities, DocumentLink,
    DocumentLinkOptions, InitializeParams, InitializeResult, OneOf, PositionEncodingKind,
    SelectionRangeProviderCapability, SemanticTokensFullOptions, SemanticTokensLegend,
    SemanticTokensOptions, ServerCapabilities, WorkspaceFoldersServerCapabilities,
    WorkspaceServerCapabilities,
};
use regex::{Match, Regex};
use texter::core::text::Text;

use super::Session;

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
/// # use auto_lsp_core::workspace::Workspace;
/// # use lsp_types::{DocumentLink, Url};
/// # use regex::Regex;
///
/// let regex = Regex::new(r"(\w+):(\d+)").unwrap();
///
/// fn to_document_link(m: regex::Match, line: usize, document: &Document, workspace: &Workspace, acc: &mut Vec<DocumentLink>) -> lsp_types::DocumentLink {
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
        workspace: &Workspace,
        acc: &mut Vec<DocumentLink>,
    ) -> lsp_types::DocumentLink,
}

/// List of options for the LSP server capabilties [`lsp_types::ServerCapabilities`]
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

/// Function to create a new [`Text`] from a [`String`]
pub(crate) type TextFn = fn(String) -> Text;

fn decide_encoding(encs: Option<&[PositionEncodingKind]>) -> (TextFn, PositionEncodingKind) {
    const DEFAULT: (TextFn, PositionEncodingKind) = (Text::new_utf16, PositionEncodingKind::UTF16);
    let Some(encs) = encs else {
        return DEFAULT;
    };

    for enc in encs {
        if *enc == PositionEncodingKind::UTF16 {
            return (Text::new_utf16, enc.clone());
        } else if *enc == PositionEncodingKind::UTF8 {
            return (Text::new, enc.clone());
        }
    }

    DEFAULT
}

impl Session {
    pub(crate) fn new(
        init_options: InitOptions,
        connection: Connection,
        io_threads: IoThreads,
        text_fn: TextFn,
    ) -> Self {
        Self {
            init_options,
            connection,
            io_threads,
            text_fn,
            extensions: HashMap::new(),
        }
    }

    /// Create a new session with the given initialization options.
    ///
    /// This will establish the connection with the client and send the server capabilities.
    pub fn create(init_options: InitOptions) -> anyhow::Result<Session> {
        // This is a workaround for a deadlock issue in WASI libc.
        // See https://github.com/WebAssembly/wasi-libc/pull/491
        #[cfg(target_arch = "wasm32")]
        fs::metadata("/workspace").unwrap();

        // Note that  we must have our logging only write out to stderr since the communication with the client
        // is done via stdin/stdout. If we write to stdout, we will corrupt the communication.
        #[cfg(feature = "log")]
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
        let (id, resp) = connection.initialize_start()?;
        let params: InitializeParams = serde_json::from_value(resp)?;

        let pos_encoding = params
            .capabilities
            .general
            .as_ref()
            .and_then(|g| g.position_encodings.as_deref());

        let (t_fn, enc) = decide_encoding(pos_encoding);

        let server_capabilities = serde_json::to_value(&InitializeResult {
            capabilities: ServerCapabilities {
                position_encoding: Some(enc),
                text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Kind(
                    lsp_types::TextDocumentSyncKind::INCREMENTAL,
                )),
                diagnostic_provider: match init_options.lsp_options.diagnostics {
                    true => Some(DiagnosticServerCapabilities::Options(DiagnosticOptions {
                        inter_file_dependencies: true,
                        workspace_diagnostics: true,
                        ..Default::default()
                    })),
                    false => None,
                },
                document_symbol_provider: match init_options.lsp_options.document_symbols {
                    true => Some(OneOf::Left(true)),
                    false => None,
                },
                folding_range_provider: match init_options.lsp_options.folding_ranges {
                    true => Some(lsp_types::FoldingRangeProviderCapability::Simple(true)),
                    false => None,
                },
                semantic_tokens_provider: match &init_options.lsp_options.semantic_tokens {
                    Some(options) => Some(
                        lsp_types::SemanticTokensServerCapabilities::SemanticTokensOptions(
                            SemanticTokensOptions {
                                legend: SemanticTokensLegend {
                                    token_types: options
                                        .semantic_token_types
                                        .map(|types| types.to_vec())
                                        .unwrap_or_default(),
                                    token_modifiers: options
                                        .semantic_token_modifiers
                                        .map(|types| types.to_vec())
                                        .unwrap_or_default(),
                                },
                                range: Some(true),
                                full: Some(SemanticTokensFullOptions::Bool(true)),
                                ..Default::default()
                            },
                        ),
                    ),
                    None => None,
                },
                hover_provider: match init_options.lsp_options.hover_info {
                    true => Some(lsp_types::HoverProviderCapability::Simple(true)),
                    false => None,
                },
                workspace_symbol_provider: match init_options.lsp_options.workspace_symbols {
                    true => Some(OneOf::Left(true)),
                    false => None,
                },
                document_link_provider: match init_options.lsp_options.document_links.is_some() {
                    true => Some(DocumentLinkOptions {
                        resolve_provider: Some(false),
                        work_done_progress_options: Default::default(),
                    }),
                    false => None,
                },
                selection_range_provider: match init_options.lsp_options.selection_ranges {
                    true => Some(SelectionRangeProviderCapability::Simple(true)),
                    false => None,
                },
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    ..Default::default()
                }),
                inlay_hint_provider: match init_options.lsp_options.inlay_hints {
                    true => Some(OneOf::Left(true)),
                    false => None,
                },
                code_lens_provider: match init_options.lsp_options.code_lens {
                    true => Some(CodeLensOptions {
                        resolve_provider: Some(false),
                    }),
                    false => None,
                },
                completion_provider: init_options.lsp_options.completions.clone(),
                definition_provider: match init_options.lsp_options.definition_provider {
                    true => Some(OneOf::Left(true)),
                    false => None,
                },
                declaration_provider: match init_options.lsp_options.declaration_provider {
                    true => Some(lsp_types::DeclarationCapability::Simple(true)),
                    false => None,
                },
                references_provider: match init_options.lsp_options.references {
                    true => Some(OneOf::Left(true)),
                    false => None,
                },
                ..Default::default()
            },
            server_info: None,
        })
        .unwrap();

        connection.initialize_finish(id, server_capabilities)?;

        let mut session = Session::new(init_options, connection, io_threads, t_fn);

        // Initialize the session with the client's initialization options.
        // This will also add all documents, parse and send diagnostics.
        session.init_workspaces(params)?;

        Ok(session)
    }
}
