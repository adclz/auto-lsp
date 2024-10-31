use lsp_server::Message;
use lsp_types::notification::Notification;

use super::Session;

impl<'a> Session<'a> {
    pub fn send_notification<N: Notification>(&self, params: N::Params) -> anyhow::Result<()> {
        let params = serde_json::to_value(&params).unwrap();
        let n = lsp_server::Notification {
            method: N::METHOD.into(),
            params,
        };
        self.connection.sender.send(Message::Notification(n))?;
        Ok(())
    }
}
