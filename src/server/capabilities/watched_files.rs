use std::{fs::File, io::Read};

use lsp_types::{DidChangeWatchedFilesParams, FileChangeType};

use crate::server::session::{Session, WORKSPACE};

impl Session {
    /// Handle the watched files change notification.
    ///
    /// The differences between this and the document requests is that the watched files are not necessarily modified by the client.
    ///
    /// Some changes can be made by external tools, github, someone editing the project with NotePad while the IDE is active, etc ...
    pub(crate) fn changed_watched_files(
        &mut self,
        params: DidChangeWatchedFilesParams,
    ) -> anyhow::Result<()> {
        params.changes.iter().try_for_each(|file| match file.typ {
            FileChangeType::CREATED => {
                let uri = &file.uri;
                let mut workspace = WORKSPACE.lock();

                if workspace.roots.contains_key(uri) {
                    // The file is already in the root
                    // We can ignore this change
                    return Ok(());
                };
                let file_path = uri
                    .to_file_path()
                    .map_err(|_| anyhow::anyhow!("Failed to read file {}", uri.to_string()))?;

                let (_url, root, document) = self.file_to_root(&file_path)?;
                workspace.roots.insert(uri.clone(), (root, document));
                Ok(())
            }
            FileChangeType::CHANGED => {
                let uri = &file.uri;
                let mut workspace = WORKSPACE.lock();
                let file_path = uri
                    .to_file_path()
                    .map_err(|_| anyhow::anyhow!("Failed to read file {}", uri.to_string()))?;
                let open_file = File::open(file_path)?;

                if workspace.roots.contains_key(uri) {
                    // The file is already in the root
                    // We compare the stored document with the new file content
                    // If there's a single byte difference, we replace the document
                    if (is_file_content_different(
                        &open_file,
                        &workspace.roots.get(uri).unwrap().1.texter.text,
                    ))? {
                        workspace.roots.remove(uri);
                        let file_path = uri.to_file_path().map_err(|_| {
                            anyhow::anyhow!("Failed to read file {}", uri.to_string())
                        })?;

                        let (_url, root, document) = self.file_to_root(&file_path)?;
                        workspace.roots.insert(uri.clone(), (root, document));
                    }
                }

                Ok(())
            }
            FileChangeType::DELETED => {
                let mut root = WORKSPACE.lock();
                let uri = &file.uri;
                root.roots.remove(uri);
                Ok(())
            }
            // Should never happen
            _ => Ok(()),
        })
    }
}

/// Compare the equality of a file with a string using buffers
fn is_file_content_different(file: &File, content: &str) -> std::io::Result<bool> {
    let mut file = std::io::BufReader::new(file);
    let content_bytes = content.as_bytes();
    let mut buffer = [0u8; 1024];
    let mut index = 0;

    loop {
        let bytes_read = file.read(&mut buffer)?;

        // Compare the file's chunk with the corresponding slice of the content
        if content_bytes.len() < index + bytes_read
            || content_bytes[index..index + bytes_read] != buffer[..bytes_read]
        {
            return Ok(true); // There's a difference
        }

        index += bytes_read;

        if bytes_read == 0 {
            break;
        }
    }

    // Ensure the content doesn't have extra bytes at the end
    Ok(index != content_bytes.len())
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    #[test]
    fn test_file_content_different() {
        use super::is_file_content_different;
        use std::fs::File;
        use std::io::Write;
        use tempfile::tempdir;

        let tmp_dir = tempdir().unwrap();

        let file_path = tmp_dir.path().join("my-temporary-note.txt");

        let mut tmp_file = File::create(&file_path).unwrap();
        tmp_file.write_all(b"Hello, World!").unwrap();

        // Bad file descriptor
        drop(tmp_file);
        let tmp_file = File::open(&file_path).unwrap();

        assert!(!is_file_content_different(&tmp_file, "Hello, World!").unwrap());
        assert!(is_file_content_different(&tmp_file, "Hello, World").unwrap());
        assert!(is_file_content_different(&tmp_file, "Hello,_World!").unwrap());
        assert!(is_file_content_different(&tmp_file, "Hello, World!!").unwrap());
    }
}
