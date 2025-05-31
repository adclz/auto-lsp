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

/// All structs and traits present in this module serve a minimal database implementation with basic file management.
///
/// Depending on your needs, you might want to create your own database and inputs.
pub mod lexer;
pub mod tracked;

use auto_lsp_core::document::Document;
use auto_lsp_core::errors::{DataBaseError, TreeSitterError};
use auto_lsp_core::parsers::Parsers;
use dashmap::{DashMap, Entry};
use lsp_types::Url;
use salsa::Setter;
use salsa::{Database, Storage};
use std::{hash::Hash, sync::Arc};
use texter::core::text::Text;

/// Input that represents a file in the database.
///
/// An [`lsp_types::Url`] is used as the unique identifier of the file.
#[salsa::input]
pub struct File {
    #[id]
    pub url: Url,
    pub parsers: &'static Parsers,
    #[return_ref]
    pub document: Arc<Document>,
}

/// Base database that stores files.
///
/// Files are stored in a [`DashMap`] for concurrent access.
///
/// This also allows to lazily compute the AST of a file when it is first queried.
///
/// Logs are also stored when running in debug mode.
#[salsa::db]
#[derive(Default, Clone)]
pub struct BaseDb {
    storage: Storage<Self>,
    pub(crate) files: DashMap<Url, File>,
}

impl BaseDb {
    /// Create a new database with a logger.
    pub fn with_logger(
        event_callback: Option<Box<dyn Fn(salsa::Event) + Send + Sync + 'static>>,
    ) -> Self {
        Self {
            storage: Storage::new(event_callback),
            files: DashMap::default(),
        }
    }
}

/// Base trait for a database that stores files.
///
/// This trait is implemented for [`BaseDb`] and can be implemented for your own database.
#[salsa::db]
pub trait BaseDatabase: Database {
    fn get_files(&self) -> &DashMap<Url, File>;

    fn get_file(&self, url: &Url) -> Option<File> {
        self.get_files().get(url).map(|file| *file)
    }
}

/// Implementation of [`salsa::Database`] for [`BaseDb`].
#[salsa::db]
impl salsa::Database for BaseDb {}
impl std::panic::RefUnwindSafe for BaseDb {}

/// Implementation of [`BaseDatabase`] for [`BaseDb`].
#[salsa::db]
impl BaseDatabase for BaseDb {
    fn get_files(&self) -> &DashMap<Url, File> {
        &self.files
    }
}

/// Trait for managing files in the database.
///
/// This trait is implemented for any database that implements [`BaseDatabase`].
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

        let mut doc = (*file.document(self)).clone();

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
