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
