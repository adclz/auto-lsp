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

use super::Session;
use auto_lsp_core::salsa::db::BaseDatabase;
use lsp_server::{Request, Response};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;

type RequestCallback<Db> = Box<
    dyn Fn(&mut Session<Db>, serde_json::Value) -> anyhow::Result<serde_json::Value> + Send + Sync,
>;

#[derive(Default)]
pub struct RequestRegistry<Db: BaseDatabase> {
    handlers: HashMap<String, RequestCallback<Db>>,
}

impl<Db: BaseDatabase> RequestRegistry<Db> {
    pub fn register<R, F>(&mut self, handler: F) -> &mut Self
    where
        R: lsp_types::request::Request,
        R::Params: DeserializeOwned,
        R::Result: Serialize,
        F: Fn(&Session<Db>, R::Params) -> anyhow::Result<R::Result> + Send + Sync + 'static,
    {
        let method = R::METHOD.to_string();
        let callback: RequestCallback<Db> = Box::new(move |session, params| {
            let parsed_params: R::Params = serde_json::from_value(params)?;
            let result = handler(session, parsed_params)?;
            Ok(serde_json::to_value(result)?)
        });

        self.handlers.insert(method, callback);
        self
    }

    pub fn register_mut<R, F>(&mut self, handler: F) -> &mut Self
    where
        R: lsp_types::request::Request,
        R::Params: DeserializeOwned,
        R::Result: Serialize,
        F: Fn(&mut Session<Db>, R::Params) -> anyhow::Result<R::Result> + Send + Sync + 'static,
    {
        let method = R::METHOD.to_string();
        let callback: RequestCallback<Db> = Box::new(move |session, params| {
            let parsed_params: R::Params = serde_json::from_value(params)?;
            let result = handler(session, parsed_params)?;
            Ok(serde_json::to_value(result)?)
        });

        self.handlers.insert(method, callback);
        self
    }

    pub fn handle(
        &self,
        session: &mut Session<Db>,
        req: Request,
    ) -> anyhow::Result<Option<Response>> {
        let id = req.id.clone();
        let params = req.params;
        if let Some(callback) = self.handlers.get(&req.method) {
            match callback(session, params) {
                Ok(result) => Ok(Some(Response {
                    id,
                    result: Some(result),
                    error: None,
                })),
                Err(err) => Ok(Some(Response {
                    id,
                    result: None,
                    error: Some(lsp_server::ResponseError {
                        code: -32803, // RequestFailed
                        message: err.to_string(),
                        data: None,
                    }),
                })),
            }
        } else {
            Ok(Some(Response {
                id,
                result: None,
                error: Some(lsp_server::ResponseError {
                    code: -32601, // MethodNotFound
                    message: format!("Method mismatch for request '{}'", req.method),
                    data: None,
                }),
            }))
        }
    }
}
