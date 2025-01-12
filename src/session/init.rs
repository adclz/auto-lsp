use std::collections::HashMap;

#[cfg(target_arch = "wasm32")]
use std::fs;

use auto_lsp_core::workspace::Parsers;
use lsp_server::{Connection, IoThreads};
use lsp_types::OneOf;
use lsp_types::{
    CodeLensOptions, DocumentLinkOptions, InitializeParams, InitializeResult, PositionEncodingKind,
    SelectionRangeProviderCapability, SemanticTokensFullOptions, SemanticTokensLegend,
    SemanticTokensOptions, ServerCapabilities, WorkspaceFoldersServerCapabilities,
    WorkspaceServerCapabilities,
};
use lsp_types::{DiagnosticOptions, DiagnosticServerCapabilities};

#[derive(Default)]
pub struct SemanticTokensList {
    pub semantic_token_types: Option<&'static [lsp_types::SemanticTokenType]>,
    pub semantic_token_modifiers: Option<&'static [lsp_types::SemanticTokenModifier]>,
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
    pub semantic_tokens: Option<SemanticTokensList>,
    pub selection_ranges: bool,
    pub workspace_symbols: bool,
    pub inlay_hints: bool,
    pub code_lens: bool,
}

pub struct InitOptions {
    pub parsers: &'static HashMap<&'static str, Parsers>,
    pub lsp_options: LspOptions,
}

pub struct InitResult {
    pub session: Session,
    pub io_threads: IoThreads,
}

use texter::core::text::Text;

use super::Session;

pub type TextFn = fn(String) -> Text;

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
    fn new(
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
            },
            server_info: None,
        })
        .unwrap();

        connection.initialize_finish(id, server_capabilities)?;

        let mut session = Session::new(init_options, connection, io_threads, t_fn);

        // Initialize the session with the client's initialization options.
        // This will also add all documents, parse and send diagnostics.
        session.init(params)?;

        Ok(session)
    }
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
        static PARSERS: std::sync::LazyLock<std::collections::HashMap<&str, $crate::auto_lsp_core::workspace::Parsers>> =
            std::sync::LazyLock::new(|| {
                let mut map = std::collections::HashMap::new();
                map.insert(
                    $($extension, $crate::auto_lsp_core::workspace::Parsers {
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
        $crate::auto_lsp_core::workspace::CstParser {
            parser: RwLock::new(parser),
            language: lang.clone(),
            queries: $crate::auto_lsp_core::workspace::Queries {
                comments: $crate::tree_sitter::Query::new(&lang, $comment_query_path).unwrap(),
                fold: $crate::tree_sitter::Query::new(&lang, $fold_query_path).unwrap(),
                highlights: $crate::tree_sitter::Query::new(&lang, $highlights_query_path).unwrap(),
                outline: $crate::tree_sitter::Query::new(&lang, $outline_query_path).unwrap(),
            },
        }
    }};
}
