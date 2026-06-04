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
