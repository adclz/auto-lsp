use std::{io::Read, path::Path, sync::Arc};

use auto_lsp_core::{
    document::Document,
    errors::{DataBaseError, ExtensionError, FileSystemError, RuntimeError, TreeSitterError},
    parsers::Parsers,
};
use auto_lsp_server::Session;
use bon::bon;
use lsp_types::{DidChangeTextDocumentParams, PositionEncodingKind, Url};
use salsa::Setter;
use texter::core::text::Text;
use tree_sitter::Tree;

use crate::{db::BaseDatabase, server::workspace_init::get_extension};

/// A salsa input that represents a file in the database.
///
/// Multiple builders are provided to create a file:
///  - `from_fs`: Creates a file by reading the file system.
///  - `from_text_doc`: Creates a file from a text document event.
///  - `from_string`: Creates a file from a string.
///
/// `from_fs` is suitable for file watchers and workspace loading (e.g. [`lsp_types::notification::DidChangeWatchedFiles`]).
///
/// `from_text_doc` is suitable for text document events (e.g. [`lsp_types::notification::DidChangeTextDocument`]).
///
/// `from_string` is suitable for tests and in-memory files.
#[salsa::input]
pub struct File {
    #[return_ref]
    pub url: Url,
    pub parsers: &'static Parsers,
    #[return_ref]
    pub document: Arc<Document>,
    // Document version, None if created via the file system.
    pub version: Option<i32>,
}

#[bon]
impl File {
    /// Creates a new file by reading the file system.
    #[builder]
    pub fn from_fs(session: &Session<impl BaseDatabase>, url: &Url) -> Result<Self, RuntimeError> {
        let file_path = url.to_file_path().map_err(|_| {
            RuntimeError::from(FileSystemError::FileUrlToFilePath { path: url.clone() })
        })?;

        let (parsers, _, text) =
            Self::read_file(session, &file_path).map_err(RuntimeError::from)?;

        let tree = Self::ts_parse(parsers, &text, url)?;
        let document = Document::new(text, tree, Some(&session.encoding));

        Ok(File::new(
            &session.db,
            url.clone(),
            parsers,
            Arc::new(document),
            None,
        ))
    }

    /// Creates a new file from a text document event.
    #[builder]
    pub fn from_text_doc(
        session: &Session<impl salsa::Database>,
        doc: &lsp_types::TextDocumentItem,
    ) -> Result<Self, RuntimeError> {
        let url = &doc.uri;
        let text = Text::new(doc.text.clone());

        let parsers = Self::get_ast_parser(session, &doc.language_id)?;
        let tree = Self::ts_parse(parsers, &text, url)?;
        let document = Document::new(text, tree, Some(&session.encoding));

        Ok(File::new(
            &session.db,
            url.clone(),
            parsers,
            Arc::new(document),
            Some(doc.version),
        ))
    }

    /// Creates a new file from a string.
    #[builder]
    pub fn from_string(
        db: &impl salsa::Database,
        source: String,
        url: &Url,
        parsers: &'static Parsers,
        encoding: Option<&PositionEncodingKind>,
    ) -> Result<Self, RuntimeError> {
        let text = Text::new(source);

        let tree = Self::ts_parse(parsers, &text, url)?;
        let document = Document::new(text, tree, encoding);

        Ok(File::new(
            db,
            url.clone(),
            parsers,
            Arc::new(document),
            None,
        ))
    }

    /// Updates the file from a [`DidChangeTextDocumentParams`] event.
    pub fn update_edit(
        &self,
        db: &mut impl salsa::Database,
        event: &DidChangeTextDocumentParams,
    ) -> Result<(), DataBaseError> {
        let changes = &event.content_changes;
        let mut doc = (*self.document(db)).clone();

        doc.update(&mut self.parsers(db).parser.write(), changes)
            .map_err(|e| DataBaseError::from((&self.url(db), e)))?;

        self.set_document(db).to(Arc::new(doc));
        self.set_version(db).to(Some(event.text_document.version));
        Ok(())
    }

    /// Updates the file from the file system.
    pub fn update_full_fs(
        &self,
        session: &mut Session<impl BaseDatabase>,
    ) -> Result<(), RuntimeError> {
        let url = self.url(&session.db).clone();

        let file_path = url.to_file_path().map_err(|_| {
            RuntimeError::from(FileSystemError::FileUrlToFilePath { path: url.clone() })
        })?;

        let (parsers, _, text) =
            Self::read_file(&session, &file_path).map_err(RuntimeError::from)?;

        let tree = Self::ts_parse(parsers, &text, &url)?;
        let document = Document::new(text, tree, Some(&session.encoding));

        let db = &mut session.db;
        self.set_document(db).to(Arc::new(document));
        Ok(())
    }

    /// Updates the file from a full text document.
    pub fn update_full_text_doc(
        &self,
        session: &mut Session<impl salsa::Database>,
        event: &lsp_types::TextDocumentItem,
    ) -> Result<(), DataBaseError> {
        let db = &mut session.db;
        let text = Text::new(event.text.clone());
        let tree = Self::ts_parse(self.parsers(db), &text, &self.url(db))?;
        let document = Document::new(text, tree, Some(&PositionEncodingKind::UTF16));

        self.set_document(db).to(Arc::new(document));
        self.set_version(db).to(Some(event.version));
        Ok(())
    }

    /// Resets the file to an empty document.
    pub fn reset(&self, db: &mut impl BaseDatabase) -> Result<(), DataBaseError> {
        let text = Text::new("".into());
        let tree = Self::ts_parse(self.parsers(db), &text, &self.url(db))?;
        let document = Document::new(text, tree, Some(&PositionEncodingKind::UTF16));

        self.set_document(db).to(Arc::new(document));
        self.set_version(db).to(None);
        Ok(())
    }

    pub fn read_file(
        session: &Session<impl BaseDatabase>,
        file: &Path,
    ) -> Result<(&'static Parsers, Url, Text), RuntimeError> {
        let url = Url::from_file_path(file).map_err(|_| FileSystemError::FilePathToUrl {
            path: file.to_path_buf(),
        })?;

        let mut open_file = std::fs::File::open(file).map_err(|e| FileSystemError::FileOpen {
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

        let parsers = Self::get_ast_parser(session, &extension)?;
        let text = Text::new(buffer);

        Ok((parsers, url, text))
    }

    /// Utility function to parse a tree sitter tree.
    fn ts_parse(parsers: &'static Parsers, text: &Text, url: &Url) -> Result<Tree, DataBaseError> {
        parsers
            .parser
            .write()
            .parse(text.text.as_bytes(), None)
            .ok_or_else(|| DataBaseError::from((url, TreeSitterError::TreeSitterParser)))
    }

    /// Utility function to get the AST parser for an extension.
    fn get_ast_parser(
        session: &Session<impl salsa::Database>,
        extension: &str,
    ) -> Result<&'static Parsers, RuntimeError> {
        // Check if the extension is registered
        let extension = match session.extensions.get(extension) {
            Some(extension) => extension,
            None => {
                if session.extensions.values().any(|x| x == extension) {
                    extension
                } else {
                    return Err(ExtensionError::UnknownExtension {
                        extension: extension.to_string(),
                        available: session.extensions.clone(),
                    }
                    .into());
                }
            }
        };

        // Check if the parser for this extension is available
        session.init_options.parsers.get(extension).ok_or_else(|| {
            RuntimeError::from(ExtensionError::UnknownParser {
                extension: extension.to_string(),
                available: session.init_options.parsers.keys().cloned().collect(),
            })
        })
    }
}
