use std::{collections::HashMap, fs::File, io::Read};

use lsp_types::{InitializeParams, Url, WorkspaceFolder};
use serde::Deserialize;
use walkdir::WalkDir;

use super::Session;

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
    /// Initializes the root by loading files and associating them with parsers.
    pub(crate) fn init_roots(&mut self, params: InitializeParams) -> anyhow::Result<()> {
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

        // Traverse root folders and add files to the session
        collect_root_files(&self.extensions, &params.workspace_folders)
            .into_iter()
            .try_for_each(|file| {
                let mut open_file = File::open(file.to_file_path().unwrap())?;
                let mut buffer = String::new();
                open_file.read_to_string(&mut buffer)?;

                let extension = get_extension(&file)?;
                self.add_document(&file, &extension, &buffer)
            })?;

        Ok(())
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

/// Collects all files in the root folders that match the specified extensions.
///
/// A vector of [`Url`]s representing the valid files in the root.
fn collect_root_files(
    extensions: &HashMap<String, String>,
    root_folders: &Option<Vec<WorkspaceFolder>>,
) -> Vec<Url> {
    let mut roots = Vec::new();
    if let Some(folders) = root_folders {
        folders.iter().for_each(|folder| {
            WalkDir::new(folder.uri.path())
                .into_iter()
                .filter_map(Result::ok)
                .filter(|entry| {
                    entry.file_type().is_file()
                        && entry.path().extension().is_some_and(|ext| {
                            extensions.contains_key(ext.to_string_lossy().as_ref())
                        })
                })
                .for_each(|file| {
                    roots.push(Url::from_file_path(file.path()).unwrap());
                });
        });
    }
    roots
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
