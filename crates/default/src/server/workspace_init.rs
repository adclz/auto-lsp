use crate::db::FileManager;
use crate::db::file::File;
use auto_lsp_core::{errors::RuntimeError, parsers::Parser};
use auto_lsp_server::Session;
use lsp_types::{InitializeParams, Url};
use walkdir::{DirEntry, WalkDir};

use crate::db::BaseDatabase;

pub trait WorkspaceInit {
    fn init_workspace<F: Fn(&DirEntry) -> Option<&'static Parser>>(
        &mut self,
        params: InitializeParams,
        add_file: F,
    ) -> Vec<RuntimeError>;
}

impl<Db: BaseDatabase> WorkspaceInit for Session<Db> {
    /// Initializes the workspace by loading files and associating them with parsers.
    fn init_workspace<F: Fn(&DirEntry) -> Option<&'static Parser>>(
        &mut self,
        params: InitializeParams,
        add_file: F,
    ) -> Vec<RuntimeError> {
        let mut errors: Vec<RuntimeError> = vec![];

        if let Some(folders) = params.workspace_folders {
            folders.into_iter().for_each(|folder| {
                WalkDir::new(folder.uri.path())
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter_map(|entry| add_file(&entry).map(|parser| (entry, parser)))
                    .for_each(|(entry, parser)| {
                        let url = Url::from_file_path(entry.into_path()).unwrap();
                        let f = match File::from_fs()
                            .session(self)
                            .url(&url)
                            .parser(parser)
                            .call()
                        {
                            Ok(f) => f,
                            Err(e) => {
                                errors.push(e);
                                return;
                            }
                        };
                        if let Err(e) = self.db.add_file(f) {
                            errors.push(e.into());
                        }
                    });
            });
        }

        errors
    }
}
