/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/

use std::{fs::File, io::Read};

use crate::server::session::Session;
use auto_lsp_core::salsa::db::BaseDatabase;
use lsp_types::{DidChangeWatchedFilesParams, FileChangeType};

/// Handle the watched files change notification.
///
/// The differences between this and the document requests is that the watched files are not necessarily modified by the client.
///
/// Some changes can be made by external tools, github, someone editing the project with NotePad while the IDE is active, etc ...
pub fn changed_watched_files<Db: BaseDatabase>(
    session: &mut Session<Db>,
    params: DidChangeWatchedFilesParams,
) -> anyhow::Result<()> {
    params.changes.iter().try_for_each(|file| match file.typ {
        FileChangeType::CREATED => {
            let uri = &file.uri;

            if session.db.get_file(uri).is_some() {
                // The file is already in db
                // We can ignore this change
                return Ok(());
            };
            let file_path = uri
                .to_file_path()
                .map_err(|_| anyhow::anyhow!("Failed to read file {}", uri.to_string()))?;

            let (parsers, url, text) = session.read_file(&file_path)?;
            log::info!("Watched Files: Created - {}", uri.to_string());
            session.db.add_file_from_texter(parsers, &url, text)
        }
        FileChangeType::CHANGED => {
            let uri = &file.uri;
            let file_path = uri
                .to_file_path()
                .map_err(|_| anyhow::anyhow!("Failed to read file {}", uri.to_string()))?;
            let open_file = File::open(file_path)?;

            match session.db.get_file(uri) {
                Some(file) => {
                    if is_file_content_different(
                        &open_file,
                        &file.document(&session.db).read().texter.text,
                    )? {
                        session.db.remove_file(uri)?;
                        let file_path = uri.to_file_path().map_err(|_| {
                            anyhow::anyhow!("Failed to read file {}", uri.to_string())
                        })?;
                        log::info!("Watched Files: Changed - {}", uri.to_string());
                        let (parsers, url, text) = session.read_file(&file_path)?;
                        session.db.add_file_from_texter(parsers, &url, text)
                    } else {
                        // The file is already in db and the content is the same
                        // We can ignore this change
                        Ok(())
                    }
                }
                None => Ok(()),
            }
        }
        FileChangeType::DELETED => {
            log::info!("Watched Files: Deleted - {}", file.uri.to_string());
            session.db.remove_file(&file.uri)
        }
        // Should never happen
        _ => Ok(()),
    })
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
