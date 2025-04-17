use std::{str::Utf8Error, sync::Arc};

use lsp_types::Url;
use thiserror::Error;

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum AutoLspError {
    #[error("{error:?}")]
    TreeSitterError {
        url: Arc<Url>,
        range: lsp_types::Range,
        error: TreeSitterError,
    },
    #[error("{error:?}")]
    DocumentError {
        url: Arc<Url>,
        range: lsp_types::Range,
        error: DocumentError,
    },
    #[error("{error:?}")]
    AstError {
        url: Arc<Url>,
        range: lsp_types::Range,
        error: String,
    },
}

impl From<AutoLspError> for lsp_types::Diagnostic {
    fn from(error: AutoLspError) -> Self {
        let range = match error {
            AutoLspError::TreeSitterError { range, .. } => range.clone(),
            AutoLspError::DocumentError { range, .. } => range.clone(),
            AutoLspError::AstError { range, .. } => range.clone(),
        };
        lsp_types::Diagnostic {
            range,
            severity: Some(lsp_types::DiagnosticSeverity::ERROR),
            message: error.to_string(),
            code: Some(lsp_types::NumberOrString::String("AUTO_LSP".into())),
            ..Default::default()
        }
    }
}

#[salsa::accumulator]
pub struct AutoLspErrorAccumulator(pub AutoLspError);

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

impl From<(Arc<Url>, TreeSitterError)> for AutoLspError {
    fn from((url, error): (Arc<Url>, TreeSitterError)) -> Self {
        let range = match &error {
            TreeSitterError::TreeSitterParser => lsp_types::Range::default(),
            TreeSitterError::Lexer { range, .. } => range.clone(),
        };
        Self::TreeSitterError { url, range, error }
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

impl From<(Arc<Url>, lsp_types::Range, DocumentError)> for AutoLspError {
    fn from((url, range, error): (Arc<Url>, lsp_types::Range, DocumentError)) -> Self {
        Self::DocumentError { url, range, error }
    }
}
