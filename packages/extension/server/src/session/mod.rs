use std::{
    collections::HashMap,
    sync::{RwLock, Weak},
};

use crate::session::dispatchers::{NotificationDispatcher, RequestDispatcher};
use auto_lsp_core::{
    builders::BuilderParams,
    symbol::{AstSymbol, DynSymbol},
    workspace::WorkspaceContext,
};
use crossbeam_channel::select;
use cst_parser::CstParser;
use lsp_server::{Connection, IoThreads, Message};
use lsp_types::request::{
    CodeLensRequest, Completion, DocumentLinkRequest, DocumentSymbolRequest, FoldingRangeRequest,
    GotoDeclaration, GotoDefinition, HoverRequest, InlayHintRequest, References,
    SelectionRangeRequest, SemanticTokensFullRequest, SemanticTokensRangeRequest,
    WorkspaceDiagnosticRequest, WorkspaceSymbolRequest,
};
use lsp_types::Url;
use lsp_types::{
    notification::{DidChangeTextDocument, DidChangeWatchedFiles, DidOpenTextDocument},
    request::DocumentDiagnosticRequest,
};
use workspace::Workspace;

pub mod comment;
pub mod cst_parser;
pub mod dispatchers;
pub mod init;
pub mod senders;
pub mod workspace;

pub type StaticBuilderFn = fn(
    &mut BuilderParams,
    Option<std::ops::Range<usize>>,
) -> Result<DynSymbol, lsp_types::Diagnostic>;

pub struct Parsers {
    pub cst_parser: CstParser,
    pub ast_parser: StaticBuilderFn,
}

#[macro_export]
macro_rules! create_parser {
    ($parser: ident) => {{
        use std::sync::RwLock;
        use $parser::{COMMENTS_QUERY, FOLD_QUERY, HIGHLIGHTS_QUERY, LANGUAGE, OUTLINE_QUERY};
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&LANGUAGE.into())
            .expect(&format!("Error loading {} parser", stringify!($parser)));
        let lang = parser.language().unwrap();
        auto_lsp::session::cst_parser::CstParser {
            parser: RwLock::new(parser),
            language: lang.clone(),
            queries: auto_lsp::session::cst_parser::Queries {
                comments: tree_sitter::Query::new(&lang, COMMENTS_QUERY).unwrap(),
                fold: tree_sitter::Query::new(&lang, FOLD_QUERY).unwrap(),
                highlights: tree_sitter::Query::new(&lang, HIGHLIGHTS_QUERY).unwrap(),
                outline: tree_sitter::Query::new(&lang, OUTLINE_QUERY).unwrap(),
            },
        }
    }};
}

#[macro_export]
macro_rules! configure_parsers {
    ($($extension: expr => { $language:ident, $builder:ident }),*) => {
        static PARSERS: std::sync::LazyLock<std::collections::HashMap<&str, auto_lsp::session::Parsers>> =
            std::sync::LazyLock::new(|| {
                let mut map = std::collections::HashMap::new();
                map.insert(
                    $($extension, auto_lsp::session::Parsers {
                        cst_parser: auto_lsp::create_parser!($language),
                        ast_parser: |params: &mut auto_lsp_core::builders::BuilderParams<'_>, range: Option<std::ops::Range<usize>>| {
                            use auto_lsp_core::builders::StaticBuilder;
                            Ok::<auto_lsp_core::symbol::DynSymbol, lsp_types::Diagnostic>(
                                auto_lsp_core::symbol::Symbol::new_and_check($builder::static_build(params, range)?, params).to_dyn(),
                            )
                        },
                    }),*
                );
                map
            });
    };
}

pub struct InitOptions {
    pub parsers: &'static HashMap<&'static str, Parsers>,
    pub semantic_token_types: &'static [lsp_types::SemanticTokenType],
    pub semantic_token_modifiers: &'static [lsp_types::SemanticTokenModifier],
}

pub struct InitResult {
    pub session: Session,
    pub io_threads: IoThreads,
}

pub struct Session {
    pub init_options: InitOptions,
    pub connection: Connection,
    pub extensions: HashMap<String, String>,
    pub workspaces: HashMap<Url, Workspace>,
}

impl Session {
    pub fn new(init_options: InitOptions, connection: Connection) -> Self {
        Self {
            init_options,
            connection,
            extensions: HashMap::new(),
            workspaces: HashMap::new(),
        }
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
