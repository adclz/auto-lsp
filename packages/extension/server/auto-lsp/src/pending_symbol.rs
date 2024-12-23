use downcast_rs::{impl_downcast, Downcast};
use lsp_textdocument::FullTextDocument;
use lsp_types::{Diagnostic, Url};
use std::cell::RefCell;
use std::fmt::Formatter;
use std::rc::Rc;
use std::sync::Arc;
use tree_sitter::Query;

use crate::builder_error;
use crate::builders::BuilderParams;

use super::convert::{TryFromBuilder, TryIntoBuilder};
use super::queryable::Queryable;
use super::symbol::{AstSymbol, DynSymbol, Symbol};

pub trait AstBuilder: Downcast {
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
        params: &mut BuilderParams,
    ) -> Result<(), Diagnostic>;

    fn try_to_dyn_symbol(
        &self,
        check: &mut BuilderParams,
    ) -> Result<DynSymbol, lsp_types::Diagnostic>;

    fn get_url(&self) -> Arc<Url>;

    fn get_range(&self) -> std::ops::Range<usize>;

    fn get_query_index(&self) -> usize;

    fn get_lsp_range(&self, doc: &FullTextDocument) -> lsp_types::Range {
        let range = self.get_range();
        let start = range.start;
        let end = range.start;
        lsp_types::Range {
            start: doc.position_at(start as u32).into(),
            end: doc.position_at(end as u32).into(),
        }
    }

    fn get_text<'a>(&self, source_code: &'a [u8]) -> &'a str {
        let range = self.get_range();
        std::str::from_utf8(&source_code[range.start..range.end]).unwrap()
    }
}

impl std::fmt::Debug for dyn AstBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "At - {:?}", self.get_range())
    }
}

impl_downcast!(AstBuilder);

#[derive(Clone)]
pub struct PendingSymbol(Rc<RefCell<dyn AstBuilder>>);

impl PendingSymbol {
    pub fn new(builder: impl AstBuilder) -> Self {
        PendingSymbol(Rc::new(RefCell::new(builder)))
    }

    pub fn get_rc(&self) -> &Rc<RefCell<dyn AstBuilder>> {
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

    pub fn new(builder: impl AstBuilder) -> Self {
        MaybePendingSymbol(Some(PendingSymbol::new(builder)))
    }

    pub fn from_pending(pending: PendingSymbol) -> Self {
        MaybePendingSymbol(Some(pending))
    }

    pub fn as_ref(&self) -> Option<&PendingSymbol> {
        self.0.as_ref().map(|pending| pending)
    }

    pub fn as_mut(&mut self) -> Option<&mut PendingSymbol> {
        self.0.as_mut().map(|pending| pending)
    }

    pub fn into_inner(self) -> Option<PendingSymbol> {
        self.0
    }
}

impl std::fmt::Debug for MaybePendingSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "-")
    }
}

pub trait TryDownCast<
    T: AstBuilder,
    Y: AstSymbol + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
>
{
    type Output;

    fn try_downcast(
        &self,
        check: &mut BuilderParams,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<Self::Output, Diagnostic>;
}

impl<T, Y> TryDownCast<T, Y> for PendingSymbol
where
    T: AstBuilder,
    Y: AstSymbol + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
{
    type Output = Y;
    fn try_downcast(
        &self,
        check: &mut BuilderParams,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<Self::Output, Diagnostic> {
        self.0
            .borrow()
            .downcast_ref::<T>()
            .ok_or(builder_error!(
                field_range,
                format!(
                    "Could not cast field {:?} into {:?}",
                    field_name, input_name
                )
            ))?
            .try_into_builder(check)
    }
}

impl<T, Y> TryDownCast<T, Y> for MaybePendingSymbol
where
    T: AstBuilder,
    Y: AstSymbol + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
{
    type Output = Option<Y>;

    fn try_downcast(
        &self,
        check: &mut BuilderParams,
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
    T: TryDownCast<Y, V, Output = V>,
    Y: AstBuilder,
    V: AstSymbol + for<'a> TryFromBuilder<&'a Y, Error = lsp_types::Diagnostic>,
{
    type Output = Vec<V>;

    fn try_downcast(
        &self,
        check: &mut BuilderParams,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<Self::Output, Diagnostic> {
        self.iter()
            .map(|item| item.try_downcast(check, field_name, field_range, input_name))
            .collect::<Result<Vec<_>, lsp_types::Diagnostic>>()
    }
}

pub trait Finalize<T: AstSymbol> {
    type Output;

    fn finalize(self, checks: &mut Vec<DynSymbol>) -> Self::Output;
}

impl<T: AstSymbol> Finalize<T> for T {
    type Output = Symbol<T>;

    fn finalize(self, checks: &mut Vec<DynSymbol>) -> Self::Output {
        Symbol::new_and_check(self, checks)
    }
}

impl<T: AstSymbol> Finalize<T> for Vec<T> {
    type Output = Vec<Symbol<T>>;

    fn finalize(self, checks: &mut Vec<DynSymbol>) -> Self::Output {
        self.into_iter()
            .map(|f| Symbol::new_and_check(f, checks))
            .collect()
    }
}

pub trait Constructor<T: AstBuilder + Queryable> {
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
