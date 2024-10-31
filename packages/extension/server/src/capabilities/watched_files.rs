use std::{fs::File, io::Read};

use lsp_types::{DidChangeWatchedFilesParams, FileChangeType};

use crate::session::Session;

impl<'a> Session<'a> {
    /// Handle the watched files change notification.
    /// The differences between this and the document requests is that the watched files are not modified by the client.
    ///
    /// Some changes can be made by external tools, github, someone editing the project with NotePad while the IDE is active, etc ...
    ///
    /// This notification has to be treated carefully, as it can be a source of bugs.
    pub fn changed_watched_files(
        &mut self,
        params: DidChangeWatchedFilesParams,
    ) -> anyhow::Result<()> {
        params.changes.iter().try_for_each(|file| match file.typ {
            FileChangeType::CREATED => {
                let uri = &file.uri;
                if self.workspaces.contains_key(&uri) {
                    // The file is already in the workspace
                    // We can ignore this change
                    return Ok(());
                };
                // TODO: define a helper fn to extract extension from uri
                let language_id = file.uri.as_str().split(".").last().unwrap().to_string();

                let mut open_file = File::open(uri.to_file_path().unwrap()).unwrap();
                let mut buffer = String::new();
                open_file.read_to_string(&mut buffer).unwrap();
                self.add_document(uri, &language_id, &buffer)
            }
            FileChangeType::CHANGED => {
                // Note: this is a naive implementation
                // Since the the client can't send the actual changes, we have to recreate the whole workspace the file
                let uri = &file.uri;
                // TODO: define a helper fn to extract extension from uri
                let language_id = file.uri.as_str().split(".").last().unwrap().to_string();

                let mut open_file = File::open(uri.to_file_path().unwrap()).unwrap();
                let mut buffer = String::new();
                open_file.read_to_string(&mut buffer).unwrap();
                self.add_document(uri, &language_id, &buffer)
            }
            FileChangeType::DELETED => {
                let uri = &file.uri;
                self.delete_document(uri)
            }
            // Should never happen
            _ => Ok(()),
        })
    }
}
