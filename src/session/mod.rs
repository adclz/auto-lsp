use std::{
    collections::HashMap,
    sync::{RwLock, Weak},
};

#[cfg(target_arch = "wasm32")]
use std::fs;

use crate::session::dispatchers::{NotificationDispatcher, RequestDispatcher};
use auto_lsp_core::{
    builders::BuilderParams,
    symbol::{AstSymbol, DynSymbol},
    workspace::WorkspaceContext,
};
use crossbeam_channel::select;
use cst_parser::CstParser;
use lsp_server::{Connection, IoThreads, Message};
use lsp_types::{
    notification::{DidChangeTextDocument, DidChangeWatchedFiles, DidOpenTextDocument},
    request::DocumentDiagnosticRequest,
    CodeLensOptions, DocumentLinkOptions, SelectionRangeProviderCapability,
    SemanticTokensFullOptions, SemanticTokensLegend, SemanticTokensOptions, ServerCapabilities,
    WorkspaceFoldersServerCapabilities, WorkspaceServerCapabilities,
};
use lsp_types::{
    request::{
        CodeLensRequest, Completion, DocumentLinkRequest, DocumentSymbolRequest,
        FoldingRangeRequest, GotoDeclaration, GotoDefinition, HoverRequest, InlayHintRequest,
        References, SelectionRangeRequest, SemanticTokensFullRequest, SemanticTokensRangeRequest,
        WorkspaceDiagnosticRequest, WorkspaceSymbolRequest,
    },
    DiagnosticOptions, DiagnosticServerCapabilities,
};
use lsp_types::{OneOf, Url};
use workspace::Workspace;

pub mod comment;
pub mod cst_parser;
pub mod dispatchers;
pub mod init;
pub mod senders;
pub mod workspace;

type StaticBuilderFn = fn(
    &mut BuilderParams,
    Option<std::ops::Range<usize>>,
) -> Result<DynSymbol, lsp_types::Diagnostic>;

pub struct Parsers {
    pub cst_parser: CstParser,
    pub ast_parser: StaticBuilderFn,
}

#[macro_export]
macro_rules! configure_parsers {
    ($($extension: expr => {
            $language:ident,
            $comment_query_path: path,
            $fold_query_path: path,
            $highlights_query_path: path,
            $outline_query_path: path,
            $builder: ident
        }),*) => {
        static PARSERS: std::sync::LazyLock<std::collections::HashMap<&str, $crate::session::Parsers>> =
            std::sync::LazyLock::new(|| {
                let mut map = std::collections::HashMap::new();
                map.insert(
                    $($extension, $crate::session::Parsers {
                        cst_parser: $crate::create_parser!($language, $comment_query_path, $fold_query_path, $highlights_query_path, $outline_query_path),
                        ast_parser: |params: &mut $crate::auto_lsp_core::builders::BuilderParams<'_>, range: Option<std::ops::Range<usize>>| {
                            use $crate::auto_lsp_core::builders::StaticBuilder;

                            Ok::<$crate::auto_lsp_core::symbol::DynSymbol, $crate::lsp_types::Diagnostic>(
                                $crate::auto_lsp_core::symbol::Symbol::new_and_check($builder::static_build(params, range)?, params).to_dyn(),
                            )
                        },
                    }),*
                );
                map
            });
    };
}

#[macro_export]
macro_rules! create_parser {
    ($parser: ident, $comment_query_path: path, $fold_query_path: path, $highlights_query_path: path, $outline_query_path: path) => {{
        use $parser::LANGUAGE;

        use std::sync::RwLock;
        let mut parser = $crate::tree_sitter::Parser::new();
        parser
            .set_language(&LANGUAGE.into())
            .expect(&format!("Error loading {} parser", stringify!($parser)));
        let lang = $crate::tree_sitter::Language::new(LANGUAGE);
        auto_lsp::session::cst_parser::CstParser {
            parser: RwLock::new(parser),
            language: lang.clone(),
            queries: auto_lsp::session::cst_parser::Queries {
                comments: $crate::tree_sitter::Query::new(&lang, $comment_query_path).unwrap(),
                fold: $crate::tree_sitter::Query::new(&lang, $fold_query_path).unwrap(),
                highlights: $crate::tree_sitter::Query::new(&lang, $highlights_query_path).unwrap(),
                outline: $crate::tree_sitter::Query::new(&lang, $outline_query_path).unwrap(),
            },
        }
    }};
}

#[derive(Default)]
pub struct LspOptions {
    pub completions: bool,
    pub diagnostics: bool,
    pub document_symbols: bool,
    pub definition_provider: bool,
    pub declaration_provider: bool,
    pub document_links: bool,
    pub folding_ranges: bool,
    pub hover_info: bool,
    pub references: bool,
    pub semantic_tokens: bool,
    pub selection_ranges: bool,
    pub workspace_symbols: bool,
    pub inlay_hints: bool,
    pub code_lens: bool,
}

pub struct InitOptions {
    pub parsers: &'static HashMap<&'static str, Parsers>,
    pub semantic_token_types: Option<&'static [lsp_types::SemanticTokenType]>,
    pub semantic_token_modifiers: Option<&'static [lsp_types::SemanticTokenModifier]>,
    pub lsp_options: LspOptions,
}

pub struct InitResult {
    pub session: Session,
    pub io_threads: IoThreads,
}

pub struct Session {
    pub init_options: InitOptions,
    pub connection: Connection,
    pub io_threads: IoThreads,
    pub extensions: HashMap<String, String>,
    pub workspaces: HashMap<Url, Workspace>,
}

impl Session {
    fn new(init_options: InitOptions, connection: Connection, io_threads: IoThreads) -> Self {
        Self {
            init_options,
            connection,
            io_threads,
            extensions: HashMap::new(),
            workspaces: HashMap::new(),
        }
    }

    pub fn create(init_options: InitOptions) -> anyhow::Result<Session> {
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
            semantic_tokens_provider: match init_options.lsp_options.semantic_tokens {
                true => Some(
                    lsp_types::SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            legend: SemanticTokensLegend {
                                token_types: init_options
                                    .semantic_token_types
                                    .map(|types| types.to_vec())
                                    .unwrap_or_default(),
                                token_modifiers: init_options
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
                false => None,
            },
            hover_provider: match init_options.lsp_options.hover_info {
                true => Some(lsp_types::HoverProviderCapability::Simple(true)),
                false => None,
            },
            workspace_symbol_provider: match init_options.lsp_options.workspace_symbols {
                true => Some(OneOf::Left(true)),
                false => None,
            },
            document_link_provider: match init_options.lsp_options.document_links {
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
            completion_provider: match init_options.lsp_options.completions {
                true => Some(lsp_types::CompletionOptions {
                    trigger_characters: None,
                    resolve_provider: Some(false),
                    ..Default::default()
                }),
                false => None,
            },
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
        })
        .unwrap();

        let initialization_params = connection.initialize(server_capabilities)?;

        let mut session = Session::new(init_options, connection, io_threads);

        // Initialize the session with the client's initialization options.
        // This will also add all documents, parse and send diagnostics.
        session.init(initialization_params)?;

        Ok(session)
    }
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
                                .on::<Completion, _>(Self::get_completion_items)?
                                .on::<GotoDefinition, _>(Self::go_to_definition)?
                                .on::<GotoDeclaration, _>(Self::go_to_declaration)?
                                .on::<References, _>(Self::get_references)?;
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

impl WorkspaceContext for Session {
    fn find(&self, node: &dyn AstSymbol) -> Option<Weak<RwLock<dyn AstSymbol>>> {
        todo!()
    }
}
