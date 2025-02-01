use std::cell::RefCell;
use std::rc::Rc;

use super::buildable::*;

/// [`PendingSymbol`] and [`MaybePendingSymbol`] represent symbols being built during the construction process.
///
/// These symbols exist only temporarily while building and are not part of the final AST.

/// A wrapper for a shared, mutable [`Buildable`] object.
#[derive(Clone)]
pub struct PendingSymbol(Rc<RefCell<dyn Buildable>>);

impl PendingSymbol {
    pub fn new(builder: impl Buildable) -> Self {
        PendingSymbol(Rc::new(RefCell::new(builder)))
    }

    pub fn get_query_index(&self) -> usize {
        self.0.borrow().get_query_index()
    }

    pub fn get_rc(&self) -> &Rc<RefCell<dyn Buildable>> {
        &self.0
    }
}

/// A wrapper that optionally holds a [`PendingSymbol`].
///
/// All [`Buildable`] objects store their fields as [`MaybePendingSymbol`]s.
///
/// When a field needs to be converted into a symbol, [`MaybePendingSymbol`] is converted to [`PendingSymbol`].
///
/// If the field is optional or a vector and the symbol is `None`, the conversion is skipped.  
/// Otherwise, the conversion will return a diagnostic.
#[derive(Clone)]
pub struct MaybePendingSymbol(Option<PendingSymbol>);

impl MaybePendingSymbol {
    pub fn none() -> Self {
        MaybePendingSymbol(None)
    }

    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    pub fn new(builder: impl Buildable) -> Self {
        MaybePendingSymbol(Some(PendingSymbol::new(builder)))
    }

    pub fn from_pending(pending: PendingSymbol) -> Self {
        MaybePendingSymbol(Some(pending))
    }

    pub fn as_ref(&self) -> Option<&PendingSymbol> {
        self.0.as_ref()
    }

    pub fn as_mut(&mut self) -> Option<&mut PendingSymbol> {
        self.0.as_mut()
    }

    pub fn into_inner(self) -> Option<PendingSymbol> {
        self.0
    }

    pub(crate) fn swap(&mut self, other: &mut Self) {
        std::mem::swap(&mut self.0, &mut other.0);
    }
}
