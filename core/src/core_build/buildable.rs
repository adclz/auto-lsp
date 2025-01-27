use std::sync::Arc;

use downcast_rs::{impl_downcast, Downcast};
use lsp_types::{Diagnostic, Position, Url};

use crate::{
    core_ast::{core::AstSymbol, symbol::Symbol},
    document::Document,
    workspace::Workspace,
};

use super::{
    symbol::{MaybePendingSymbol, PendingSymbol},
    utils::tree_sitter_range_to_lsp_range,
};

/// Macro to create a builder error diagnostic
///
/// This is used internally by the library to avoid redundancy when creating diagnostics during the build process
#[macro_export]
macro_rules! builder_error {
    ($range: expr, $text: expr) => {
        lsp_types::Diagnostic::new(
            $range,
            Some(lsp_types::DiagnosticSeverity::ERROR),
            None,
            None,
            $text.into(),
            None,
            None,
        )
    };
    ($path: ident, $range: expr, $text: expr) => {
        $path::lsp_types::Diagnostic::new(
            $range,
            Some($path::lsp_types::DiagnosticSeverity::ERROR),
            None,
            None,
            $text.into(),
            None,
            None,
        )
    };
}

/// Macro to create a builder warning diagnostic
///
/// This is used internally by the library to avoid redundancy when creating diagnostics during the build process
#[macro_export]
macro_rules! builder_warning {
    ($range: expr, $text: expr) => {
        lsp_types::Diagnostic::new(
            $range,
            Some(lsp_types::DiagnosticSeverity::WARNING),
            None,
            None,
            $text.into(),
            None,
            None,
        )
    };
    ($path: ident, $range: expr, $text: expr) => {
        $path::lsp_types::Diagnostic::new(
            $range,
            Some($path::lsp_types::DiagnosticSeverity::WARNING),
            None,
            None,
            $text.into(),
            None,
            None,
        )
    };
}

pub trait Buildable: Downcast {
    fn new(
        url: Arc<Url>,
        query: &tree_sitter::Query,
        capture: &tree_sitter::QueryCapture,
    ) -> Option<Self>
    where
        Self: Sized;

    fn add(
        &mut self,
        capture: &tree_sitter::QueryCapture,
        workspace: &mut Workspace,
        document: &Document,
    ) -> Result<Option<PendingSymbol>, Diagnostic>;

    fn get_url(&self) -> Arc<Url>;

    fn get_range(&self) -> std::ops::Range<usize>;

    fn get_query_index(&self) -> usize;
    fn get_lsp_range(&self, workspace: &Document) -> lsp_types::Range {
        let range = self.get_range();
        let node = workspace
            .tree
            .root_node()
            .descendant_for_byte_range(range.start, range.end)
            .unwrap();

        lsp_types::Range {
            start: Position {
                line: node.start_position().row as u32,
                character: node.start_position().column as u32,
            },
            end: Position {
                line: node.end_position().row as u32,
                character: node.end_position().column as u32,
            },
        }
    }

    fn get_text<'a>(&self, source_code: &'a [u8]) -> &'a str {
        let range = self.get_range();
        std::str::from_utf8(&source_code[range.start..range.end]).unwrap()
    }
}

impl_downcast!(Buildable);

/// List of queries associated with a struct or enum.
///
/// - struct has one query
/// - enum has as many queries as variants
///
pub trait Queryable {
    const QUERY_NAMES: &'static [&'static str];
}

pub trait Constructor<T: Buildable + Queryable> {
    fn new(
        url: Arc<Url>,
        query: &tree_sitter::Query,
        capture: &tree_sitter::QueryCapture,
    ) -> Option<T> {
        let query_name = query.capture_names()[capture.index as usize];
        if T::QUERY_NAMES.contains(&query_name) {
            T::new(url, query, capture)
        } else {
            None
        }
    }
}

pub trait AddSymbol {
    fn add<Y: Buildable + Queryable>(
        &mut self,
        capture: &tree_sitter::QueryCapture,
        workspace: &mut Workspace,
        parent_name: &str,
        field_name: &str,
    ) -> Result<Option<PendingSymbol>, Diagnostic>;
}

impl AddSymbol for PendingSymbol {
    fn add<Y: Buildable + Queryable>(
        &mut self,
        capture: &tree_sitter::QueryCapture,
        workspace: &mut Workspace,
        parent_name: &str,
        field_name: &str,
    ) -> Result<Option<PendingSymbol>, Diagnostic> {
        let name =
            workspace.parsers.tree_sitter.queries.core.capture_names()[capture.index as usize];
        if Y::QUERY_NAMES.contains(&name) {
            match Y::new(
                workspace.url.clone(),
                &workspace.parsers.tree_sitter.queries.core,
                capture,
            ) {
                Some(node) => {
                    let node = PendingSymbol::new(node);
                    *self = node.clone();
                    return Ok(None);
                }
                None => {
                    return Err(builder_error!(
                        tree_sitter_range_to_lsp_range(&capture.node.range()),
                        format!(
                            "Invalid {:?} for {:?}, expected: {:?}, received: {:?}",
                            field_name,
                            parent_name,
                            name,
                            Y::QUERY_NAMES
                        )
                    ))
                }
            }
        }
        Ok(None)
    }
}

impl AddSymbol for MaybePendingSymbol {
    fn add<Y: Buildable + Queryable>(
        &mut self,
        capture: &tree_sitter::QueryCapture,
        workspace: &mut Workspace,
        parent_name: &str,
        field_name: &str,
    ) -> Result<Option<PendingSymbol>, Diagnostic> {
        let name =
            workspace.parsers.tree_sitter.queries.core.capture_names()[capture.index as usize];
        if Y::QUERY_NAMES.contains(&name) {
            match self.as_ref() {
                Some(_) => {
                    return Ok(None);
                }
                None => match Y::new(
                    workspace.url.clone(),
                    &workspace.parsers.tree_sitter.queries.core,
                    capture,
                ) {
                    Some(node) => {
                        self.swap(&mut MaybePendingSymbol::new(node));
                        return Ok(Some(self.as_ref().unwrap().clone()));
                    }
                    None => {
                        return Err(builder_error!(
                            tree_sitter_range_to_lsp_range(&capture.node.range()),
                            format!(
                                "Invalid {:?} for {:?}, expected: {:?}, received: {:?}",
                                field_name,
                                parent_name,
                                name,
                                Y::QUERY_NAMES
                            )
                        ))
                    }
                },
            }
        }
        Ok(None)
    }
}

impl AddSymbol for Vec<PendingSymbol> {
    fn add<Y: Buildable + Queryable>(
        &mut self,
        capture: &tree_sitter::QueryCapture,
        workspace: &mut Workspace,
        parent_name: &str,
        field_name: &str,
    ) -> Result<Option<PendingSymbol>, Diagnostic> {
        let name =
            workspace.parsers.tree_sitter.queries.core.capture_names()[capture.index as usize];
        if Y::QUERY_NAMES.contains(&name) {
            match Y::new(
                workspace.url.clone(),
                &workspace.parsers.tree_sitter.queries.core,
                capture,
            ) {
                Some(node) => {
                    let node = PendingSymbol::new(node);
                    self.push(node.clone());
                    return Ok(Some(node));
                }
                None => {
                    return Err(builder_error!(
                        tree_sitter_range_to_lsp_range(&capture.node.range()),
                        format!(
                            "Invalid {:?} for {:?}, expected: {:?}, received: {:?}",
                            field_name,
                            parent_name,
                            name,
                            Y::QUERY_NAMES
                        )
                    ))
                }
            }
        }
        Ok(None)
    }
}

pub trait Finalize<T: AstSymbol> {
    type Output;

    fn finalize(self, workspace: &mut Workspace) -> Self::Output;
}

impl<T: AstSymbol> Finalize<T> for T {
    type Output = Symbol<T>;

    fn finalize(self, workspace: &mut Workspace) -> Self::Output {
        Symbol::new_and_check(self, workspace)
    }
}

impl<T: AstSymbol> Finalize<T> for Option<T> {
    type Output = Option<Symbol<T>>;

    fn finalize(self, workspace: &mut Workspace) -> Self::Output {
        match self {
            Some(symbol) => Some(Symbol::new_and_check(symbol, workspace)),
            None => None,
        }
    }
}

impl<T: AstSymbol> Finalize<T> for Vec<T> {
    type Output = Vec<Symbol<T>>;

    fn finalize(self, workspace: &mut Workspace) -> Self::Output {
        self.into_iter()
            .map(|f| Symbol::new_and_check(f, workspace))
            .collect()
    }
}
