use lsp_types::Url;

use crate::session::Session;

impl Session {
    pub fn delete_document(&mut self, uri: &Url) -> anyhow::Result<()> {
        self.workspaces
            .remove(uri)
            .ok_or_else(|| anyhow::format_err!("Workspace not found"))?;
        Ok(())
    }
}
