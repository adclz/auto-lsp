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
use lsp_server::Notification;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

type RequestCallback<Db> = Box<
    dyn Fn(&mut Session<Db>, serde_json::Value) -> anyhow::Result<serde_json::Value> + Send + Sync,
>;

#[derive(Default)]
pub struct NotificationRegistry<Db: BaseDatabase> {
    handlers: HashMap<String, RequestCallback<Db>>,
}

impl<Db: BaseDatabase> NotificationRegistry<Db> {
    pub fn register<N, F>(&mut self, handler: F) -> &mut Self
    where
        N: lsp_types::notification::Notification,
        N::Params: DeserializeOwned,
        F: Fn(&mut Session<Db>, N::Params) -> anyhow::Result<()> + Send + Sync + 'static,
    {
        let method = N::METHOD.to_string();
        let callback: RequestCallback<Db> = Box::new(move |session, params| {
            let parsed_params: N::Params = serde_json::from_value(params)?;
            handler(session, parsed_params)?;
            Ok(serde_json::to_value(())?)
        });

        self.handlers.insert(method, callback);
        self
    }

    pub fn handle(&self, session: &mut Session<Db>, req: Notification) -> anyhow::Result<()> {
        let params = req.params;
        if let Some(callback) = self.handlers.get(&req.method) {
            match callback(session, params) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        } else {
            Err(anyhow::anyhow!(
                "Method mismatch for notification '{}'",
                req.method
            ))
        }
    }
}
