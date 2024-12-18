use std::{collections::HashMap, fs::File, io::Read};

use lsp_types::{InitializeParams, Url, WorkspaceFolder};
use serde::Deserialize;
use walkdir::WalkDir;

use crate::CST_PARSERS;

use super::Session;

#[allow(non_snake_case, reason = "JSON")]
#[derive(Debug, Deserialize)]
struct InitializationOptions {
    perFileParser: HashMap<String, String>,
}

pub fn get_extension(url: &Url) -> Option<String> {
    let path = url.to_file_path().unwrap();
    let extension = path.extension().unwrap();
    extension.to_str().map(|s| s.to_string())
}

impl Session {
    pub fn init(&mut self, params: serde_json::Value) -> anyhow::Result<()> {
        let params: InitializeParams = serde_json::from_value(params).unwrap();
        let options = InitializationOptions::deserialize(
            params
                .initialization_options
                .expect("Missing initialization options from client"),
        )
        .unwrap();

        // Check if extensions provided by clients are valid

        for (file_extension, parser) in &options.perFileParser {
            if let false = CST_PARSERS.contains_key(parser.as_str()) {
                return Err(anyhow::format_err!(
                    "Error: Parser {} not found for file extension {}",
                    parser,
                    file_extension
                ));
            }
        }

        self.extensions = options.perFileParser;

        // Add workspace folders

        convert_workspace_folders_to_urls(&self.extensions, &params.workspace_folders)
            .into_iter()
            .for_each(|file| {
                let mut open_file = File::open(&file.to_file_path().unwrap()).unwrap();
                let mut buffer = String::new();
                open_file.read_to_string(&mut buffer).unwrap();

                let extension = get_extension(&file).unwrap();
                self.add_document(&file, &extension, &buffer).unwrap();
            });

        Ok(())
    }
}

fn convert_workspace_folders_to_urls(
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

#[macro_export]
macro_rules! create_parser {
    ($parser: ident) => {{
        use std::sync::RwLock;
        use $parser::{COMMENTS_QUERY, FOLD_QUERY, HIGHLIGHTS_QUERY, LANGUAGE, OUTLINE_QUERY};
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&LANGUAGE.into())
            .expect(&format!("Error loading {} parser", stringify!($parser)));
        let lang = parser.language().unwrap();
        crate::session::cst_parser::CstParser {
            parser: RwLock::new(parser),
            language: lang.clone(),
            queries: crate::session::cst_parser::Queries {
                comments: tree_sitter::Query::new(&lang, COMMENTS_QUERY).unwrap(),
                fold: tree_sitter::Query::new(&lang, FOLD_QUERY).unwrap(),
                highlights: tree_sitter::Query::new(&lang, HIGHLIGHTS_QUERY).unwrap(),
                outline: tree_sitter::Query::new(&lang, OUTLINE_QUERY).unwrap(),
            },
        }
    }};
}
