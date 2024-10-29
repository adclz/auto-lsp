use std::collections::HashMap;
#[cfg(target_arch = "wasm32")]
use std::fs;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex, RwLock};
use std::{error::Error, str::FromStr};

use capabilities::document_symbols::get_document_symbols;
use capabilities::hover_info::get_hover_info;
use capabilities::semantic_tokens::{SUPPORTED_MODIFIERS, SUPPORTED_TYPES};
use globals::{Session, PARSERS};
use lsp_textdocument::FullTextDocument;
use lsp_types::notification::{
    DidChangeWorkspaceFolders, DidCreateFiles, DidDeleteFiles, DidOpenTextDocument,
    PublishDiagnostics,
};
use lsp_types::request::{
    DocumentLinkRequest, DocumentSymbolRequest, FoldingRangeRequest, HoverRequest,
    SelectionRangeRequest, SemanticTokensFullRequest, SemanticTokensRangeRequest,
    WorkspaceSymbolRequest,
};
use lsp_types::{
    DocumentLinkOptions, DocumentSymbol, FileChangeType, FileOperationRegistrationOptions,
    SelectionRange, SelectionRangeProviderCapability, SemanticTokensFullOptions,
    SemanticTokensLegend, SemanticTokensOptions, WorkspaceFoldersServerCapabilities,
    WorkspaceServerCapabilities,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use symbols::symbols::Symbol;
use walkdir::WalkDir;

use auto_lsp::builders::ast_item::{builder, localized_builder};
use lsp_server::{Connection, ExtractError, Message, Notification, RequestId, Response};
use lsp_types::{
    notification::{DidChangeTextDocument, DidChangeWatchedFiles, DidCloseTextDocument},
    request::{DocumentDiagnosticRequest, GotoDefinition, Request},
    DiagnosticOptions, DiagnosticServerCapabilities, DocumentDiagnosticReport,
    FullDocumentDiagnosticReport, GotoDefinitionResponse, InitializeParams, Location, OneOf,
    PublishDiagnosticsParams, RelatedFullDocumentDiagnosticReport, ServerCapabilities, Url,
};
use workspace::init::get_workspace_folders;

mod capabilities;
mod globals;
mod symbols;
mod tree_sitter_extend;
mod workspace;

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

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Kind(
            lsp_types::TextDocumentSyncKind::INCREMENTAL,
        )),
        diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
            DiagnosticOptions::default(),
        )),
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
                }
                .into(),
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
        definition_provider: Some(OneOf::Left(true)),
        ..Default::default()
    })
    .unwrap();

    let initialization_params = connection.initialize(server_capabilities)?;

    main_loop(connection, initialization_params)?;
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("Shutting down server");
    Ok(())
}

#[allow(non_snake_case, reason = "JSON")]
#[derive(Debug, Deserialize)]
struct InitializationOptions {
    perFileParser: HashMap<String, String>,
}

pub fn add_document(session: &mut Session, uri: &str, language_id: &str, source_code: &str) {
    let document = FullTextDocument::new(language_id.to_owned(), 0, source_code.to_string());
    let extension = match session.extensions.get(language_id) {
        Some(extension) => extension,
        None => {
            eprintln!("No parser found for {}", language_id);
            return;
        }
    };
    let provider = PARSERS
        .get(extension)
        .expect(&format!("{} not found", extension));

    let cst;
    let ast;
    let errors;

    let source_code = source_code.as_bytes();

    cst = provider.try_parse(&source_code, None).unwrap().clone();
    ast = builder(
        &provider.queries.outline,
        Symbol::query_binder,
        Symbol::builder_binder,
        cst.root_node(),
        source_code,
    );
    errors = capabilities::documents::diagnostics::analyze_document(&cst, source_code);

    session.workspaces.insert(
        uri.to_owned(),
        globals::Workspace {
            provider,
            document,
            errors,
            cst,
            ast,
        },
    );
}

fn main_loop(
    connection: Connection,
    params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut session = Session::default();

    // Receive custom initialization options from LSP client
    let params: InitializeParams = serde_json::from_value(params).unwrap();
    let options = InitializationOptions::deserialize(
        params
            .initialization_options
            .expect("Missing initialization options from client"),
    )
    .unwrap();

    // Check if the parser for each file extension is available
    options
        .perFileParser
        .iter()
        .for_each(|(extension, parser)| {
            if let false = PARSERS.contains_key(parser.as_str()) {
                panic!(
                    "Error: Parser {} not found for file extension {}",
                    parser, extension
                );
            }
        });

    session.extensions = options.perFileParser;

    // Initialize the workspace with the files from the workspace folders
    for (uri, document) in get_workspace_folders(&params.workspace_folders) {
        let extension = match session.extensions.get(document.language_id()) {
            Some(extension) => extension,
            None => {
                eprintln!("No parser found for {}", document.language_id());
                continue;
            }
        };
        let provider = PARSERS
            .get(extension)
            .expect(&format!("{} not found", extension));

        let cst;
        let ast;
        let errors;

        let source_code = document.get_content(None).as_bytes();

        cst = provider.try_parse(&source_code, None).unwrap().clone();

        ast = builder(
            &provider.queries.outline,
            Symbol::query_binder,
            Symbol::builder_binder,
            cst.root_node(),
            source_code,
        );
        errors = capabilities::documents::diagnostics::analyze_document(&cst, source_code);

        if errors.len() > 0 {
            let params = PublishDiagnosticsParams {
                uri: Url::from_file_path(&uri).unwrap(),
                diagnostics: errors.clone(),
                version: None,
            };

            send_notification::<PublishDiagnostics>(&connection, params).unwrap();
        }

        session.workspaces.insert(
            uri.to_owned(),
            globals::Workspace {
                provider,
                document,
                errors,
                cst,
                ast,
            },
        );
    }

    // Encapsulate the session in a Arc RwLock for concurrent access
    let session = Arc::new(RwLock::new(session));

    // Main crossbeam loop
    for msg in &connection.receiver {
        match msg {
            Message::Notification(not) => {
                match cast_notification::<DidOpenTextDocument>(not.clone()) {
                    Ok(params) => {
                        eprintln!("Opening document {}", params.text_document.uri.as_str());
                        add_document(
                            &mut session.write().unwrap(),
                            params.text_document.uri.as_str(),
                            params.text_document.language_id.as_str(),
                            &params.text_document.text,
                        );
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                match cast_notification::<DidChangeTextDocument>(not.clone()) {
                    Ok(params) => {
                        let mut lock = session.write().unwrap();
                        let uri = params.text_document.uri.as_str();
                        let workspace = lock.workspaces.get_mut(uri).unwrap();

                        workspace
                            .document
                            .update(&params.content_changes[..], params.text_document.version);
                        tree_sitter_extend::tree_sitter_edit::edit_tree(&params, uri, &mut lock);

                        let workspace = lock.workspaces.get(uri).unwrap();

                        let source_code = workspace.document.get_content(None).as_bytes();

                        let diagnostics = capabilities::documents::diagnostics::analyze_document(
                            &workspace.cst,
                            source_code,
                        );

                        let workspace = lock.workspaces.get_mut(uri).unwrap();

                        workspace.errors = diagnostics;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                match cast_notification::<DidChangeWatchedFiles>(not.clone()) {
                    Ok(params) => {
                        let mut session = session.write().unwrap();

                        params.changes.iter().for_each(|file| match file.typ {
                            FileChangeType::CREATED => {
                                if session.workspaces.contains_key(file.uri.as_str()) {
                                    return;
                                } else {
                                    let uri = file.uri.as_str();
                                    let language_id =
                                        file.uri.as_str().split(".").last().unwrap().to_string();
                                    eprintln!("Opening document {}", file.uri.as_str());
                                    let mut open_file = File::open(file.uri.as_str()).unwrap();
                                    let mut buffer = String::new();
                                    open_file.read_to_string(&mut buffer).unwrap();
                                    add_document(&mut session, uri, &language_id, &buffer);
                                }
                            }
                            FileChangeType::DELETED => {
                                session.workspaces.remove(file.uri.as_str());
                            }
                            FileChangeType::CHANGED => (),
                            _ => (),
                        });
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
            }
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                match cast_request::<DocumentDiagnosticRequest>(req.clone()) {
                    Ok((id, params)) => {
                        eprintln!("Diagnostic document {}", params.text_document.uri.as_str());
                        let mut lock = session.write().unwrap();

                        let workspace = lock
                            .workspaces
                            .get_mut(params.text_document.uri.as_str())
                            .unwrap();

                        let diagnostics = workspace.errors.clone();

                        let result =
                            DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
                                related_documents: None,
                                full_document_diagnostic_report: FullDocumentDiagnosticReport {
                                    result_id: None,
                                    items: diagnostics,
                                },
                            });

                        let result = serde_json::to_value(&result).unwrap();
                        let resp = Response {
                            id,
                            result: Some(result),
                            error: None,
                        };

                        connection.sender.send(Message::Response(resp))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                match cast_request::<DocumentSymbolRequest>(req.clone()) {
                    Ok((id, params)) => {
                        connection
                            .sender
                            .send(Message::Response(get_document_symbols(
                                id,
                                session
                                    .read()
                                    .unwrap()
                                    .workspaces
                                    .get(params.text_document.uri.as_str())
                                    .unwrap(),
                            )))
                            .unwrap();
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                match cast_request::<FoldingRangeRequest>(req.clone()) {
                    Ok((id, params)) => {
                        connection.sender.send(Message::Response(
                            capabilities::folding_ranges::get_folding_ranges(
                                id,
                                &params,
                                &session.read().unwrap(),
                            ),
                        ))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                match cast_request::<SemanticTokensFullRequest>(req.clone()) {
                    Ok((id, params)) => {
                        connection.sender.send(Message::Response(
                            capabilities::semantic_tokens::get_semantic_tokens_full(
                                id,
                                params,
                                &session.read().unwrap(),
                            ),
                        ))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                match cast_request::<SemanticTokensRangeRequest>(req.clone()) {
                    Ok((id, params)) => {
                        connection.sender.send(Message::Response(
                            capabilities::semantic_tokens::get_semantic_tokens_range(
                                id,
                                params,
                                &session.read().unwrap(),
                            ),
                        ))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                match cast_request::<HoverRequest>(req.clone()) {
                    Ok((id, params)) => {
                        let session = session.read().unwrap();
                        let uri = &params.text_document_position_params.text_document.uri;
                        let workspace = session.workspaces.get(uri.as_str()).unwrap();
                        connection.sender.send(Message::Response(
                            capabilities::hover_info::get_hover_info(id, &params, workspace),
                        ))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                match cast_request::<WorkspaceSymbolRequest>(req.clone()) {
                    Ok((id, params)) => {
                        let session = session.read().unwrap();
                        let symbols = capabilities::workspace_symbols::get_workspace_symbols(
                            id, &params, &session,
                        );
                        connection.sender.send(Message::Response(symbols))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                match cast_request::<DocumentLinkRequest>(req.clone()) {
                    Ok((id, params)) => {
                        let session = session.read().unwrap();
                        let symbols =
                            capabilities::document_link::get_document_link(id, &params, &session);
                        connection.sender.send(Message::Response(symbols))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                match cast_request::<SelectionRangeRequest>(req.clone()) {
                    Ok((id, params)) => {
                        let session = session.read().unwrap();
                        let symbols = capabilities::selection_ranges::get_selection_ranges(
                            id, &params, &session,
                        );
                        connection.sender.send(Message::Response(symbols))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                match cast_request::<GotoDefinition>(req.clone()) {
                    Ok((id, params)) => {
                        let uri = params.text_document_position_params.text_document.uri;
                        let loc = Location::new(
                            uri,
                            lsp_types::Range::new(
                                lsp_types::Position::new(0, 0),
                                lsp_types::Position::new(0, 0),
                            ),
                        );
                        let mut vec = Vec::new();
                        vec.push(loc);
                        let result = Some(GotoDefinitionResponse::Array(vec));
                        let result = serde_json::to_value(&result).unwrap();
                        let resp = Response {
                            id,
                            result: Some(result),
                            error: None,
                        };
                        connection.sender.send(Message::Response(resp))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
            }
            Message::Response(_resp) => {}
        }
    }
    Ok(())
}

fn cast_request<R>(
    req: lsp_server::Request,
) -> Result<(RequestId, R::Params), ExtractError<lsp_server::Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}

fn cast_notification<N>(
    notif: lsp_server::Notification,
) -> Result<N::Params, ExtractError<lsp_server::Notification>>
where
    N: lsp_types::notification::Notification,
{
    notif.extract(N::METHOD)
}

fn send_notification<N>(server: &Connection, params: N::Params) -> Result<(), ()>
where
    N: lsp_types::notification::Notification,
    N::Params: Serialize,
{
    server
        .sender
        .send(lsp_server::Notification::new(N::METHOD.to_string(), params).into())
        .unwrap();
    Ok(())
}
