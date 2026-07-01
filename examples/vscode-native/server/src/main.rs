use ast_python::capabilities::code_actions::code_actions;
use ast_python::capabilities::code_lenses::code_lenses;
use ast_python::capabilities::completion_items::completion_items;
use ast_python::capabilities::diagnostics::diagnostics;
use ast_python::capabilities::document_symbols::document_symbols;
use ast_python::capabilities::folding_ranges::folding_ranges;
use ast_python::capabilities::hover::hover;
use ast_python::capabilities::inlay_hints::inlay_hints;
use ast_python::capabilities::selection_ranges::selection_ranges;
use ast_python::capabilities::semantic_tokens::{
    SUPPORTED_MODIFIERS, SUPPORTED_TYPES, semantic_tokens_full,
};
use ast_python::capabilities::workspace_diagnostics::workspace_diagnostics;
use ast_python::capabilities::workspace_symbols::workspace_symbols;
use ast_python::db::PYTHON;
use auto_lsp::default::db::{BaseDatabase, BaseDb};
use auto_lsp::default::server::capabilities::{
    TEXT_DOCUMENT_SYNC, WORKSPACE_PROVIDER, semantic_tokens_provider,
};
use auto_lsp::default::server::file_events::{
    change_text_document, changed_watched_files, open_text_document,
};
use auto_lsp::default::server::workspace_init::WorkspaceInit;
use auto_lsp::lsp_server::{self, Connection};
use auto_lsp::lsp_types::notification::{
    Cancel, DidChangeTextDocument, DidChangeWatchedFiles, DidCloseTextDocument,
    DidOpenTextDocument, DidSaveTextDocument, SetTrace,
};
use auto_lsp::lsp_types::request::{
    CodeActionRequest, CodeLensRequest, Completion, DocumentDiagnosticRequest,
    DocumentSymbolRequest, FoldingRangeRequest, HoverRequest, InlayHintRequest,
    SelectionRangeRequest, SemanticTokensFullRequest, WorkspaceDiagnosticRequest,
    WorkspaceSymbolRequest,
};
use auto_lsp::lsp_types::{
    CodeActionProviderCapability, CodeLensOptions, CompletionOptions, DiagnosticOptions,
    DiagnosticServerCapabilities, HoverProviderCapability, OneOf, ServerCapabilities,
};
use auto_lsp::server::Session;
use auto_lsp::server::notification_registry::NotificationRegistry;
use auto_lsp::server::options::InitOptions;
use auto_lsp::server::request_registry::RequestRegistry;
use auto_lsp::server::vendored::intent::ThreadIntent;
use std::error::Error;
use std::panic::RefUnwindSafe;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let (connection, io_threads) = Connection::stdio();
    let db = BaseDb::default();

    stderrlog::new()
        .modules([module_path!(), "auto_lsp"])
        .verbosity(4)
        .init()
        .unwrap();

    let (mut session, params) = Session::create(
        InitOptions {
            capabilities: ServerCapabilities {
                text_document_sync: TEXT_DOCUMENT_SYNC.clone(),
                workspace: WORKSPACE_PROVIDER.clone(),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        workspace_diagnostics: true,
                        ..Default::default()
                    },
                )),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                code_lens_provider: Some(CodeLensOptions {
                    resolve_provider: Some(false),
                }),
                inlay_hint_provider: Some(OneOf::Left(true)),
                semantic_tokens_provider: semantic_tokens_provider(
                    false,
                    Some(SUPPORTED_TYPES),
                    Some(SUPPORTED_MODIFIERS),
                ),
                document_symbol_provider: Some(OneOf::Left(true)),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            server_info: None,
        },
        connection,
        db,
    )?;

    // Initialize the session with the client's initialization options.
    // This will also add all documents, parse and send diagnostics.
    let init_results = session.init_workspace(params, |entry| {
        if !entry.file_type().is_file() {
            return None;
        }
        entry
            .path()
            .extension()
            .and_then(|ext| (ext == "py").then(|| &*PYTHON))
    });
    if !init_results.is_empty() {
        init_results.into_iter().for_each(|result| {
            eprintln!("{}", result);
        });
    };

    let mut request_registry = RequestRegistry::<BaseDb>::default();
    let mut notification_registry = NotificationRegistry::<BaseDb>::default();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    session.main_loop(
        on_requests(&mut request_registry),
        on_notifications(&mut notification_registry),
    )?;
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("Shutting down server");
    Ok(())
}

fn on_requests<Db: BaseDatabase + Clone + RefUnwindSafe>(
    registry: &mut RequestRegistry<Db>,
) -> &mut RequestRegistry<Db> {
    registry
        .on::<DocumentDiagnosticRequest, _>(ThreadIntent::Worker, diagnostics)
        .on::<DocumentSymbolRequest, _>(ThreadIntent::Worker, document_symbols)
        .on::<FoldingRangeRequest, _>(ThreadIntent::Worker, folding_ranges)
        .on::<HoverRequest, _>(ThreadIntent::Worker, hover)
        .on::<SemanticTokensFullRequest, _>(ThreadIntent::Worker, semantic_tokens_full)
        .on::<SelectionRangeRequest, _>(ThreadIntent::Worker, selection_ranges)
        .on::<WorkspaceSymbolRequest, _>(ThreadIntent::Worker, workspace_symbols)
        .on::<WorkspaceDiagnosticRequest, _>(ThreadIntent::Worker, workspace_diagnostics)
        .on::<InlayHintRequest, _>(ThreadIntent::Worker, inlay_hints)
        .on::<CodeActionRequest, _>(ThreadIntent::Worker, code_actions)
        .on::<CodeLensRequest, _>(ThreadIntent::Worker, code_lenses)
        .on::<Completion, _>(ThreadIntent::Worker, completion_items)
}

fn on_notifications<Db: BaseDatabase + Clone + RefUnwindSafe>(
    registry: &mut NotificationRegistry<Db>,
) -> &mut NotificationRegistry<Db> {
    registry
        .on_mut::<DidOpenTextDocument, _>(|s, p| match p.text_document.language_id.as_str() {
            "python" => Ok(open_text_document(s, p, &PYTHON)?),
            _ => Ok(()),
        })
        .on_mut::<DidChangeTextDocument, _>(|s, p| Ok(change_text_document(s, p)?))
        .on_mut::<DidChangeWatchedFiles, _>(|s, p| {
            Ok(changed_watched_files(s, p, |url| {
                let path = url.to_file_path().ok()?;
                let ext = path.extension()?.to_str()?;
                (ext == "py").then(|| &*PYTHON)
            })?)
        })
        .on_mut::<Cancel, _>(|s, p| {
            let id: lsp_server::RequestId = match p.id {
                auto_lsp::lsp_types::NumberOrString::Number(id) => id.into(),
                auto_lsp::lsp_types::NumberOrString::String(id) => id.into(),
            };
            if let Some(response) = s.req_queue.incoming.cancel(id) {
                s.connection.sender.send(response.into())?;
            }
            Ok(())
        })
        .on::<DidSaveTextDocument, _>(ThreadIntent::Worker, |_s, _p| Ok(()))
        .on::<DidCloseTextDocument, _>(ThreadIntent::Worker, |_s, _p| Ok(()))
        .on::<SetTrace, _>(ThreadIntent::Worker, |_s, _p| Ok(()))
}
