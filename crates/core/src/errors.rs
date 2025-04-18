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

use std::str::Utf8Error;

use ariadne::{ColorGenerator, Fmt, Label, ReportBuilder, Source};
use thiserror::Error;

use crate::document::Document;

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum AutoLspError {
    #[error("{error:?}")]
    TreeSitterError {
        range: lsp_types::Range,
        error: TreeSitterError,
    },
    #[error("{error:?}")]
    TexterError {
        range: lsp_types::Range,
        error: TexterError,
    },
    #[error("{error:?}")]
    AstError {
        range: lsp_types::Range,
        error: AstError,
    },
}

impl From<&AutoLspError> for lsp_types::Diagnostic {
    fn from(error: &AutoLspError) -> Self {
        let message = error.to_string();
        let range = match error {
            AutoLspError::TreeSitterError { range, .. } => *range,
            AutoLspError::AstError { range, .. } => *range,
            AutoLspError::TexterError { range, .. } => *range,
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

impl AutoLspError {
    pub fn to_label(
        &self,
        source: &Source<&str>,
        colors: &mut ColorGenerator,
        report: &mut ReportBuilder<'_, std::ops::Range<usize>>,
    ) {
        let range = match self {
            AutoLspError::TreeSitterError { range, .. } => range,
            AutoLspError::AstError { range, .. } => range,
            AutoLspError::TexterError { range, .. } => range,
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

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum AstError {
    #[error("No root node found with {query:?}")]
    NoRootNode {
        range: std::ops::Range<usize>,
        query: &'static [&'static str],
    },
    #[error("Failed to create root node with {query:?}")]
    InvalidRootNode {
        range: std::ops::Range<usize>,
        query: &'static str,
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
    #[error("Missing symbol {symbol:?} in {parent_name:?}")]
    MissingSymbol {
        range: std::ops::Range<usize>,
        symbol: &'static str,
        parent_name: &'static str,
    },
}

impl From<(&Document, AstError)> for AutoLspError {
    fn from((document, error): (&Document, AstError)) -> Self {
        let range = match &error {
            AstError::NoRootNode { range, .. } => range,
            AstError::InvalidRootNode { range, .. } => range,
            AstError::UnknownSymbol { range, .. } => range,
            AstError::InvalidSymbol { range, .. } => range,
            AstError::MissingSymbol { range, .. } => range,
        };
        let range = document.range_at(range.clone()).unwrap_or_default();
        Self::AstError { range, error }
    }
}

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum TreeSitterError {
    #[error("Tree sitter failed to parse tree")]
    TreeSitterParser,
    #[error("{error:?}")]
    Lexer {
        range: lsp_types::Range,
        error: String,
    },
}

impl From<TreeSitterError> for AutoLspError {
    fn from(error: TreeSitterError) -> Self {
        let range = match &error {
            TreeSitterError::TreeSitterParser => lsp_types::Range::default(),
            TreeSitterError::Lexer { range, .. } => *range,
        };
        Self::TreeSitterError { range, error }
    }
}

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum TexterError {
    #[error("texter failed to handle document")]
    TexterError(#[from] texter::error::Error),
}

impl From<TexterError> for AutoLspError {
    fn from(error: TexterError) -> Self {
        let range = match &error {
            TexterError::TexterError(_) => lsp_types::Range::default(),
        };
        Self::TexterError { range, error }
    }
}

#[salsa::accumulator]
pub struct AutoLspErrorAccumulator(pub AutoLspError);

impl AutoLspErrorAccumulator {
    pub fn to_label(
        &self,
        source: &Source<&str>,
        colors: &mut ColorGenerator,
        report: &mut ReportBuilder<'_, std::ops::Range<usize>>,
    ) {
        self.0.to_label(source, colors, report);
    }
}

impl From<&AutoLspErrorAccumulator> for lsp_types::Diagnostic {
    fn from(error: &AutoLspErrorAccumulator) -> Self {
        Self::from(&error.0)
    }
}

impl From<&AutoLspError> for AutoLspErrorAccumulator {
    fn from(diagnostic: &AutoLspError) -> Self {
        Self(diagnostic.clone())
    }
}

impl From<AutoLspError> for AutoLspErrorAccumulator {
    fn from(diagnostic: AutoLspError) -> Self {
        Self(diagnostic)
    }
}

impl From<&AutoLspErrorAccumulator> for AutoLspError {
    fn from(diagnostic: &AutoLspErrorAccumulator) -> Self {
        diagnostic.0.clone()
    }
}

impl From<TreeSitterError> for AutoLspErrorAccumulator {
    fn from(error: TreeSitterError) -> Self {
        Self(error.into())
    }
}

impl From<TexterError> for AutoLspErrorAccumulator {
    fn from(error: TexterError) -> Self {
        Self(error.into())
    }
}

impl From<(&Document, AstError)> for AutoLspErrorAccumulator {
    fn from((document, error): (&Document, AstError)) -> Self {
        Self(AutoLspError::from((document, error)))
    }
}

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum DocumentError {
    #[error("Can not find position of offset {offset:?}, max line length is {length:?}")]
    DocumentLineOutOfBound { offset: usize, length: usize },
    #[error("Failed to get position of offset {offset:?}")]
    DocumentPosition { offset: usize },
    #[error("Failed to get range of {range:?}: {position_error:?}")]
    DocumentRange {
        range: std::ops::Range<usize>,
        #[source]
        position_error: Box<DocumentError>,
    },
    #[error("Failed to get text in {range:?}")]
    DocumentTextRange { range: std::ops::Range<usize> },
    #[error("Failed to get text in {range:?}: Encountered UTF-8 error {utf8_error:?}")]
    DocumentTextUTF8 {
        range: std::ops::Range<usize>,
        utf8_error: Utf8Error,
    },
}
