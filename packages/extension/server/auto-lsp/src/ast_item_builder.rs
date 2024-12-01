use downcast_rs::{impl_downcast, Downcast};
use lsp_types::{Diagnostic, Url};
use std::cell::RefCell;
use std::fmt::Formatter;
use std::rc::Rc;
use std::sync::Arc;
use tree_sitter::Query;

use crate::builder_error;

use super::ast_item::{AstItem, DynSymbol, Symbol};
use super::convert::{TryFromBuilder, TryIntoBuilder};
use super::queryable::Queryable;

pub trait AstItemBuilder: Downcast {
    fn new(
        url: Arc<Url>,
        _query: &tree_sitter::Query,
        query_index: usize,
        range: tree_sitter::Range,
        start_position: tree_sitter::Point,
        end_position: tree_sitter::Point,
    ) -> Option<Self>
    where
        Self: Sized;

    fn query_binder(
        &self,
        url: Arc<Url>,
        capture: &tree_sitter::QueryCapture,
        query: &tree_sitter::Query,
    ) -> MaybePendingSymbol;

    fn add(
        &mut self,
        query: &Query,
        node: PendingSymbol,
        source_code: &[u8],
    ) -> Result<(), Diagnostic>;

    fn try_to_dyn_symbol(
        &self,
        check: &mut Vec<DynSymbol>,
    ) -> Result<DynSymbol, lsp_types::Diagnostic>;

    fn get_url(&self) -> Arc<Url>;

    fn get_range(&self) -> tree_sitter::Range;

    fn get_query_index(&self) -> usize;

    fn get_start_position(&self) -> tree_sitter::Point {
        self.get_range().start_point
    }

    fn get_end_position(&self) -> tree_sitter::Point {
        self.get_range().end_point
    }

    fn get_lsp_range(&self) -> lsp_types::Range {
        let range = self.get_range();
        let start = range.start_point;
        let end = range.end_point;
        lsp_types::Range {
            start: lsp_types::Position {
                line: start.row as u32,
                character: start.column as u32,
            },
            end: lsp_types::Position {
                line: end.row as u32,
                character: end.column as u32,
            },
        }
    }

    fn get_text<'a>(&self, source_code: &'a [u8]) -> &'a str {
        let range = self.get_range();
        std::str::from_utf8(&source_code[range.start_byte..range.end_byte]).unwrap()
    }
}

impl std::fmt::Debug for dyn AstItemBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "At - {:?}", self.get_range())
    }
}

impl_downcast!(AstItemBuilder);

#[derive(Clone)]
pub struct PendingSymbol(Rc<RefCell<dyn AstItemBuilder>>);

impl PendingSymbol {
    pub fn new(builder: impl AstItemBuilder) -> Self {
        PendingSymbol(Rc::new(RefCell::new(builder)))
    }

    pub fn get_rc(&self) -> &Rc<RefCell<dyn AstItemBuilder>> {
        &self.0
    }

    pub fn get_query_index(&self) -> usize {
        self.0.borrow().get_query_index()
    }
}

impl std::fmt::Debug for PendingSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "-")
    }
}

#[derive(Clone)]
pub struct MaybePendingSymbol(Option<PendingSymbol>);

impl MaybePendingSymbol {
    pub fn none() -> Self {
        MaybePendingSymbol(None)
    }

    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    pub fn new(builder: impl AstItemBuilder) -> Self {
        MaybePendingSymbol(Some(PendingSymbol::new(builder)))
    }

    pub fn from_pending(pending: PendingSymbol) -> Self {
        MaybePendingSymbol(Some(pending))
    }

    pub fn as_ref(&self) -> Option<&PendingSymbol> {
        self.0.as_ref().map(|pending| pending)
    }
}

impl std::fmt::Debug for MaybePendingSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "-")
    }
}

pub trait TryDownCast<
    T: AstItemBuilder,
    Y: Clone + AstItem + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
>
{
    type Output;

    fn try_downcast(
        &self,
        check: &mut Vec<DynSymbol>,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<Self::Output, Diagnostic>;
}

impl<T, Y> TryDownCast<T, Y> for PendingSymbol
where
    T: AstItemBuilder,
    Y: Clone + AstItem + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
{
    type Output = Symbol<Y>;
    fn try_downcast(
        &self,
        check: &mut Vec<DynSymbol>,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<Self::Output, Diagnostic> {
        let item: Y = self
            .0
            .borrow()
            .downcast_ref::<T>()
            .ok_or(builder_error!(
                field_range,
                format!(
                    "Could not cast field {:?} into {:?}",
                    field_name, input_name
                )
            ))?
            .try_into_builder(check)?;
        let arc = Symbol::new(item);
        if *arc.read().is_accessor() {
            check.push(arc.to_dyn());
        }
        arc.write().set_parent(arc.to_weak());
        Ok(arc)
    }
}

impl<T, Y> TryDownCast<T, Y> for MaybePendingSymbol
where
    T: AstItemBuilder,
    Y: Clone + AstItem + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
{
    type Output = Option<Symbol<Y>>;

    fn try_downcast(
        &self,
        check: &mut Vec<DynSymbol>,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<Self::Output, Diagnostic> {
        self.0.as_ref().map_or(Ok(None), |pending| {
            pending
                .try_downcast(check, field_name, field_range, input_name)
                .map(Some)
        })
    }
}

impl<T, Y, V> TryDownCast<Y, V> for Vec<T>
where
    T: TryDownCast<Y, V, Output = Symbol<V>>,
    Y: AstItemBuilder,
    V: Clone + AstItem + for<'a> TryFromBuilder<&'a Y, Error = lsp_types::Diagnostic>,
{
    type Output = Vec<Symbol<V>>;

    fn try_downcast(
        &self,
        check: &mut Vec<DynSymbol>,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<Self::Output, Diagnostic> {
        self.iter()
            .map(|item| item.try_downcast(check, field_name, field_range, input_name))
            .collect::<Result<Vec<_>, lsp_types::Diagnostic>>()
    }
}

pub trait Constructor<T: AstItemBuilder + Queryable> {
    fn new(
        url: Arc<Url>,
        query: &tree_sitter::Query,
        query_index: usize,
        range: tree_sitter::Range,
        start_position: tree_sitter::Point,
        end_position: tree_sitter::Point,
    ) -> Option<T> {
        let query_name = query.capture_names()[query_index];
        if T::QUERY_NAMES.contains(&query_name) {
            T::new(url, query, query_index, range, start_position, end_position)
        } else {
            None
        }
    }
}

pub trait AddSymbol {
    fn add<Y: Queryable>(
        &mut self,
        query_name: &str,
        node: PendingSymbol,
        range: lsp_types::Range,
        parent_name: &str,
        field_name: &str,
    ) -> Result<Option<PendingSymbol>, Diagnostic>;
}

impl AddSymbol for MaybePendingSymbol {
    fn add<Y: Queryable>(
        &mut self,
        query_name: &str,
        node: PendingSymbol,
        range: lsp_types::Range,
        parent_name: &str,
        field_name: &str,
    ) -> Result<Option<PendingSymbol>, Diagnostic> {
        if Y::QUERY_NAMES.contains(&query_name) {
            match self.0 {
                Some(_) => {
                    return Err(builder_error!(
                        range,
                        format!(
                            "Field {:?} already set in {:?} for {:?}",
                            field_name, parent_name, query_name
                        )
                    ))
                }
                None => {
                    self.0 = Some(node);
                    return Ok(None);
                }
            }
        }
        Ok(Some(node))
    }
}

impl AddSymbol for Vec<PendingSymbol> {
    fn add<Y: Queryable>(
        &mut self,
        query_name: &str,
        node: PendingSymbol,
        _range: lsp_types::Range,
        _parent_name: &str,
        _field_name: &str,
    ) -> Result<Option<PendingSymbol>, Diagnostic> {
        if Y::QUERY_NAMES.contains(&query_name) {
            self.push(node);
            return Ok(None);
        }
        Ok(Some(node))
    }
}
