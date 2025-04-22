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

use super::{main_loop::Task, Session};
use auto_lsp_core::salsa::db::BaseDatabase;
use lsp_server::{Message, Notification};
use serde::de::DeserializeOwned;
use std::{collections::HashMap, panic::RefUnwindSafe, sync::Arc};

/// Callback for parallelized notifications
type Callback<Db> = Arc<
    dyn Fn(&Db, serde_json::Value) -> anyhow::Result<serde_json::Value>
        + Send
        + Sync
        + RefUnwindSafe
        + 'static,
>;

/// Callback for synchronous mutable notifications
type SyncMutCallback<Db> =
    Box<dyn Fn(&mut Session<Db>, serde_json::Value) -> anyhow::Result<serde_json::Value>>;

/// A registry for LSP notifications.
///
/// This registry allows you to register handlers for LSP notifications.
///
/// The handlers can be executed in a separate thread or synchronously with mutable access to the session.
///
/// The handlers are registered using the `on` and `on_mut` methods.
#[derive(Default)]
pub struct NotificationRegistry<Db: BaseDatabase> {
    handlers: HashMap<String, Callback<Db>>,
    sync_mut_handlers: HashMap<String, SyncMutCallback<Db>>,
}

impl<Db: BaseDatabase + Clone + Send + RefUnwindSafe> NotificationRegistry<Db> {
    /// Register a notification handler that will be pushed to the task pool.
    ///
    /// This handler is Cancelable and will be executed in a separate thread.
    ///
    /// Note that there is no retry mechanism for cancelled or failed notifications.
    pub fn on<N, F>(&mut self, handler: F) -> &mut Self
    where
        N: lsp_types::notification::Notification,
        N::Params: DeserializeOwned,
        F: Fn(&Db, N::Params) -> anyhow::Result<()> + Send + Sync + RefUnwindSafe + 'static,
    {
        let method = N::METHOD.to_string();
        let callback: Callback<Db> = Arc::new(move |session, params| {
            let parsed_params: N::Params = serde_json::from_value(params)?;
            handler(session, parsed_params)?;
            Ok(serde_json::to_value(())?)
        });

        self.handlers.insert(method, callback);
        self
    }

    /// Register a notification handler that will be executed synchronously with a mutable access to [`Session`].
    pub fn on_mut<N, F>(&mut self, handler: F) -> &mut Self
    where
        N: lsp_types::notification::Notification,
        N::Params: DeserializeOwned,
        F: Fn(&mut Session<Db>, N::Params) -> anyhow::Result<()> + Send + 'static,
    {
        let method = N::METHOD.to_string();
        let callback: SyncMutCallback<Db> = Box::new(move |session, params| {
            let parsed_params: N::Params = serde_json::from_value(params)?;
            handler(session, parsed_params)?;
            Ok(serde_json::to_value(())?)
        });

        self.sync_mut_handlers.insert(method, callback);
        self
    }

    pub(crate) fn get(&self, req: &Notification) -> Option<&Callback<Db>> {
        self.handlers.get(&req.method)
    }

    pub(crate) fn get_sync_mut(&self, req: &Notification) -> Option<&SyncMutCallback<Db>> {
        self.sync_mut_handlers.get(&req.method)
    }

    /// Push a notification handler to the task pool.
    pub(crate) fn exec(session: &Session<Db>, callback: &Callback<Db>, not: Notification) {
        let params = not.params;

        let snapshot = session.snapshot();
        let cb = Arc::clone(callback);
        session
            .task_pool
            .spawn(move |sender| match snapshot.with_db(|db| cb(&db, params)) {
                Err(e) => log::warn!("Cancelled notification: {}", e),
                Ok(result) => {
                    if let Err(e) = result {
                        sender.send(Task::NotificationError(e.into())).unwrap();
                    }
                }
            });
    }

    /// Execute a synchronous mutable notification handler immediatly.
    ///
    /// Depending on the handler, this may cancel parallelized notifications.
    pub(crate) fn exec_sync_mut(
        session: &mut Session<Db>,
        callback: &SyncMutCallback<Db>,
        not: Notification,
    ) -> anyhow::Result<()> {
        if let Err(e) = callback(session, not.params) {
            Self::handle_error(session, e)
        } else {
            Ok(())
        }
    }

    pub(crate) fn handle_error(session: &Session<Db>, error: anyhow::Error) -> anyhow::Result<()> {
        session
            .connection
            .sender
            .send(Message::Notification(Notification {
                method: "window/showMessage".to_string(),
                params: serde_json::json!({
                    "type": lsp_types::MessageType::ERROR,
                    "message": error.to_string(),
                }),
            }))?;
        Ok(())
    }
}
