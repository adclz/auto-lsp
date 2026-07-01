use auto_lsp_core::errors::DataBaseError;
use auto_lsp_core::errors::FileSystemError;
use auto_lsp_core::errors::RuntimeError;
use auto_lsp_core::parsers::Parser;
use auto_lsp_server::Session;
use lsp_types::DidChangeTextDocumentParams;
use lsp_types::DidChangeWatchedFilesParams;
use lsp_types::DidOpenTextDocumentParams;
use lsp_types::FileChangeType;
use lsp_types::Url;
use salsa::Setter;

use crate::db::BaseDatabase;
use crate::db::{FileManager, file::File};

/// Handles a [`DidOpenTextDocument`] request.
///
/// If the file already exists in the database, it updates its version.
///
/// If it does not exist, it creates a new file from the text document parameters.
pub fn open_text_document<Db: BaseDatabase>(
    session: &mut Session<Db>,
    params: DidOpenTextDocumentParams,
    parser: &'static Parser,
) -> Result<(), RuntimeError> {
    let url = &params.text_document.uri;

    match session.db.get_file(url) {
        Some(file) => {
            log::info!(target: "auto_lsp::default::file_events", "Did Open Text Document: Already exists - {url}");
            file.set_version(&mut session.db)
                .to(Some(params.text_document.version));
            Ok(())
        }
        None => {
            let file = File::from_text_doc()
                .doc(&params.text_document)
                .session(session)
                .parser(parser)
                .call()?;

            log::info!(target: "auto_lsp::default::file_events", "Did Open Text Document: Created - {url}");
            session.db.add_file(file).map_err(|e| e.into())
        }
    }
}

/// Handles a [`DidChangeTextDocument`] request.
pub fn change_text_document<Db: BaseDatabase>(
    session: &mut Session<Db>,
    params: DidChangeTextDocumentParams,
) -> Result<(), RuntimeError> {
    let file = session
        .db
        .get_file(&params.text_document.uri)
        .ok_or_else(|| DataBaseError::FileNotFound {
            uri: params.text_document.uri.clone(),
        })?;

    log::info!(target: "auto_lsp::default::file_events", "Did Change Text Document: {}", params.text_document.uri);
    Ok(file.update_edit(&mut session.db, &params)?)
}

/// Handle the watched files change notification.
pub fn changed_watched_files<Db: BaseDatabase, F: Fn(&Url) -> Option<&'static Parser>>(
    session: &mut Session<Db>,
    params: DidChangeWatchedFilesParams,
    get_parser: F,
) -> Result<(), RuntimeError> {
    params.changes.iter().try_for_each(|file| {
        if file.uri.scheme() != "file" {
            return Ok(());
        }
        match file.typ {
            FileChangeType::CREATED => {
                let url = &file.uri;
                if session.db.get_file(url).is_some() {
                    // The file is already in db
                    // We can ignore this change
                    return Ok(());
                }
                if let Some(parser) = get_parser(url) {
                    let file = File::from_fs()
                        .session(session)
                        .url(&url)
                        .parser(parser)
                        .call()?;

                    log::info!(target: "auto_lsp::default::file_events", "Watched Files: Created - {url}");
                    session.db.add_file(file).map_err(RuntimeError::from)?;
                }
                Ok(())
            }
            FileChangeType::CHANGED => {
                let url: &lsp_types::Url = &file.uri;
                let file = session.db.get_file(&url).ok_or_else(|| {
                    RuntimeError::from(FileSystemError::FileUrlToFilePath { path: url.clone() })
                })?;

                if let Some(parser) = get_parser(url) {
                    file.update_full_fs(session, parser)
                        .map_err(RuntimeError::from)?;
                    log::info!(target: "auto_lsp::default::file_events", "Watched Files: Changed - {url}");
                }
                Ok(())
            }
            FileChangeType::DELETED => {
                let url = &file.uri;
                if session.db.get_file(&url).is_none() {
                    // The file is not in db, we can ignore this change
                    return Ok(());
                }

                let file = session.db.get_file(&url).unwrap();
                file.reset(&mut session.db).map_err(RuntimeError::from)?;

                log::info!(target: "auto_lsp::default::file_events", "Watched Files: Deleted - {}", &url);
                session.db.remove_file(&url).map_err(RuntimeError::from)
            }
            // Should never happen
            _ => Ok(()),
        }
    })
}
