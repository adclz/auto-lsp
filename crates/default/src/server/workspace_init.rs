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

use auto_lsp_core::errors::{ExtensionError, FileSystemError, RuntimeError};
use auto_lsp_core::parsers::Parsers;
use auto_lsp_server::Session;
use lsp_types::{InitializeParams, Url};
use rayon::prelude::*;
use std::path::PathBuf;
use std::{fs::File, io::Read};
use texter::core::text::Text;
use walkdir::WalkDir;

use crate::db::BaseDatabase;
use crate::db::FileManager;

pub trait WorkspaceInit {
    fn init_workspace(
        &mut self,
        params: InitializeParams,
    ) -> Result<Vec<Result<(), RuntimeError>>, RuntimeError>;

    fn read_file(&self, file: &PathBuf) -> Result<(&'static Parsers, Url, Text), FileSystemError>;
}

impl<Db: BaseDatabase> WorkspaceInit for Session<Db> {
    /// Initializes the workspace by loading files and associating them with parsers.
    fn init_workspace(
        &mut self,
        params: InitializeParams,
    ) -> Result<Vec<Result<(), RuntimeError>>, RuntimeError> {
        let mut errors: Vec<Result<(), RuntimeError>> = vec![];

        if let Some(folders) = params.workspace_folders {
            let files = folders
                .into_iter()
                .flat_map(|folder| {
                    WalkDir::new(folder.uri.path())
                        .into_iter()
                        .filter_map(Result::ok)
                        .filter(|entry| {
                            entry.file_type().is_file()
                                && entry.path().extension().is_some_and(|ext| {
                                    self.extensions.contains_key(ext.to_string_lossy().as_ref())
                                })
                        })
                })
                .collect::<Vec<_>>();

            errors.extend(rayon_par_bridge::par_bridge(
                16,
                files.into_par_iter(),
                |file_iter| {
                    file_iter
                        .map(|file| match self.read_file(&file.into_path()) {
                            Ok((parsers, url, text)) => self
                                .db
                                .add_file_from_texter(parsers, &url, text)
                                .map_err(RuntimeError::from),
                            Err(err) => Err(RuntimeError::from(err)),
                        })
                        .collect::<Vec<_>>()
                },
            ));
        }

        Ok(errors)
    }

    fn read_file(&self, file: &PathBuf) -> Result<(&'static Parsers, Url, Text), FileSystemError> {
        let url = Url::from_file_path(file)
            .map_err(|_| FileSystemError::FilePathToUrl { path: file.clone() })?;

        let mut open_file = File::open(file).map_err(|e| FileSystemError::FileOpen {
            path: url.clone(),
            error: e.to_string(),
        })?;
        let mut buffer = String::new();
        open_file
            .read_to_string(&mut buffer)
            .map_err(|e| FileSystemError::FileRead {
                path: url.clone(),
                error: e.to_string(),
            })?;

        let extension = get_extension(&url)?;

        let text = (self.text_fn)(buffer.to_string());
        let extension = match self.extensions.get(&extension) {
            Some(extension) => extension,
            None => {
                return Err(FileSystemError::from(ExtensionError::UnknownExtension {
                    extension: extension.clone(),
                    available: self.extensions.clone(),
                }))
            }
        };

        let parsers = self
            .init_options
            .parsers
            .get(extension.as_str())
            .ok_or_else(|| {
                FileSystemError::from(ExtensionError::UnknownParser {
                    extension: extension.clone(),
                    available: self.init_options.parsers.keys().cloned().collect(),
                })
            })?;
        Ok((parsers, url, text))
    }
}

/// Get the extension of a file from a [`Url`] path
#[cfg(windows)]
pub(crate) fn get_extension(path: &Url) -> Result<String, FileSystemError> {
    // Ensure the host is either empty or "localhost" on Windows
    if let Some(host) = path.host_str() {
        if !host.is_empty() && host != "localhost" {
            return Err(FileSystemError::FileUrlHost {
                host: host.to_string(),
                path: path.clone(),
            });
        }
    }

    path.to_file_path()
        .map_err(|_| FileSystemError::FileUrlToFilePath { path: path.clone() })?
        .extension()
        .map_or_else(
            || Err(FileSystemError::FileExtension { path: path.clone() }),
            |ext| Ok(ext.to_string_lossy().to_string()),
        )
}

#[cfg(not(windows))]
pub(crate) fn get_extension(path: &Url) -> Result<String, FileSystemError> {
    path.to_file_path()
        .map_err(|_| FileSystemError::FileUrlToFilePath { path: path.clone() })?
        .extension()
        .map_or_else(
            || Err(FileSystemError::FileExtension { path: path.clone() }),
            |ext| Ok(ext.to_string_lossy().to_string()),
        )
}

#[cfg(test)]
mod tests {
    use super::get_extension;
    use auto_lsp_core::errors::FileSystemError;
    use lsp_types::Url;

    #[cfg(windows)]
    #[test]
    fn test_get_extension_windows() {
        // Valid Windows paths
        assert_eq!(
            get_extension(&Url::parse("file:///C:/path/to/file.rs").unwrap())
                .unwrap()
                .as_str(),
            "rs"
        );

        assert_eq!(
            get_extension(&Url::parse("file:///C:/path/to/file.with.multiple.dots").unwrap())
                .unwrap()
                .as_str(),
            "dots"
        );

        // Empty extension
        assert_eq!(
            get_extension(&Url::parse("file:///C:/path/to/file").unwrap()),
            Err(FileSystemError::FileExtension {
                path: Url::parse("file:///C:/path/to/file").unwrap()
            })
        );
    }

    #[cfg(not(windows))]
    #[test]
    fn test_get_extension_non_windows() {
        // Valid Linux/Unix paths

        assert_eq!(
            get_extension(&Url::parse("file:///path/to/file.rs").unwrap())
                .unwrap()
                .as_str(),
            "rs"
        );

        assert_eq!(
            get_extension(&Url::parse("file:///path/to/file.with.multiple.dots").unwrap())
                .unwrap()
                .as_str(),
            "dots"
        );

        // Empty extension
        assert_eq!(
            get_extension(&Url::parse("file:///path/to/file").unwrap()),
            Err(FileSystemError::FileExtension {
                path: Url::parse("file:///path/to/file").unwrap()
            })
        );

        // Note: On non-Windows systems, the host is typically ignored, so this should work
        assert_eq!(
            get_extension(&Url::parse("file://localhost/path/to/file.rs").unwrap())
                .unwrap()
                .as_str(),
            "rs"
        );
    }
}
