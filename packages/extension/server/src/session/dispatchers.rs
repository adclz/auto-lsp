use lsp_server::{Connection, ExtractError, Message, Notification, Request, Response};
use serde::Serialize;

use super::Session;
/**
 * Code taken from https://github.com/oxlip-lang/oal/blob/b6741ff99f7c9338551e2067c0de7acd492fad00/oal-client/src/lsp/dispatcher.rs
 */
pub struct RequestDispatcher<'a, 'b> {
    session: &'a mut Session<'b>,
    req: Option<Request>,
}

impl<'a, 'b> RequestDispatcher<'a, 'b> {
    pub fn new(session: &'a mut Session<'b>, req: Request) -> Self {
        RequestDispatcher {
            session,
            req: Some(req),
        }
    }

    pub fn on<R, T>(
        &'a mut self,
        hook: impl Fn(&mut Session<'b>, R::Params) -> anyhow::Result<T>,
    ) -> anyhow::Result<&'a mut Self>
    where
        R: lsp_types::request::Request,
        R::Params: serde::de::DeserializeOwned,
        T: Serialize,
    {
        let req = match self.req.take() {
            Some(r) => r,
            None => return Ok(self),
        };

        match req.extract::<R::Params>(R::METHOD) {
            Ok((id, params)) => {
                let resp = Response {
                    id,
                    result: Some(serde_json::to_value(hook(self.session, params)?).unwrap()),
                    error: None,
                };
                self.session
                    .connection
                    .sender
                    .send(Message::Response(resp))?;
                Ok(self)
            }
            Err(err @ ExtractError::JsonError { .. }) => Err(anyhow::Error::from(err)),
            Err(ExtractError::MethodMismatch(req)) => {
                self.req = Some(req);
                Ok(self)
            }
        }
    }
}

pub struct NotificationDispatcher<'a, 'b> {
    session: &'a mut Session<'b>,
    not: Option<Notification>,
}

impl<'a, 'b> NotificationDispatcher<'a, 'b> {
    pub fn new(session: &'a mut Session<'b>, not: Notification) -> Self {
        NotificationDispatcher {
            session,
            not: Some(not),
        }
    }

    pub fn on<N>(
        &'a mut self,
        hook: impl Fn(&mut Session<'b>, N::Params) -> anyhow::Result<()>,
    ) -> anyhow::Result<&'a mut Self>
    where
        N: lsp_types::notification::Notification,
        N::Params: serde::de::DeserializeOwned,
    {
        let not = match self.not.take() {
            Some(r) => r,
            None => return Ok(self),
        };

        match not.extract::<N::Params>(N::METHOD) {
            Ok(params) => {
                hook(self.session, params)?;
                Ok(self)
            }
            Err(err @ ExtractError::JsonError { .. }) => return Err(anyhow::Error::from(err)),
            Err(ExtractError::MethodMismatch(not)) => {
                self.not = Some(not);
                return Ok(self);
            }
        }
    }
}
