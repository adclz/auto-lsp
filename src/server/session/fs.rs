use super::{Session, WORKSPACE};
use auto_lsp_core::document::Document;
use auto_lsp_core::root::Root;
use lsp_types::{InitializeParams, Url};
use serde::Deserialize;
use std::path::PathBuf;
use std::{collections::HashMap, fs::File, io::Read};
use walkdir::WalkDir;

#[allow(non_snake_case, reason = "JSON")]
#[derive(Debug, Deserialize)]
struct InitializationOptions {
    /// Maps file extensions to parser names.
    ///
    /// Example: { "rs": "rust", "py": "python" }
    /// This option is provided by the client to define how different file types should be parsed.
    perFileParser: HashMap<String, String>,
}

impl Session {
    /// Initializes the workspace by loading files and associating them with parsers.
    pub(crate) fn init_workspace(&mut self, params: InitializeParams) -> anyhow::Result<()> {
        let options = InitializationOptions::deserialize(
            params
                .initialization_options
                .expect("Missing initialization options from client"),
        )?;

        // Validate that the parsers provided by the client exist
        for (file_extension, parser) in &options.perFileParser {
            if !self.init_options.parsers.contains_key(parser.as_str()) {
                return Err(anyhow::format_err!(
                    "Error: Parser {} not found for file extension {}",
                    parser,
                    file_extension
                ));
            }
        }

        self.extensions = options.perFileParser;

        let mut errors: Vec<Result<(), anyhow::Error>> = vec![];

        if let Some(folders) = params.workspace_folders {
            let mut workspace = WORKSPACE.lock();
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

            #[cfg(not(feature = "rayon"))]
            errors.extend(
                files
                    .into_iter()
                    .map(|file| match self.file_to_root(&file.into_path()) {
                        Ok((url, root, document)) => {
                            workspace.roots.insert(url, (root, document));
                            Ok(())
                        }
                        Err(err) => Err(err),
                    })
                    .collect::<Vec<Result<(), anyhow::Error>>>(),
            );

            #[cfg(feature = "rayon")]
            {
                use rayon::prelude::*;
                errors.extend(rayon_par_bridge::par_bridge(
                    16,
                    files.into_par_iter(),
                    |file_iter| {
                        file_iter
                            .map(|file| match self.file_to_root(&file.into_path()) {
                                Ok((url, root, document)) => {
                                    workspace.roots.insert(url, (root, document));
                                    Ok(())
                                }
                                Err(err) => Err(err),
                            })
                            .collect::<Vec<Result<(), anyhow::Error>>>()
                    },
                ));
            }

            workspace.resolve_references();
            workspace.resolve_checks();
        }

        Ok(())
    }

    pub(crate) fn file_to_root(&self, file: &PathBuf) -> anyhow::Result<(Url, Root, Document)> {
        let url = Url::from_file_path(file)
            .map_err(|_| anyhow::anyhow!("Failed to read file {}", file.display()))?;

        let mut open_file = File::open(url.to_file_path().unwrap())?;
        let mut buffer = String::new();
        open_file.read_to_string(&mut buffer)?;

        let extension = get_extension(&url)?;

        let text = (self.text_fn)(buffer.to_string());
        let extension = match self.extensions.get(&extension) {
            Some(extension) => extension,
            None => {
                return Err(anyhow::format_err!(
                    "Extension {} is not registered",
                    extension
                ))
            }
        };

        let parsers = self
            .init_options
            .parsers
            .get(extension.as_str())
            .ok_or(anyhow::format_err!("No parser available for {}", extension))?;

        let result = Root::from_texter(parsers, url.clone(), text)?;
        Ok((url, result.0, result.1))
    }
}

/// Get the extension of a file from a [`Url`] path
#[cfg(windows)]
pub(crate) fn get_extension(path: &Url) -> anyhow::Result<String> {
    // Ensure the host is either empty or "localhost" on Windows
    if let Some(host) = path.host_str() {
        if !host.is_empty() && host != "localhost" {
            return Err(anyhow::anyhow!(
                "Invalid host '{}' for file URL {}",
                host,
                path
            ));
        }
    }

    path.to_file_path()
        .map_err(|_| anyhow::anyhow!("Failed to read file URL {}", path))?
        .extension()
        .map_or_else(
            || {
                Err(anyhow::anyhow!(format!(
                    "Invalid extension for file {}",
                    path
                )))
            },
            |ext| Ok(ext.to_string_lossy().to_string()),
        )
}

#[cfg(not(windows))]
pub(crate) fn get_extension(path: &Url) -> anyhow::Result<String> {
    path.to_file_path()
        .map_err(|_| anyhow::anyhow!("Failed to read file URL {}", path))?
        .extension()
        .map_or_else(
            || {
                Err(anyhow::anyhow!(format!(
                    "Invalid extension for file {}",
                    path
                )))
            },
            |ext| Ok(ext.to_string_lossy().to_string()),
        )
}

#[cfg(test)]
mod tests {
    use super::get_extension;
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
            get_extension(&Url::parse("file:///C:/path/to/file").unwrap())
                .unwrap_err()
                .to_string()
                .as_str(),
            "Invalid extension for file file:///C:/path/to/file"
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
            get_extension(&Url::parse("file:///path/to/file").unwrap())
                .unwrap_err()
                .to_string()
                .as_str(),
            "Invalid extension for file file:///path/to/file"
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
