use super::Session;
use lsp_server::Notification;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

type RequestCallback =
    Box<dyn Fn(&mut Session, serde_json::Value) -> anyhow::Result<serde_json::Value> + Send + Sync>;

#[derive(Default)]
pub struct NotificationRegistry {
    handlers: HashMap<String, RequestCallback>,
}

impl NotificationRegistry {
    pub fn register<N, F>(&mut self, handler: F)
    where
        N: lsp_types::notification::Notification,
        N::Params: DeserializeOwned,
        F: Fn(&mut Session, N::Params) -> anyhow::Result<()> + Send + Sync + 'static,
    {
        let method = N::METHOD.to_string();
        let callback: RequestCallback = Box::new(move |session, params| {
            let parsed_params: N::Params = serde_json::from_value(params)?;
            let result = handler(session, parsed_params)?;
            Ok(serde_json::to_value(result)?)
        });

        self.handlers.insert(method, callback);
    }

    pub fn handle(&self, session: &mut Session, req: Notification) -> anyhow::Result<()> {
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
