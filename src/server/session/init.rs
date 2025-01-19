use std::collections::HashMap;

#[cfg(target_arch = "wasm32")]
use std::fs;

use auto_lsp_core::workspace::{Parsers, TreeSitter};
use lsp_server::{Connection, IoThreads};
use lsp_types::{
    CodeLensOptions, DocumentLinkOptions, InitializeParams, InitializeResult, PositionEncodingKind,
    SelectionRangeProviderCapability, SemanticTokensFullOptions, SemanticTokensLegend,
    SemanticTokensOptions, ServerCapabilities, WorkspaceFoldersServerCapabilities,
    WorkspaceServerCapabilities,
};
use lsp_types::{DiagnosticOptions, DiagnosticServerCapabilities};
use lsp_types::{DocumentLink, OneOf};
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
pub struct RegexToDocumentLink {
    pub regex: Regex,
    pub to_document_link:
        for<'a> fn(Match<'a>, acc: &mut Vec<DocumentLink>) -> lsp_types::DocumentLink,
}

pub struct DocumentLinksOption {
    pub with_regex: RegexToDocumentLink,
}

/// List of options for the LSP server capabilties [`lsp_types::ServerCapabilities`]
///
/// Use `..Default::default()` to set the rest of the options to false
///
/// # Example
/// ```rust
/// # use auto_lsp::server::LspOptions;
/// let options = LspOptions {
///    completions: true,
///    diagnostics: true,
///    ..Default::default()
/// };
/// ```
#[derive(Default)]
pub struct LspOptions {
    pub completions: bool,
    pub diagnostics: bool,
    pub document_symbols: bool,
    pub definition_provider: bool,
    pub declaration_provider: bool,
    pub document_links: Option<DocumentLinksOption>,
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

        #[cfg(feature = "deadlock_detection")]
        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_secs(2));
            for deadlock in parking_lot::deadlock::check_deadlock() {
                for deadlock in deadlock {
                    log::error!(
                        "Found a deadlock! {}:\n{:?}",
                        deadlock.thread_id(),
                        deadlock.backtrace()
                    );
                }
            }
        });

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
        session.init_workspaces(params)?;

        Ok(session)
    }
}

/// Create the parsers with any given language and queries.
///
/// Every document in the workspace is linked to a parser, which is used to parse the source code and build both the CST and AST.
///
/// To determine which parser to use for a document, the server will check the file extension against the keys in the `PARSERS` map generated by this macro
///
/// # Example
/// ```rust
/// # use auto_lsp::configure_parsers;
/// # use auto_lsp::core::ast::*;
/// # use auto_lsp::macros::seq;
///
/// static CORE_QUERY: &'static str = "
/// (module) @module
/// (function_definition
///    name: (identifier) @function.name) @function
/// ";
///
/// static COMMENT_QUERY: &'static str = "
/// (comment) @comment
/// ";
/// #[seq(query_name = "module", kind(symbol()))]
/// struct Module {}
///
/// configure_parsers!(
///     "python" => {
///         language: tree_sitter_python::LANGUAGE,
///         node_types: tree_sitter_python::NODE_TYPES,
///         ast_root: Module,
///         core: CORE_QUERY,
///         comment: Some(COMMENT_QUERY),
///         fold: None,
///         highlights: None
///     }
/// );
/// ```
#[macro_export]
macro_rules! configure_parsers {
    ($($extension: expr => {
            language: $language: path,
            node_types: $node_types: path,
            ast_root: $root: ident,
            core: $core: path,
            comment: $comment: expr,
            fold: $fold: expr,
            highlights: $highlights: expr
        }),*) => {
        pub static PARSERS: std::sync::LazyLock<std::collections::HashMap<&str, $crate::core::workspace::Parsers>> =
            std::sync::LazyLock::new(|| {
                let mut map = std::collections::HashMap::new();
                map.insert(
                    $($extension, $crate::core::workspace::Parsers {
                        tree_sitter: $crate::server::create_parser($language, $node_types, $core, $comment, $fold, $highlights),
                        ast_parser: |params: &mut $crate::core::build::MainBuilder<'_>, range: Option<std::ops::Range<usize>>| {
                            use $crate::core::build::StaticBuildable;

                            Ok::<$crate::core::ast::DynSymbol, $crate::lsp_types::Diagnostic>(
                                $crate::core::ast::Symbol::new_and_check($root::static_build(params, range)?, params).to_dyn(),
                            )
                        },
                    }),*
                );
                map
            });
    };
}

#[doc(hidden)]
pub fn create_parser(
    language: tree_sitter_language::LanguageFn,
    node_types: &'static str,
    core: &'static str,
    comments: Option<&'static str>,
    fold: Option<&'static str>,
    highlights: Option<&'static str>,
) -> TreeSitter {
    let mut parser = crate::tree_sitter::Parser::new();
    parser.set_language(&language.into()).unwrap();

    let language = crate::tree_sitter::Language::new(language);

    let core = crate::tree_sitter::Query::new(&language, core).unwrap();
    let comments = comments.map(|path| crate::tree_sitter::Query::new(&language, path).unwrap());
    let fold = fold.map(|path| crate::tree_sitter::Query::new(&language, path).unwrap());
    let highlights =
        highlights.map(|path| crate::tree_sitter::Query::new(&language, path).unwrap());
    TreeSitter {
        parser: crate::parking_lot::RwLock::new(parser),
        node_types,
        language,
        queries: crate::core::workspace::Queries {
            comments,
            fold,
            highlights,
            core,
        },
    }
}
