use crate::{main_loop::Task, vendored::intent::ThreadIntent};

use super::Session;
use lsp_server::{Message, Notification};
use serde::de::DeserializeOwned;
use std::{collections::HashMap, panic::RefUnwindSafe, sync::Arc};

/// Callback for parallelized notifications
type Callback<Db> = Arc<
    dyn Fn(&Db, serde_json::Value) -> anyhow::Result<()> + Send + Sync + RefUnwindSafe + 'static,
>;

/// Callback for synchronous mutable notifications
type SyncMutCallback<Db> = Box<dyn Fn(&mut Session<Db>, serde_json::Value) -> anyhow::Result<()>>;

type CallbackWithIntent<Db> = (Callback<Db>, ThreadIntent);

/// A registry for LSP notifications.
///
/// This registry allows you to register handlers for LSP notifications.
///
/// The handlers can be executed in a separate thread or synchronously with mutable access to the session.
///
/// The handlers are registered using the `on` and `on_mut` methods.
#[derive(Default)]
pub struct NotificationRegistry<Db: salsa::Database> {
    handlers: HashMap<String, CallbackWithIntent<Db>>,
    sync_mut_handlers: HashMap<String, SyncMutCallback<Db>>,
}

impl<Db: salsa::Database + Clone + Send + RefUnwindSafe> NotificationRegistry<Db> {
    /// Register a notification handler that will be pushed to the task pool.
    ///
    /// This handler is Cancelable and will be executed in a separate thread.
    ///
    /// Note that there is no retry mechanism for cancelled or failed notifications.
    pub fn on<N, F>(&mut self, intent: ThreadIntent, handler: F) -> &mut Self
    where
        N: lsp_types::notification::Notification,
        N::Params: DeserializeOwned,
        F: Fn(&Db, N::Params) -> anyhow::Result<()> + Send + Sync + RefUnwindSafe + 'static,
    {
        let method = N::METHOD.to_string();
        let callback: Callback<Db> = Arc::new(move |session, params| {
            let parsed_params: N::Params = serde_json::from_value(params)?;
            handler(session, parsed_params)?;
            Ok(())
        });

        self.handlers.insert(method, (callback, intent));
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
            Ok(())
        });

        self.sync_mut_handlers.insert(method, callback);
        self
    }

    pub(crate) fn get(&self, req: &Notification) -> Option<&CallbackWithIntent<Db>> {
        self.handlers.get(&req.method)
    }

    pub(crate) fn get_sync_mut(&self, req: &Notification) -> Option<&SyncMutCallback<Db>> {
        self.sync_mut_handlers.get(&req.method)
    }

    /// Push a notification handler to the task pool.
    pub(crate) fn exec(
        session: &Session<Db>,
        callback: &CallbackWithIntent<Db>,
        not: Notification,
    ) {
        let params = not.params;

        let db = session.db.clone();
        let cb = Arc::clone(&callback.0);
        let intent = callback.1;
        let sender = session.task_sender.clone();
        let on_error = session.on_error;
        session.task_pool.spawn(
            intent,
            std::panic::AssertUnwindSafe(move || {
                match salsa::Cancelled::catch(|| cb(&db, params)) {
                    Err(e) => log::warn!("Cancelled notification: {e}"),
                    Ok(result) => {
                        if let Err(e) = result {
                            if let Some(on_error) = on_error {
                                on_error(&e);
                            }
                            sender.send(Task::NotificationError(e)).unwrap();
                        }
                    }
                }
            }),
        );
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
