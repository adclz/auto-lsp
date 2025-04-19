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

use auto_lsp::core::salsa::db::{BaseDatabase, BaseDb};
use auto_lsp::lsp_server::{self, Connection};
use auto_lsp::lsp_types;
use auto_lsp::lsp_types::notification::{
    Cancel, DidChangeTextDocument, DidChangeWatchedFiles, DidCloseTextDocument,
    DidOpenTextDocument, DidSaveTextDocument, LogTrace, SetTrace,
};
use auto_lsp::lsp_types::request::DocumentDiagnosticRequest;
use auto_lsp::python::PYTHON_PARSERS;
use auto_lsp::server::capabilities::{changed_watched_files, get_diagnostics, open_text_document};
use auto_lsp::server::RequestRegistry;
use auto_lsp::server::{InitOptions, LspOptions, NotificationRegistry, Session};
use native_lsp::requests::GetWorkspaceFiles;
use std::error::Error;
use std::ops::Deref;

pub trait ExtendDb: BaseDatabase {
    fn get_urls(&self) -> Vec<String> {
        self.get_files()
            .iter()
            .map(|file| file.url(self).to_string())
            .collect()
    }
}

impl ExtendDb for BaseDb {}

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let (connection, io_threads) = Connection::stdio();
    let db = BaseDb::default();

    let mut session = Session::create(
        InitOptions {
            parsers: &PYTHON_PARSERS,
            lsp_options: LspOptions {
                ..Default::default()
            },
        },
        connection,
        db,
    )?;

    let mut request_registry = RequestRegistry::<BaseDb>::default();
    let mut notification_registry = NotificationRegistry::<BaseDb>::default();

    request_registry.register::<GetWorkspaceFiles, _>(|session, _| Ok(session.db.get_urls()));

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    session.main_loop(
        &request_registry,
        register_notifications(&mut notification_registry),
    )?;
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("Shutting down server");
    Ok(())
}

fn register_notifications<Db: BaseDatabase>(
    registry: &mut NotificationRegistry<Db>,
) -> &mut NotificationRegistry<Db> {
    registry
        .register::<DidOpenTextDocument, _>(|s, p| Ok(open_text_document(s, p)?))
        .register::<DidChangeTextDocument, _>(|s, p| {
            s.db.update(&p.text_document.uri, &p.content_changes)?;
            Ok(())
        })
        .register::<DidChangeWatchedFiles, _>(|s, p| Ok(changed_watched_files(s, p)?))
        .register::<Cancel, _>(|s, p| {
            let id: lsp_server::RequestId = match p.id {
                lsp_types::NumberOrString::Number(id) => id.into(),
                lsp_types::NumberOrString::String(id) => id.into(),
            };
            if let Some(response) = s.req_queue.incoming.cancel(id) {
                s.connection.sender.send(response.into())?;
            }
            Ok(())
        })
        .register::<DidSaveTextDocument, _>(|s, p| Ok(()))
        .register::<DidCloseTextDocument, _>(|s, p| Ok(()))
        .register::<SetTrace, _>(|s, p| Ok(()))
        .register::<LogTrace, _>(|s, p| Ok(()))
}
