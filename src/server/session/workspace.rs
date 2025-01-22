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
    /// Initializes the workspace by loading files and associating them with parsers.
    pub(crate) fn init_workspaces(&mut self, params: InitializeParams) -> anyhow::Result<()> {
        let options = InitializationOptions::deserialize(
            params
                .initialization_options
                .expect("Missing initialization options from client"),
        )?;

        // Validate that the parsers provided by the client exist
        for (file_extension, parser) in &options.perFileParser {
            if let false = self.init_options.parsers.contains_key(parser.as_str()) {
                return Err(anyhow::format_err!(
                    "Error: Parser {} not found for file extension {}",
                    parser,
                    file_extension
                ));
            }
        }

        self.extensions = options.perFileParser;

        // Traverse workspace folders and add files to the session
        collect_workspace_files(&self.extensions, &params.workspace_folders)
            .into_iter()
            .try_for_each(|file| {
                let mut open_file = File::open(&file.to_file_path().unwrap())?;
                let mut buffer = String::new();
                open_file.read_to_string(&mut buffer)?;

                let extension = get_extension(&file)?;
                self.add_document(&file, &extension, &buffer)
            })?;

        Ok(())
    }
}

/// Get the extension of a file from a [`Url`] path
pub(crate) fn get_extension(path: &Url) -> anyhow::Result<String> {
    path.to_file_path()
        .map_err(|_| anyhow::anyhow!("Failed to read file {}", path.to_string()))?
        .extension()
        .map_or_else(
            || {
                Err(anyhow::anyhow!(format!(
                    "Invalid extension for file {}",
                    path.to_string()
                )))
            },
            |ext| Ok(ext.to_string_lossy().to_string()),
        )
}

/// Collects all files in the workspace folders that match the specified extensions.
///
/// A vector of [`Url`]s representing the valid files in the workspace.
fn collect_workspace_files(
    extensions: &HashMap<String, String>,
    workspace_folders: &Option<Vec<WorkspaceFolder>>,
) -> Vec<Url> {
    let mut roots = Vec::new();
    if let Some(folders) = workspace_folders {
        folders.into_iter().for_each(|folder| {
            WalkDir::new(folder.uri.path())
                .into_iter()
                .filter_map(Result::ok)
                .filter(|entry| {
                    entry.file_type().is_file()
                        && entry.path().extension().map_or(false, |ext| {
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
    #[test]
    fn test_get_extension() {
        use super::get_extension;
        use lsp_types::Url;

        // rs
        assert_eq!(
            get_extension(&Url::parse("file:///path/to/file.rs").unwrap())
                .unwrap()
                .as_str(),
            "rs"
        );

        // multiple dots
        assert_eq!(
            get_extension(&Url::parse("file:///path/to/file.with.multiple.dots").unwrap())
                .unwrap()
                .as_str(),
            "dots"
        );

        // empty
        assert_eq!(
            get_extension(&Url::parse("file:///path/to/file").unwrap())
                .unwrap_err()
                .to_string()
                .as_str(),
            "Invalid extension for file file:///path/to/file"
        );
    }
}
