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

use crate::server::session::Session;
use auto_lsp_core::salsa::db::FileManager;
use auto_lsp_core::{
    errors::{ExtensionError, RuntimeError},
    salsa::db::BaseDatabase,
};
use lsp_types::DidOpenTextDocumentParams;

pub fn open_text_document<Db: BaseDatabase>(
    session: &mut Session<Db>,
    params: DidOpenTextDocumentParams,
) -> Result<(), RuntimeError> {
    let url = &params.text_document.uri;

    if session.db.get_file(url).is_some() {
        // The file is already in db
        // We can ignore this change
        return Ok(());
    };

    let extension = &params.text_document.language_id;

    let extension = match session.extensions.get(extension) {
        Some(extension) => extension,
        None => {
            if session.extensions.values().any(|x| x == extension) {
                extension
            } else {
                return Err(ExtensionError::UnknownExtension {
                    extension: extension.clone(),
                    available: session.extensions.clone(),
                }
                .into());
            }
        }
    };

    let text = (session.text_fn)(params.text_document.text.clone());

    let parsers = session
        .init_options
        .parsers
        .get(extension.as_str())
        .ok_or_else(|| {
            RuntimeError::from(ExtensionError::UnknownParser {
                extension: extension.clone(),
                available: session.init_options.parsers.keys().cloned().collect(),
            })
        })?;

    log::info!("Did Open Text Document: Created - {}", url.to_string());
    session
        .db
        .add_file_from_texter(parsers, url, text)
        .map_err(|e| e.into())
}
