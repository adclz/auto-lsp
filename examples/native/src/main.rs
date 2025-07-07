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

use ast_python::db::PYTHON_PARSERS;
use auto_lsp::default::db::{BaseDatabase, BaseDb};
use auto_lsp::default::server::capabilities::WORKSPACE_PROVIDER;
use auto_lsp::default::server::file_events::{changed_watched_files, open_text_document};
use auto_lsp::default::server::workspace_init::WorkspaceInit;
use auto_lsp::lsp_server::{self, Connection};
use auto_lsp::lsp_types::notification::{
    Cancel, DidChangeTextDocument, DidChangeWatchedFiles, DidCloseTextDocument,
    DidOpenTextDocument, DidSaveTextDocument, LogTrace, SetTrace,
};
use auto_lsp::lsp_types::ServerCapabilities;
use auto_lsp::server::notification_registry::NotificationRegistry;
use auto_lsp::server::options::InitOptions;
use auto_lsp::server::request_registry::RequestRegistry;
use auto_lsp::server::Session;
use auto_lsp::{anyhow, lsp_types};
use native_lsp::requests::GetWorkspaceFiles;
use std::error::Error;
use std::panic::RefUnwindSafe;

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

    let (mut session, params) = Session::create(
        InitOptions {
            parsers: &PYTHON_PARSERS,
            capabilities: ServerCapabilities {
                workspace: WORKSPACE_PROVIDER.clone(),
                ..Default::default()
            },
            server_info: None,
        },
        connection,
        db,
    )?;

    let mut request_registry = RequestRegistry::<BaseDb>::default();
    let mut notification_registry = NotificationRegistry::<BaseDb>::default();

    request_registry.on::<GetWorkspaceFiles, _>(|s, _| Ok(s.get_urls()));

    // Initialize the session with the client's initialization options.
    // This will also add all documents, parse and send diagnostics.
    let init_results = session.init_workspace(params)?;
    if !init_results.is_empty() {
        init_results.into_iter().for_each(|result| {
            if let Err(err) = result {
                eprintln!("{}", err);
            }
        });
    };

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    session.main_loop(
        &request_registry,
        on_notifications(&mut notification_registry),
    )?;
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("Shutting down server");
    Ok(())
}

fn on_notifications<Db: BaseDatabase + Clone + RefUnwindSafe>(
    registry: &mut NotificationRegistry<Db>,
) -> &mut NotificationRegistry<Db> {
    registry
        .on_mut::<DidOpenTextDocument, _>(|s, p| Ok(open_text_document(s, p)?))
        .on_mut::<DidChangeTextDocument, _>(|s, p| {
            let file =
                s.db.get_file(&p.text_document.uri)
                    .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;
            file.update_edit(&mut s.db, &p)?;
            Ok(())
        })
        .on_mut::<DidChangeWatchedFiles, _>(|s, p| Ok(changed_watched_files(s, p)?))
        .on_mut::<Cancel, _>(|s, p| {
            let id: lsp_server::RequestId = match p.id {
                lsp_types::NumberOrString::Number(id) => id.into(),
                lsp_types::NumberOrString::String(id) => id.into(),
            };
            if let Some(response) = s.req_queue.incoming.cancel(id) {
                s.connection.sender.send(response.into())?;
            }
            Ok(())
        })
        .on::<DidSaveTextDocument, _>(|_s, _p| Ok(()))
        .on::<DidCloseTextDocument, _>(|_s, _p| Ok(()))
        .on::<SetTrace, _>(|_s, _p| Ok(()))
        .on::<LogTrace, _>(|_s, _p| Ok(()))
}
