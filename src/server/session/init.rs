use std::collections::HashMap;

#[cfg(target_arch = "wasm32")]
use std::fs;
use std::sync::{Arc, LazyLock};
use lsp_server::{Connection, ReqQueue};
use lsp_types::WorkspaceServerCapabilities;
use lsp_types::{
    CodeLensOptions, DiagnosticOptions, DiagnosticServerCapabilities, DocumentLinkOptions,
    InitializeParams, InitializeResult, OneOf, PositionEncodingKind,
    SelectionRangeProviderCapability, SemanticTokensFullOptions, SemanticTokensLegend,
    SemanticTokensOptions, ServerCapabilities, WorkspaceFoldersServerCapabilities,
};
use parking_lot::Mutex;
use serde::Serialize;
use texter::core::text::Text;
use auto_lsp_core::salsa::db::WorkspaceDatabase;
use crate::server::session::notification_registry::NotificationRegistry;
use crate::server::session::request_registry::RequestRegistry;
use super::{InitOptions};
use super::{Session};

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

macro_rules! register_default_requests {
    ($session:expr, { $($req:ty => $handler:expr),* $(,)? }) => {
        $(
            $session.register_request::<$req, _>($handler);
        )*
    };
}

macro_rules! register_default_notifications {
    ($session:expr, { $($req:ty => $handler:expr),* $(,)? }) => {
        $(
            $session.register_notification::<$req, _>($handler);
        )*
    };
}

impl<Db: WorkspaceDatabase + Default> Session<Db> {
    pub(crate) fn new(init_options: InitOptions, connection: Connection, text_fn: TextFn, db: Db) -> Self {
        Self {
            init_options,
            connection,
            text_fn,
            extensions: HashMap::new(),
            req_queue: ReqQueue::default(),
            db: Mutex::new(Box::new(db)),
        }
    }

    pub fn register_request<R, F>(req_registry: &mut RequestRegistry<Db>, handler: F)
    where
        R: lsp_types::request::Request,
        R::Params: serde::de::DeserializeOwned,
        R::Result: Serialize,
        F: Fn(&mut Session<Db>, R::Params) -> anyhow::Result<R::Result> + Send + Sync + 'static,
    {
        req_registry.register::<R, F>(handler);
    }

    pub fn register_notification<N, F>(not_registry: &mut NotificationRegistry<Db>, handler: F)
    where
        N: lsp_types::notification::Notification,
        N::Params: serde::de::DeserializeOwned,
        F: Fn(&mut Session<Db>, N::Params) -> anyhow::Result<()> + Send + Sync + 'static,
    {
        not_registry.register::<N, F>(handler);
    }

    /// Create a new session with the given initialization options.
    ///
    /// This will establish the connection with the client and send the server capabilities.
    pub fn create(init_options: InitOptions, connection: Connection, db: Db) -> anyhow::Result<Session<Db>> {
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
                semantic_tokens_provider: init_options.lsp_options.semantic_tokens.as_ref().map(
                    |options| {
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
                        )
                    },
                ),
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

        let mut session = Session::new(init_options, connection, t_fn, db);

        /*register_default_requests!(session, {
            lsp_types::request::DocumentDiagnosticRequest => |session, params| session.get_diagnostics(params),
            lsp_types::request::DocumentLinkRequest => |session, params| session.get_document_links(params),
            lsp_types::request::DocumentSymbolRequest => |session, params| session.get_document_symbols(params),
            lsp_types::request::FoldingRangeRequest => |session, params| session.get_folding_ranges(params),
            lsp_types::request::HoverRequest => |session, params| session.get_hover(params),
            lsp_types::request::SemanticTokensFullRequest => |session, params| session.get_semantic_tokens_full(params),
            lsp_types::request::SemanticTokensRangeRequest => |session, params| session.get_semantic_tokens_range(params),
            lsp_types::request::SelectionRangeRequest => |session, params| session.get_selection_ranges(params),
            lsp_types::request::WorkspaceSymbolRequest => |session, params| session.get_workspace_symbols(params),
            lsp_types::request::WorkspaceDiagnosticRequest => |session, params| session.get_workspace_diagnostics(params),
            lsp_types::request::InlayHintRequest => |session, params| session.get_inlay_hints(params),
            lsp_types::request::CodeActionRequest => |session, params| session.get_code_actions(params),
            lsp_types::request::CodeLensRequest => |session, params| session.get_code_lenses(params),
            lsp_types::request::Completion => |session, params| session.get_completion_items(params),
            lsp_types::request::GotoDefinition => |session, params| session.go_to_definition(params),
            lsp_types::request::GotoDeclaration => |session, params| session.go_to_declaration(params),
            lsp_types::request::References => |session, params| session.get_references(params),
        });

        register_default_notifications!(session, {
            lsp_types::notification::DidOpenTextDocument => |session, params| session.open_text_document(params),
            lsp_types::notification::DidChangeTextDocument => |session, params| session.edit_text_document(params),
            lsp_types::notification::DidChangeWatchedFiles => |session, params| session.changed_watched_files(params),
            lsp_types::notification::Cancel => |session, params| {
                let id: lsp_server::RequestId = match params.id {
                    lsp_types::NumberOrString::Number(id) => id.into(),
                    lsp_types::NumberOrString::String(id) => id.into(),
                };
                if let Some(response) = session.req_queue.incoming.cancel(id) {
                    session.connection.sender.send(response.into())?;
                }
                Ok(())
            },

             // Disabled notifications (temporary)
            lsp_types::notification::DidSaveTextDocument => |_, _| Ok(()),
            lsp_types::notification::DidCloseTextDocument => |_, _| Ok(()),
            lsp_types::notification::SetTrace => |_, _| Ok(()),
            lsp_types::notification::LogTrace => |_, _| Ok(()),
        });*/

        // Initialize the session with the client's initialization options.
        // This will also add all documents, parse and send diagnostics.
        session.init_workspace(params)?;

        Ok(session)
    }
}
