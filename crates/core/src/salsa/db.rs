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

use crate::document::Document;
use crate::errors::{DataBaseError, TreeSitterError};
use crate::parsers::Parsers;
use dashmap::{DashMap, Entry};
use lsp_types::Url;
use salsa::Setter;
use salsa::{Database, Storage};
use std::{hash::Hash, sync::Arc};
use texter::core::text::Text;

#[salsa::input]
pub struct File {
    #[id]
    pub url: Url,
    pub parsers: &'static Parsers,
    #[return_ref]
    pub document: Arc<Document>,
}

#[salsa::db]
#[derive(Default, Clone)]
pub struct BaseDb {
    storage: Storage<Self>,
    pub(crate) files: DashMap<Url, File>,
    #[cfg(debug_assertions)]
    logs: Arc<parking_lot::Mutex<Vec<String>>>,
}

#[salsa::db]
pub trait BaseDatabase: Database {
    fn get_files(&self) -> &DashMap<Url, File>;

    fn get_file(&self, url: &Url) -> Option<File> {
        self.get_files().get(url).map(|file| *file)
    }

    #[cfg(debug_assertions)]
    fn take_logs(&self) -> Vec<String>;
}

#[salsa::db]
impl salsa::Database for BaseDb {
    fn salsa_event(&self, _event: &dyn Fn() -> salsa::Event) {
        #[cfg(debug_assertions)]
        {
            let event = _event();
            if let salsa::EventKind::WillExecute { .. } = event.kind {
                self.logs.lock().push(format!("{event:?}"));
            }
        }
    }
}

impl std::panic::RefUnwindSafe for BaseDb {}

#[salsa::db]
impl BaseDatabase for BaseDb {
    fn get_files(&self) -> &DashMap<Url, File> {
        &self.files
    }

    #[cfg(debug_assertions)]
    fn take_logs(&self) -> Vec<String> {
        std::mem::take(&mut self.logs.lock())
    }
}

pub trait FileManager: BaseDatabase + salsa::Database {
    fn add_file_from_texter(
        &mut self,
        parsers: &'static Parsers,
        url: &Url,
        texter: Text,
    ) -> Result<(), DataBaseError> {
        let tree = parsers
            .parser
            .write()
            .parse(texter.text.as_bytes(), None)
            .ok_or_else(|| DataBaseError::from((url, TreeSitterError::TreeSitterParser)))?;

        let document = Document { texter, tree };
        let file = File::new(self, url.clone(), parsers, Arc::new(document));

        match self.get_files().entry(url.clone()) {
            Entry::Occupied(_) => Err(DataBaseError::FileAlreadyExists { uri: url.clone() }),
            Entry::Vacant(entry) => {
                entry.insert(file);
                Ok(())
            }
        }
    }

    fn update(
        &mut self,
        url: &Url,
        changes: &[lsp_types::TextDocumentContentChangeEvent],
    ) -> Result<(), DataBaseError> {
        let file = *self
            .get_files()
            .get_mut(url)
            .ok_or_else(|| DataBaseError::FileNotFound { uri: url.clone() })?;

        let mut doc = (**file.document(self)).clone();

        doc.update(&mut file.parsers(self).parser.write(), changes)
            .map_err(|e| DataBaseError::from((url, e)))?;

        file.set_document(self).to(Arc::new(doc));
        Ok(())
    }

    fn remove_file(&mut self, url: &Url) -> Result<(), DataBaseError> {
        match self.get_files().remove(url) {
            None => Err(DataBaseError::FileNotFound { uri: url.clone() }),
            Some(_) => Ok(()),
        }
    }
}

impl<T> FileManager for T where T: BaseDatabase + salsa::Database {}
