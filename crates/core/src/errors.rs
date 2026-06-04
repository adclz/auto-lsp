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

use std::{path::PathBuf, str::Utf8Error};

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
    #[error("{error}")]
    LexerError {
        span: tree_sitter::Range,
        #[source]
        error: LexerError,
    },
    #[error("{error}")]
    AstError {
        span: tree_sitter::Range,
        #[source]
        error: AstError,
    },
}

impl ParseError {
    pub fn to_lsp_diagnostic(
        &self,
        doc: &Document,
    ) -> Result<lsp_types::Diagnostic, DocumentError> {
        let (range, message) = match self {
            ParseError::AstError { span: range, error } => (range, error.to_string()),
            ParseError::LexerError { span: range, error } => (range, error.to_string()),
        };
        Ok(lsp_types::Diagnostic {
            range: doc.denormalize_range(range)?,
            severity: Some(lsp_types::DiagnosticSeverity::ERROR),
            message,
            code: Some(lsp_types::NumberOrString::String("AUTO_LSP".into())),
            ..Default::default()
        })
    }

    /// Creates a label for the error using ariadne.
    pub fn to_label(
        &self,
        source: &Source<&str>,
        colors: &mut ColorGenerator,
        report: &mut ReportBuilder<'_, std::ops::Range<usize>>,
    ) {
        let range = match self {
            ParseError::LexerError { span: range, .. } => range,
            ParseError::AstError { span: range, .. } => range,
        };
        let start_line = source.line(range.start_point.column).unwrap().offset();
        let end_line = source.line(range.end_point.row).unwrap().offset();
        let start = start_line + range.start_point.column;
        let end = end_line + range.end_point.column;
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
    #[error("Unexpected {symbol} in {parent_name}")]
    UnexpectedSymbol {
        range: tree_sitter::Range,
        symbol: &'static str,
        parent_name: &'static str,
    },
}

impl From<AstError> for ParseError {
    fn from(error: AstError) -> Self {
        let range = match &error {
            AstError::UnexpectedSymbol { range, .. } => *range,
        };
        Self::AstError {
            span: range.into(),
            error,
        }
    }
}

/// Error type for tree-sitter.
///
/// Can either be a syntax error or a missing symbol error.
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum LexerError {
    #[error("{error}")]
    Missing {
        range: tree_sitter::Range,
        error: String,
        // Missing node's symbol name as it appears in the grammar ignoring aliases as a string
        grammar_name: &'static str,
    },
    #[error("{error}")]
    Syntax {
        range: tree_sitter::Range,
        error: String,
        affected: String,
    },
}

impl From<LexerError> for ParseError {
    fn from(error: LexerError) -> Self {
        let range = match &error {
            LexerError::Missing { range, .. } => *range,
            LexerError::Syntax { range, .. } => *range,
        };
        Self::LexerError {
            span: range.into(),
            error,
        }
    }
}

/// Main accumulator for parse errors
///
/// This is meant to be used in salsa queries to accumulate parse errors.
#[derive(Debug)]
#[salsa::accumulator]
pub struct ParseErrorAccumulator(pub ParseError);

impl ParseErrorAccumulator {
    pub fn to_lsp_diagnostic(
        &self,
        doc: &Document,
    ) -> Result<lsp_types::Diagnostic, DocumentError> {
        self.0.to_lsp_diagnostic(doc)
    }

    pub fn to_label(
        &self,
        source: &Source<&str>,
        colors: &mut ColorGenerator,
        report: &mut ReportBuilder<'_, std::ops::Range<usize>>,
    ) {
        self.0.to_label(source, colors, report);
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

impl From<AstError> for ParseErrorAccumulator {
    fn from(error: AstError) -> Self {
        Self(ParseError::from(error))
    }
}

/// Error type for position errors.
///
/// Emitted by [`crate::ast::AstNode::get_text`] when slicing source code.
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum PositionError {
    #[error("Failed to get text in {range:?}")]
    WrongTextRange { range: std::ops::Range<usize> },
    #[error("Failed to get text in {range:?}: Encountered UTF-8 error {utf8_error}")]
    UTF8Error {
        range: std::ops::Range<usize>,
        utf8_error: Utf8Error,
    },
}

/// Error type produced by the runtime - aka the server -.
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum RuntimeError {
    #[error("Document error in {uri}: {error}")]
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
    #[error("Failed to convert url {path} to file path")]
    FileUrlToFilePath { path: Url },
    #[error("Failed to convert file path {path} to url")]
    FilePathToUrl { path: PathBuf },
    #[error("Failed to open file {path}: {error}")]
    FileOpen { path: Url, error: String },
    #[error("Failed to read file {path}: {error}")]
    FileRead { path: Url, error: String },
}

/// Error type triggered by the database.
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum DataBaseError {
    #[error("Failed to get file {uri}")]
    FileNotFound { uri: Url },
    #[error("File {uri} already exists")]
    FileAlreadyExists { uri: Url },
    #[error("Document error in {uri}: {error}")]
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

impl From<texter::error::Error> for DocumentError {
    fn from(error: texter::error::Error) -> Self {
        DocumentError::Texter(TexterError::from(error))
    }
}
