use super::Session;
use lsp_server::{Request, Response};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;

type RequestCallback =
    Box<dyn Fn(&mut Session, serde_json::Value) -> anyhow::Result<serde_json::Value> + Send + Sync>;

#[derive(Default)]
pub struct RequestRegistry {
    handlers: HashMap<String, RequestCallback>,
}

impl RequestRegistry {
    pub fn register<R, F>(&mut self, handler: F)
    where
        R: lsp_types::request::Request,
        R::Params: DeserializeOwned,
        R::Result: Serialize,
        F: Fn(&mut Session, R::Params) -> anyhow::Result<R::Result> + Send + Sync + 'static,
    {
        let method = R::METHOD.to_string();
        let callback: RequestCallback = Box::new(move |session, params| {
            let parsed_params: R::Params = serde_json::from_value(params)?;
            let result = handler(session, parsed_params)?;
            Ok(serde_json::to_value(result)?)
        });

        self.handlers.insert(method, callback);
    }

    pub fn handle(&self, session: &mut Session, req: Request) -> anyhow::Result<Option<Response>> {
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
