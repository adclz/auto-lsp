use downcast_rs::{impl_downcast, Downcast};
use lsp_types::{Diagnostic, Url};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use tree_sitter::Query;

use crate::builder_error;

use super::ast_item::AstItem;
use super::convert::{TryFromBuilder, TryIntoBuilder};

pub trait TryDownCast {
    fn try_downcast<
        T: AstItemBuilder,
        Y: AstItem + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
    >(
        &self,
        check: &mut Vec<Arc<RwLock<dyn AstItem>>>,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<Arc<RwLock<Y>>, Diagnostic>;
}

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

impl TryDownCast for PendingSymbol {
    fn try_downcast<
        T: AstItemBuilder,
        Y: AstItem + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
    >(
        &self,
        check: &mut Vec<Arc<RwLock<dyn AstItem>>>,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<Arc<RwLock<Y>>, Diagnostic> {
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
        let arc = Arc::new(RwLock::new(item));
        arc.write().unwrap().set_parent(Arc::downgrade(&arc) as _);
        Ok(arc)
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

impl TryDownCast for MaybePendingSymbol {
    fn try_downcast<
        T: AstItemBuilder,
        Y: AstItem + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
    >(
        &self,
        check: &mut Vec<Arc<RwLock<dyn AstItem>>>,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<Arc<RwLock<Y>>, Diagnostic> {
        self.0
            .as_ref()
            .ok_or(builder_error!(
                field_range,
                format!("Missing field {:?} in {:?}", field_name, input_name)
            ))?
            .try_downcast::<T, Y>(check, field_name, field_range, input_name)
    }
}

impl std::fmt::Debug for MaybePendingSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "-")
    }
}

pub trait TryDownCastVec {
    fn try_downcast_vec<
        T: AstItemBuilder,
        Y: AstItem + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
    >(
        &self,
        check: &mut Vec<Arc<RwLock<dyn AstItem>>>,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<Vec<Arc<RwLock<Y>>>, Diagnostic>;
}

impl<T: TryDownCast> TryDownCastVec for Vec<T> {
    fn try_downcast_vec<
        Y: AstItemBuilder,
        V: AstItem + for<'a> TryFromBuilder<&'a Y, Error = lsp_types::Diagnostic>,
    >(
        &self,
        check: &mut Vec<Arc<RwLock<dyn AstItem>>>,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<Vec<Arc<RwLock<V>>>, Diagnostic> {
        self.iter()
            .map(|item| item.try_downcast::<Y, V>(check, field_name, field_range, input_name))
            .collect::<Result<Vec<_>, lsp_types::Diagnostic>>()
    }
}

pub trait TryDownCastMap {
    fn try_downcast_map<
        T: AstItemBuilder,
        Y: AstItem + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
    >(
        &self,
        check: &mut Vec<Arc<RwLock<dyn AstItem>>>,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<HashMap<String, Arc<RwLock<Y>>>, Diagnostic>;
}

impl<T: TryDownCast> TryDownCastMap for HashMap<String, T> {
    fn try_downcast_map<
        Y: AstItemBuilder,
        V: AstItem + for<'a> TryFromBuilder<&'a Y, Error = lsp_types::Diagnostic>,
    >(
        &self,
        check: &mut Vec<Arc<RwLock<dyn AstItem>>>,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<HashMap<String, Arc<RwLock<V>>>, Diagnostic> {
        self.iter()
            .map(|(key, item)| {
                item.try_downcast::<Y, V>(check, field_name, field_range, input_name)
                    .map(|item| (key.clone(), item))
            })
            .collect::<Result<HashMap<_, _>, lsp_types::Diagnostic>>()
    }
}

pub type DeferredClosure = Box<
    dyn Fn(
        PendingSymbol, // parent
        PendingSymbol, // child
        &[u8],         // source_code
    ) -> Result<(), Diagnostic>,
>;

pub trait AstItemBuilder: Downcast {
    fn new(
        url: Arc<lsp_types::Url>,
        _query: &tree_sitter::Query,
        query_index: usize,
        range: tree_sitter::Range,
        start_position: tree_sitter::Point,
        end_position: tree_sitter::Point,
    ) -> Option<Self>
    where
        Self: Sized;

    fn static_query_binder(
        url: Arc<Url>,
        capture: &tree_sitter::QueryCapture,
        query: &tree_sitter::Query,
    ) -> MaybePendingSymbol
    where
        Self: Sized;

    fn try_into_item(
        &self,
        check: &mut Vec<Arc<RwLock<dyn AstItem>>>,
    ) -> Result<Arc<RwLock<dyn AstItem>>, lsp_types::Diagnostic>;

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
    ) -> Result<Option<DeferredClosure>, Diagnostic>;

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
