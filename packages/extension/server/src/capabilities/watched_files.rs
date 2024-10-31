use lsp_types::{DidChangeWatchedFilesParams, FileChangeType};

use crate::session::Session;

impl<'a> Session<'a> {
    pub fn changed_watched_files(
        &mut self,
        params: DidChangeWatchedFilesParams,
    ) -> anyhow::Result<()> {
        // Todo!
        params.changes.iter().for_each(|file| match file.typ {
            FileChangeType::CREATED => {}
            FileChangeType::CHANGED => {}
            FileChangeType::DELETED => {}
            _ => {}
        });

        Ok(())
    }
}
