use downcast_rs::{impl_downcast, Downcast};
use lsp_types::{Diagnostic, Position, Url};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use crate::builder_error;
use crate::builders::{tree_sitter_range_to_lsp_range, BuilderParams};
use crate::workspace::Document;

use super::convert::{TryFromBuilder, TryIntoBuilder};
use super::queryable::Queryable;
use super::symbol::{AstSymbol, Symbol};

pub trait AstBuilder: Downcast {
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
        params: &mut BuilderParams,
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
                format!("Invalid {:?} for {:?}", field_name, input_name)
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

    fn finalize(self, params: &mut BuilderParams) -> Self::Output;
}

impl<T: AstSymbol> Finalize<T> for T {
    type Output = Symbol<T>;

    fn finalize(self, params: &mut BuilderParams) -> Self::Output {
        Symbol::new_and_check(self, params)
    }
}

impl<T: AstSymbol> Finalize<T> for Option<T> {
    type Output = Option<Symbol<T>>;

    fn finalize(self, params: &mut BuilderParams) -> Self::Output {
        match self {
            Some(symbol) => Some(Symbol::new_and_check(symbol, params)),
            None => None,
        }
    }
}

impl<T: AstSymbol> Finalize<T> for Vec<T> {
    type Output = Vec<Symbol<T>>;

    fn finalize(self, params: &mut BuilderParams) -> Self::Output {
        self.into_iter()
            .map(|f| Symbol::new_and_check(f, params))
            .collect()
    }
}

pub trait Constructor<T: AstBuilder + Queryable> {
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
    fn add<Y: AstBuilder + Queryable>(
        &mut self,
        capture: &tree_sitter::QueryCapture,
        params: &mut BuilderParams,
        parent_name: &str,
        field_name: &str,
    ) -> Result<Option<PendingSymbol>, Diagnostic>;
}

impl AddSymbol for PendingSymbol {
    fn add<Y: AstBuilder + Queryable>(
        &mut self,
        capture: &tree_sitter::QueryCapture,
        params: &mut BuilderParams,
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
    fn add<Y: AstBuilder + Queryable>(
        &mut self,
        capture: &tree_sitter::QueryCapture,
        params: &mut BuilderParams,
        parent_name: &str,
        field_name: &str,
    ) -> Result<Option<PendingSymbol>, Diagnostic> {
        let name = params.query.capture_names()[capture.index as usize];
        if Y::QUERY_NAMES.contains(&name) {
            match self.0 {
                Some(_) => {
                    return Err(builder_error!(
                        tree_sitter_range_to_lsp_range(&capture.node.range()),
                        format!("{:?} already set in {:?}", field_name, parent_name)
                    ));
                }
                None => match Y::new(params.url.clone(), params.query, capture) {
                    Some(node) => {
                        self.0 = Some(PendingSymbol::new(node));
                        return Ok(self.0.clone());
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
    fn add<Y: AstBuilder + Queryable>(
        &mut self,
        capture: &tree_sitter::QueryCapture,
        params: &mut BuilderParams,
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
