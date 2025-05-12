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
use lsp_server::{Message, Request, RequestId, Response};
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, panic::RefUnwindSafe, sync::Arc};

/// Callback for parallelized requests
type Callback<Db> = Arc<
    dyn Fn(&Db, serde_json::Value) -> anyhow::Result<serde_json::Value>
        + Send
        + Sync
        + RefUnwindSafe
        + 'static,
>;

/// Callback for synchronous mutable requests
type SyncMutCallback<Db> =
    Box<dyn Fn(&mut Session<Db>, serde_json::Value) -> anyhow::Result<serde_json::Value>>;

/// A registry for LSP requests.
///
/// This registry allows you to register handlers for LSP requests.
///
/// The handlers can be executed in a separate thread or synchronously with mutable access to the session.
///
/// The handlers are registered using the `on` and `on_mut` methods.
#[derive(Default)]
pub struct RequestRegistry<Db: BaseDatabase> {
    handlers: HashMap<String, Callback<Db>>,
    sync_mut_handlers: HashMap<String, SyncMutCallback<Db>>,
}

impl<Db: BaseDatabase + Clone + Send + RefUnwindSafe> RequestRegistry<Db> {
    pub fn on<R, F>(&mut self, handler: F) -> &mut Self
    where
        R: lsp_types::request::Request,
        R::Params: DeserializeOwned,
        R::Result: Serialize,
        F: Fn(&Db, R::Params) -> anyhow::Result<R::Result> + Send + Sync + RefUnwindSafe + 'static,
    {
        let method = R::METHOD.to_string();
        let callback: Callback<Db> = Arc::new(move |session, params| {
            let parsed_params: R::Params = serde_json::from_value(params)?;
            let result = handler(session, parsed_params)?;
            Ok(serde_json::to_value(result)?)
        });

        self.handlers.insert(method, callback);
        self
    }

    /// Register a synchronous mutable request handler.
    ///
    /// This handler is executed synchronously with mutable access to [`Session`].
    ///
    /// Note that there is no retry mechanism for cancelled or failed requests.
    pub fn on_mut<R, F>(&mut self, handler: F) -> &mut Self
    where
        R: lsp_types::request::Request,
        R::Params: DeserializeOwned,
        R::Result: Serialize,
        F: Fn(&mut Session<Db>, R::Params) -> anyhow::Result<R::Result> + Send + Sync + 'static,
    {
        let method = R::METHOD.to_string();
        let callback: SyncMutCallback<Db> = Box::new(move |session, params| {
            let parsed_params: R::Params = serde_json::from_value(params)?;
            let result = handler(session, parsed_params)?;
            Ok(serde_json::to_value(result)?)
        });

        self.sync_mut_handlers.insert(method, callback);
        self
    }

    pub(crate) fn get(&self, req: &Request) -> Option<&Callback<Db>> {
        self.handlers.get(&req.method)
    }

    pub(crate) fn get_sync_mut(&self, req: &Request) -> Option<&SyncMutCallback<Db>> {
        self.sync_mut_handlers.get(&req.method)
    }

    /// Push a request handler to the task pool.
    pub(crate) fn exec(session: &Session<Db>, callback: &Callback<Db>, req: Request) {
        let params = req.params;
        let id = req.id.clone();

        let snapshot = session.snapshot();
        let cb = Arc::clone(callback);
        session.task_pool.spawn(move |sender| {
            let cb = cb.clone();
            match snapshot.with_db(|db| cb(db, params)) {
                Err(e) => {
                    log::warn!("Cancelled request: {}", e);
                }
                Ok(result) => match result {
                    Ok(result) => sender
                        .send(Task::Response(Response {
                            id,
                            result: Some(result),
                            error: None,
                        }))
                        .unwrap(),
                    Err(e) => {
                        sender
                            .send(Task::Response(Self::response_error(id, e)))
                            .unwrap();
                    }
                },
            }
        });
    }

    /// Execute a synchronous mutable request handler.
    ///
    /// Depending on the handler, this may cancel parallelized requests.
    pub(crate) fn exec_sync_mut(
        session: &mut Session<Db>,
        callback: &SyncMutCallback<Db>,
        req: Request,
    ) -> anyhow::Result<()> {
        if let Err(e) = callback(session, req.params.clone()) {
            Self::complete(session, Self::response_error(req.id, e))
        } else {
            Ok(())
        }
    }

    pub(crate) fn complete(
        session: &mut Session<Db>,
        response: lsp_server::Response,
    ) -> anyhow::Result<()> {
        let id = response.id.clone();
        if !session.req_queue.incoming.is_completed(&id) {
            session.req_queue.incoming.complete(&id);
        }
        Ok(session
            .connection
            .sender
            .send(Message::Response(response))?)
    }

    pub(crate) fn response_error(id: RequestId, error: anyhow::Error) -> lsp_server::Response {
        Response {
            id,
            result: None,
            error: Some(lsp_server::ResponseError {
                code: -32803, // RequestFailed
                message: error.to_string(),
                data: None,
            }),
        }
    }

    pub(crate) fn request_mismatch(id: RequestId, error: anyhow::Error) -> lsp_server::Response {
        Response {
            id,
            result: None,
            error: Some(lsp_server::ResponseError {
                code: -32601, // MethodNotFound
                message: format!("Method mismatch for request '{error}'"),
                data: None,
            }),
        }
    }
}
