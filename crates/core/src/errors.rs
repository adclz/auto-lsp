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

use std::{collections::HashMap, path::PathBuf, str::Utf8Error};

use ariadne::{ColorGenerator, Fmt, Label, ReportBuilder, Source};
use lsp_types::Url;
use thiserror::Error;

use crate::document::Document;

/// Error type coming from either tree-sitter or ast parsing.
///
/// This error is only produced by auto-lsp.
///
/// [`ParseError`] can be converted to [`lsp_types::Diagnostic`] to be sent to the client.
///
/// An ariadne report can be generated from [`ParseError`] using the `to_label` method.
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    #[error("{error:?}")]
    LexerError {
        range: lsp_types::Range,
        #[source]
        error: LexerError,
    },
    #[error("{error:?}")]
    AstError {
        range: lsp_types::Range,
        #[source]
        error: AstError,
    },
}

impl From<ParseError> for lsp_types::Diagnostic {
    fn from(error: ParseError) -> Self {
        (&error).into()
    }
}

impl From<&ParseError> for lsp_types::Diagnostic {
    fn from(error: &ParseError) -> Self {
        let message = error.to_string();
        let range = match error {
            ParseError::LexerError { range, .. } => *range,
            ParseError::AstError { range, .. } => *range,
        };
        lsp_types::Diagnostic {
            range,
            severity: Some(lsp_types::DiagnosticSeverity::ERROR),
            message,
            code: Some(lsp_types::NumberOrString::String("AUTO_LSP".into())),
            ..Default::default()
        }
    }
}

impl ParseError {
    /// Creates a label for the error using ariadne.
    pub fn to_label(
        &self,
        source: &Source<&str>,
        colors: &mut ColorGenerator,
        report: &mut ReportBuilder<'_, std::ops::Range<usize>>,
    ) {
        let range = match self {
            ParseError::LexerError { range, .. } => range,
            ParseError::AstError { range, .. } => range,
        };
        let start_line = source.line(range.start.line as usize).unwrap().offset();
        let end_line = source.line(range.end.line as usize).unwrap().offset();
        let start = start_line + range.start.character as usize;
        let end = end_line + range.end.character as usize;
        let curr_color = colors.next();

        report.add_label(
            Label::new(start..end)
                .with_message(format!("{}", self.to_string().fg(curr_color)))
                .with_color(curr_color),
        );
    }
}

/// Error type for AST parsing.
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum AstError {
    #[error("No root node found with {query:?}")]
    NoRootNode {
        range: std::ops::Range<usize>,
        query: &'static [&'static str],
    },
    #[error("Invalid {field_name:?} for {parent_name:?}, received query: {query:?}")]
    InvalidSymbol {
        range: std::ops::Range<usize>,
        field_name: String,
        parent_name: String,
        query: &'static str,
    },
    #[error("Unknown symbol {symbol:?} in {parent_name:?}")]
    UnknownSymbol {
        range: std::ops::Range<usize>,
        symbol: &'static str,
        parent_name: &'static str,
    },
    // Happens when a #[seq] field tis empty
    #[error("Missing symbol {symbol:?} in {parent_name:?}")]
    MissingSymbol {
        range: std::ops::Range<usize>,
        symbol: &'static str,
        parent_name: &'static str,
    },
}

impl From<(&Document, AstError)> for ParseError {
    fn from((document, error): (&Document, AstError)) -> Self {
        let range = match &error {
            AstError::NoRootNode { range, .. } => range,
            AstError::UnknownSymbol { range, .. } => range,
            AstError::InvalidSymbol { range, .. } => range,
            AstError::MissingSymbol { range, .. } => range,
        };
        let range = document.range_at(range.clone()).unwrap_or_default();
        Self::AstError { range, error }
    }
}

/// Error type for tree-sitter.
///
/// Can either be a syntax error or a missing symbol error.
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum LexerError {
    #[error("{error:?}")]
    Missing {
        range: lsp_types::Range,
        error: String,
    },
    #[error("{error:?}")]
    Syntax {
        range: lsp_types::Range,
        error: String,
    },
}

impl From<LexerError> for ParseError {
    fn from(error: LexerError) -> Self {
        let range = match &error {
            LexerError::Missing { range, .. } => *range,
            LexerError::Syntax { range, .. } => *range,
        };
        Self::LexerError { range, error }
    }
}

//// Main accumulator for parse errors
///
/// This is meant to be used in salsa queries to accumulate parse errors.
#[salsa::accumulator]
pub struct ParseErrorAccumulator(pub ParseError);

impl ParseErrorAccumulator {
    pub fn to_label(
        &self,
        source: &Source<&str>,
        colors: &mut ColorGenerator,
        report: &mut ReportBuilder<'_, std::ops::Range<usize>>,
    ) {
        self.0.to_label(source, colors, report);
    }
}

impl From<&ParseErrorAccumulator> for lsp_types::Diagnostic {
    fn from(error: &ParseErrorAccumulator) -> Self {
        Self::from(&error.0)
    }
}

impl From<&ParseError> for ParseErrorAccumulator {
    fn from(diagnostic: &ParseError) -> Self {
        Self(diagnostic.clone())
    }
}

impl From<ParseError> for ParseErrorAccumulator {
    fn from(diagnostic: ParseError) -> Self {
        Self(diagnostic)
    }
}

impl From<&ParseErrorAccumulator> for ParseError {
    fn from(diagnostic: &ParseErrorAccumulator) -> Self {
        diagnostic.0.clone()
    }
}

impl From<LexerError> for ParseErrorAccumulator {
    fn from(error: LexerError) -> Self {
        Self(error.into())
    }
}

impl From<(&Document, AstError)> for ParseErrorAccumulator {
    fn from((document, error): (&Document, AstError)) -> Self {
        Self(ParseError::from((document, error)))
    }
}

/// Error type for position errors.
///
/// Only emitted by methods of the [`Document`] struct.
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum PositionError {
    #[error("Failed to find position of offset {offset:?}, max line length is {length:?}")]
    LineOutOfBound { offset: usize, length: usize },
    #[error("Failed to get position of offset {offset:?}")]
    WrongPosition { offset: usize },
    #[error("Failed to get range of {range:?}: {position_error:?}")]
    WrongRange {
        range: std::ops::Range<usize>,
        #[source]
        position_error: Box<PositionError>,
    },
    #[error("Failed to get text in {range:?}")]
    WrongTextRange { range: std::ops::Range<usize> },
    #[error("Failed to get text in {range:?}: Encountered UTF-8 error {utf8_error:?}")]
    UTF8Error {
        range: std::ops::Range<usize>,
        utf8_error: Utf8Error,
    },
}

/// Error type produced by the runtime - aka the server -.
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum RuntimeError {
    #[error("Document error in {uri:?}: {error:?}")]
    DocumentError {
        uri: Url,
        #[source]
        error: DocumentError,
    },
    #[error("Missing initialization options from client")]
    MissingOptions,
    #[error(transparent)]
    DataBaseError(#[from] DataBaseError),
    #[error(transparent)]
    FileSystemError(#[from] FileSystemError),
    #[error(transparent)]
    ExtensionError(#[from] ExtensionError),
}

impl From<(&Url, DocumentError)> for RuntimeError {
    fn from((uri, error): (&Url, DocumentError)) -> Self {
        RuntimeError::DocumentError {
            uri: uri.clone(),
            error,
        }
    }
}

impl From<(&Url, TreeSitterError)> for RuntimeError {
    fn from((uri, error): (&Url, TreeSitterError)) -> Self {
        RuntimeError::DocumentError {
            uri: uri.clone(),
            error: DocumentError::TreeSitter(error),
        }
    }
}

impl From<(&Url, TexterError)> for RuntimeError {
    fn from((uri, error): (&Url, TexterError)) -> Self {
        RuntimeError::DocumentError {
            uri: uri.clone(),
            error: DocumentError::Texter(error),
        }
    }
}

/// Error types produced by the server when performing file system operations.
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum FileSystemError {
    #[cfg(windows)]
    #[error("Invalid host '{host:?}' for file path: {path:?}")]
    FileUrlHost { host: String, path: Url },
    #[error("Failed to convert url {path:?} to file path")]
    FileUrlToFilePath { path: Url },
    #[error("Failed to convert file path {path:?} to url")]
    FilePathToUrl { path: PathBuf },
    #[error("Failed to get extension of file {path:?}")]
    FileExtension { path: Url },
    #[error("Failed to open file {path:?}: {error:?}")]
    FileOpen { path: Url, error: String },
    #[error("Failed to read file {path:?}: {error:?}")]
    FileRead { path: Url, error: String },
    #[error(transparent)]
    ExtensionError(#[from] ExtensionError),
}

/// Error type for file extensions and parsers associated with them.
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum ExtensionError {
    #[error("Unknown file extension {extension:?}, available extensions are: {available:?}")]
    UnknownExtension {
        extension: String,
        available: HashMap<String, String>,
    },
    #[error("No parser found for extension {extension:?}, available parsers are: {available:?}")]
    UnknownParser {
        extension: String,
        available: Vec<&'static str>,
    },
}

/// Error type triggered by the database.
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum DataBaseError {
    #[error("Failed to get file {uri:?}")]
    FileNotFound { uri: Url },
    #[error("File {uri:?} already exists")]
    FileAlreadyExists { uri: Url },
    #[error("Document error in {uri:?}: {error:?}")]
    DocumentError {
        uri: Url,
        #[source]
        error: DocumentError,
    },
}

impl From<(&Url, DocumentError)> for DataBaseError {
    fn from((uri, error): (&Url, DocumentError)) -> Self {
        DataBaseError::DocumentError {
            uri: uri.clone(),
            error,
        }
    }
}

impl From<(&Url, TreeSitterError)> for DataBaseError {
    fn from((uri, error): (&Url, TreeSitterError)) -> Self {
        DataBaseError::DocumentError {
            uri: uri.clone(),
            error: DocumentError::TreeSitter(error),
        }
    }
}

/// Error type for document handling
///
/// Produced by an error coming from either tree-sitter or texter.
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum DocumentError {
    #[error(transparent)]
    TreeSitter(#[from] TreeSitterError),
    #[error(transparent)]
    Texter(#[from] TexterError),
}

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum TreeSitterError {
    #[error("Tree sitter failed to parse tree")]
    TreeSitterParser,
}

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum TexterError {
    #[error("Texter failed to handle document")]
    TexterError(#[from] texter::error::Error),
}
