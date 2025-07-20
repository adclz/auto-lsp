use std::{
    io::{BufReader, Read},
    path::Path,
    sync::Arc,
};

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

use crate::server::workspace_init::get_extension;

/// A salsa input that represents a file in the database.
///
/// # Creating a File
///
/// Multiple builders are provided to create a file:
///  - `from_fs`: Creates a file by reading the file system.
///  - `from_text_doc`: Creates a file from a text document event.
///  - `from_string`: Creates a file from a string.
///
/// `from_fs` is suitable for file watchers and workspace loading (e.g. [`lsp_types::notification::DidChangeWatchedFiles`]).
///
/// `from_text_doc` is suitable for open text document events (e.g. [`lsp_types::notification::DidOpenTextDocument`]).
///
/// `from_string` is suitable for tests and in-memory files.
///
/// # Updating a File
///
/// Files can be updated using:
///  - `update_edit`: Updates the file from a [`DidChangeTextDocumentParams`] event.
///  - `update_full_fs`: Updates the file by reading the file system.
///         - This will aso check if the file has changed and only update if necessary.
///  - `update_full_text_doc`: Updates the file from [`TextDocumentItem`].
///  - `reset`: Resets the file to an empty document.
///
/// # Deleting a File
///
/// `reset` can be used to delete the content of a file.
///
#[salsa::input(debug)]
pub struct File {
    #[returns(ref)]
    pub url: Url,

    pub parsers: &'static Parsers,

    #[returns(ref)]
    pub document: Arc<Document>,

    // Document version, None if created via the file system.
    pub version: Option<i32>,
}

#[bon]
impl File {
    /// Creates a new file by reading the file system.
    #[builder]
    pub fn from_fs(
        session: &Session<impl salsa::Database>,
        url: &Url,
    ) -> Result<Self, RuntimeError> {
        let file_path = url.to_file_path().map_err(|_| {
            RuntimeError::from(FileSystemError::FileUrlToFilePath { path: url.clone() })
        })?;

        let (_file, buffer) = Self::read_file_content(&file_path).map_err(RuntimeError::from)?;

        let extension = get_extension(&url)?;
        let parsers = Self::get_ast_parser(session, &extension)?;
        let tree = Self::ts_parse(parsers, &buffer, url)?;
        let document = Document::new(buffer, tree, Some(&session.encoding));

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

        let parsers = Self::get_ast_parser(session, &doc.language_id)?;
        let tree = Self::ts_parse(parsers, &doc.text, url)?;
        let document = Document::new(doc.text.clone(), tree, Some(&session.encoding));

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
        let tree = Self::ts_parse(parsers, &source, url)?;
        let document = Document::new(source, tree, encoding);

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
        let mut doc = (**self.document(db)).clone();

        doc.update(&mut self.parsers(db).parser.write(), changes)
            .map_err(|e| DataBaseError::from((self.url(db), e)))?;

        self.set_document(db).to(Arc::new(doc));
        self.set_version(db).to(Some(event.text_document.version));
        Ok(())
    }

    /// Updates the file from the file system.
    pub fn update_full_fs(
        &self,
        session: &mut Session<impl salsa::Database>,
    ) -> Result<(), RuntimeError> {
        let url = self.url(&session.db).clone();

        let file_path = url.to_file_path().map_err(|_| {
            RuntimeError::from(FileSystemError::FileUrlToFilePath { path: url.clone() })
        })?;

        let (_file, buffer) = Self::read_file_content(&file_path).map_err(RuntimeError::from)?;

        if self.fail_fast_check(&session.db, &buffer) {
            log::info!("File unchanged: {}", url);
            return Ok(());
        }

        let extension = get_extension(&url)?;

        let parsers = Self::get_ast_parser(session, &extension)?;
        let tree = Self::ts_parse(parsers, &buffer, &url)?;
        let document = Document::new(buffer, tree, Some(&session.encoding));

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
        let tree = Self::ts_parse(self.parsers(db), &event.text, &self.url(db))?;
        let document = Document::new(event.text.clone(), tree, Some(&session.encoding));

        self.set_document(db).to(Arc::new(document));
        self.set_version(db).to(Some(event.version));
        Ok(())
    }

    /// Resets the file to an empty document.
    pub fn reset(&self, db: &mut impl salsa::Database) -> Result<(), DataBaseError> {
        let tree = Self::ts_parse(self.parsers(db), &"", &self.url(db))?;
        let document = Document {
            texter: Text::new("".into()),
            tree,
            encoding: self.document(db).encoding,
        };

        self.set_document(db).to(Arc::new(document));
        self.set_version(db).to(None);
        Ok(())
    }

    pub fn read_file_content(file: &Path) -> Result<(std::fs::File, String), RuntimeError> {
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

        Ok((open_file, buffer))
    }

    /// Check if this file is equal to a string and stops as soon as it finds a difference.
    pub fn fail_fast_check(&self, db: &impl salsa::Database, file2: &str) -> bool {
        let doc = self.document(db);

        // simple length check
        if doc.as_str().len() != file2.len() {
            return false;
        }

        let mut reader1 = BufReader::new(doc.as_bytes());
        let mut reader2 = BufReader::new(file2.as_bytes());
        let mut buf1 = [0; 10000];
        let mut buf2 = [0; 10000];
        loop {
            if let Result::Ok(n1) = reader1.read(&mut buf1) {
                if n1 > 0 {
                    if let Result::Ok(n2) = reader2.read(&mut buf2) {
                        if n1 == n2 {
                            if buf1 == buf2 {
                                continue;
                            }
                        }
                        return false;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        true
    }

    /// Utility function to parse a tree sitter tree.
    fn ts_parse(parsers: &'static Parsers, source: &str, url: &Url) -> Result<Tree, DataBaseError> {
        parsers
            .parser
            .write()
            .parse(source.as_bytes(), None)
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
