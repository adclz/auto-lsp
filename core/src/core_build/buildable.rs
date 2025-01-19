use std::sync::Arc;

use downcast_rs::{impl_downcast, Downcast};
use lsp_types::{Diagnostic, Position, Url};

use crate::{
    core_ast::{core::AstSymbol, symbol::Symbol},
    workspace::Document,
};

use super::{
    downcast::TryFromBuilder,
    main_builder::MainBuilder,
    stack_builder::StackBuilder,
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
        params: &mut MainBuilder,
    ) -> Result<Option<PendingSymbol>, Diagnostic>;

    fn get_url(&self) -> Arc<Url>;

    fn get_range(&self) -> std::ops::Range<usize>;

    fn get_query_index(&self) -> usize;
    fn get_lsp_range(&self, workspace: &Document) -> lsp_types::Range {
        let range = self.get_range();
        let node = workspace
            .cst
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

pub trait StaticBuildable<
    T: Buildable + Queryable,
    Y: AstSymbol + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
>
{
    fn static_build<'a>(
        params: &'a mut MainBuilder,
        range: Option<std::ops::Range<usize>>,
    ) -> Result<Y, Diagnostic>;
}

impl<T, Y> StaticBuildable<T, Y> for Y
where
    T: Buildable + Queryable,
    Y: AstSymbol + for<'b> TryFromBuilder<&'b T, Error = lsp_types::Diagnostic>,
{
    fn static_build<'a>(
        builder_params: &'a mut MainBuilder,
        range: Option<std::ops::Range<usize>>,
    ) -> Result<Y, Diagnostic> {
        StackBuilder::<T>::new(builder_params)
            .build(&range)
            .to_static_symbol(&range)
    }
}

/// List of queries associated with a struct or enum.
///
/// - struct has one query
/// - enum has as many queries as variants
///
pub trait Queryable {
    const QUERY_NAMES: &'static [&'static str];
}

/// Call [`check_conflicts`] to find duplicated queries with a struct or enum
///
/// Executed at compile time
#[cfg(feature = "assertions")]
pub trait CheckQueryable {
    const CHECK: ();
}

/// Compare the names of the queries and panic if there are duplicates
#[cfg(feature = "assertions")]
pub const fn check_conflicts(
    input_name: &str,
    fields: &'static [&'static str],
    names: &'static [&'static str],
) {
    let mut i = 0;
    while i < names.len() {
        let mut j = i + 1;
        while j < names.len() {
            const_panic::concat_assert!(
                !const_str::equal!(names[i], names[j]),
                "\n\n\n**** Conflicting Queries detected! ****\n\n",
                "\nSymbol -->           ",
                input_name,
                "\nQuery name -->       ",
                names[i],
                "\n\n\n1# : ",
                fields[i],
                "\n2# : ",
                fields[j],
                "\n\n\n\n"
            );
            j += 1;
        }
        i += 1;
    }
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
        params: &mut MainBuilder,
        parent_name: &str,
        field_name: &str,
    ) -> Result<Option<PendingSymbol>, Diagnostic>;
}

impl AddSymbol for PendingSymbol {
    fn add<Y: Buildable + Queryable>(
        &mut self,
        capture: &tree_sitter::QueryCapture,
        params: &mut MainBuilder,
        parent_name: &str,
        field_name: &str,
    ) -> Result<Option<PendingSymbol>, Diagnostic> {
        let name = params.query.capture_names()[capture.index as usize];
        if Y::QUERY_NAMES.contains(&name) {
            match Y::new(params.url.clone(), params.query, capture) {
                Some(node) => {
                    let node = PendingSymbol::new(node);
                    *self = node.clone();
                    return Ok(None);
                }
                None => {
                    return Err(builder_error!(
                        tree_sitter_range_to_lsp_range(&capture.node.range()),
                        format!("Invalid {:?} for {:?}", field_name, parent_name)
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
        params: &mut MainBuilder,
        parent_name: &str,
        field_name: &str,
    ) -> Result<Option<PendingSymbol>, Diagnostic> {
        let name = params.query.capture_names()[capture.index as usize];
        if Y::QUERY_NAMES.contains(&name) {
            match self.as_ref() {
                Some(_) => {
                    return Err(builder_error!(
                        tree_sitter_range_to_lsp_range(&capture.node.range()),
                        format!("{:?} already set in {:?}", field_name, parent_name)
                    ));
                }
                None => match Y::new(params.url.clone(), params.query, capture) {
                    Some(node) => {
                        self.swap(&mut MaybePendingSymbol::new(node));
                        return Ok(Some(self.as_ref().unwrap().clone()));
                    }
                    None => {
                        return Err(builder_error!(
                            tree_sitter_range_to_lsp_range(&capture.node.range()),
                            format!("Invalid {:?} for {:?}", field_name, parent_name)
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
        params: &mut MainBuilder,
        parent_name: &str,
        field_name: &str,
    ) -> Result<Option<PendingSymbol>, Diagnostic> {
        let name = params.query.capture_names()[capture.index as usize];

        if Y::QUERY_NAMES.contains(&name) {
            match Y::new(params.url.clone(), params.query, capture) {
                Some(node) => {
                    let node = PendingSymbol::new(node);
                    self.push(node.clone());
                    return Ok(Some(node));
                }
                None => {
                    return Err(builder_error!(
                        tree_sitter_range_to_lsp_range(&capture.node.range()),
                        format!("Invalid {:?} for {:?}", field_name, parent_name)
                    ))
                }
            }
        }
        Ok(None)
    }
}

pub trait Finalize<T: AstSymbol> {
    type Output;

    fn finalize(self, params: &mut MainBuilder) -> Self::Output;
}

impl<T: AstSymbol> Finalize<T> for T {
    type Output = Symbol<T>;

    fn finalize(self, params: &mut MainBuilder) -> Self::Output {
        Symbol::new_and_check(self, params)
    }
}

impl<T: AstSymbol> Finalize<T> for Option<T> {
    type Output = Option<Symbol<T>>;

    fn finalize(self, params: &mut MainBuilder) -> Self::Output {
        match self {
            Some(symbol) => Some(Symbol::new_and_check(symbol, params)),
            None => None,
        }
    }
}

impl<T: AstSymbol> Finalize<T> for Vec<T> {
    type Output = Vec<Symbol<T>>;

    fn finalize(self, params: &mut MainBuilder) -> Self::Output {
        self.into_iter()
            .map(|f| Symbol::new_and_check(f, params))
            .collect()
    }
}
