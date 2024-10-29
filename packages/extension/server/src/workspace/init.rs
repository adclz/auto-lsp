use std::io::Read;
use std::{collections::HashMap, fs::File};

use lsp_textdocument::FullTextDocument;
use lsp_types::{CreateFilesParams, Url, WorkspaceFolder};
use walkdir::WalkDir;

// URIs are broken in lsp_types 0.96.0
// https://github.com/gluon-lang/lsp-types/issues/284

pub fn get_workspace_folders(
    workspace_folders: &Option<Vec<WorkspaceFolder>>,
) -> HashMap<Url, FullTextDocument> {
    let mut roots = HashMap::new();
    if let Some(folders) = workspace_folders {
        folders.into_iter().for_each(|folder| {
            WalkDir::new(folder.uri.path())
                .into_iter()
                .filter_map(Result::ok)
                .filter(|entry| entry.file_type().is_file())
                .for_each(|file| {
                    let mut open_file = File::open(&file.path()).unwrap();
                    let mut buffer = String::new();
                    open_file.read_to_string(&mut buffer).unwrap();

                    eprintln!("file {:?}", file.path());

                    roots.insert(
                        Url::from_file_path(file.path()).unwrap(),
                        FullTextDocument::new(
                            file.path()
                                .extension()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string(),
                            0,
                            buffer.clone(),
                        ),
                    );
                });
        });
    }
    roots
}

pub fn add_files(params: &CreateFilesParams) -> HashMap<String, FullTextDocument> {
    let mut roots = HashMap::new();
    params.files.iter().for_each(|file| {
        let mut open_file = File::open(&file.uri).unwrap();
        let mut buffer = String::new();
        open_file.read_to_string(&mut buffer).unwrap();

        roots.insert(
            file.uri.to_owned(),
            FullTextDocument::new(
                file.uri.split(".").last().unwrap().to_string(),
                0,
                buffer.clone(),
            ),
        );
    });
    roots
}
