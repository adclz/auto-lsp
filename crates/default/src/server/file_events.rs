/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/

use auto_lsp_core::errors::DataBaseError;
use auto_lsp_core::errors::FileSystemError;
use auto_lsp_core::errors::RuntimeError;
use auto_lsp_server::Session;
use lsp_types::DidChangeTextDocumentParams;
use lsp_types::DidChangeWatchedFilesParams;
use lsp_types::DidOpenTextDocumentParams;
use lsp_types::FileChangeType;
use salsa::Setter;

use crate::db::BaseDatabase;
use crate::db::{file::File, FileManager};

/// Handles a [`DidOpenTextDocument`] request.
///
/// If the file already exists in the database, it updates its version.
///
/// If it does not exist, it creates a new file from the text document parameters.
pub fn open_text_document<Db: BaseDatabase>(
    session: &mut Session<Db>,
    params: DidOpenTextDocumentParams,
) -> Result<(), RuntimeError> {
    let url = &params.text_document.uri;

    match session.db.get_file(url) {
        Some(file) => {
            log::info!("Did Open Text Document: Already exists - {url}");
            file.set_version(&mut session.db)
                .to(Some(params.text_document.version));
            Ok(())
        }
        None => {
            let file = File::from_text_doc()
                .doc(&params.text_document)
                .session(session)
                .call()?;

            log::info!("Did Open Text Document: Created - {url}");
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

    log::info!("Did Change Text Document: {}", params.text_document.uri);
    Ok(file.update_edit(&mut session.db, &params)?)
}

/// Handle the watched files change notification.
pub fn changed_watched_files<Db: BaseDatabase>(
    session: &mut Session<Db>,
    params: DidChangeWatchedFilesParams,
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
                let file = File::from_fs().session(session).url(&url).call()?;

                log::info!("Watched Files: Created - {url}");
                session.db.add_file(file).map_err(RuntimeError::from)
            }
            FileChangeType::CHANGED => {
                let url: &lsp_types::Url = &file.uri;
                let file = session.db.get_file(&url).ok_or_else(|| {
                    RuntimeError::from(FileSystemError::FileUrlToFilePath { path: url.clone() })
                })?;

                log::info!("Watched Files: Changed - {url}");
                file.update_full_fs(session).map_err(RuntimeError::from)
            }
            FileChangeType::DELETED => {
                let url = &file.uri;
                if session.db.get_file(&url).is_none() {
                    // The file is not in db, we can ignore this change
                    return Ok(());
                }

                let file = session.db.get_file(&url).unwrap();
                file.reset(&mut session.db).map_err(RuntimeError::from)?;

                log::info!("Watched Files: Deleted - {}", &url);
                session.db.remove_file(&url).map_err(RuntimeError::from)
            }
            // Should never happen
            _ => Ok(()),
        }
    })
}
