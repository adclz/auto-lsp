use std::cell::RefCell;
use std::rc::Rc;

use super::buildable::*;

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
        self.0.as_ref().map(|pending| pending)
    }

    pub fn as_mut(&mut self) -> Option<&mut PendingSymbol> {
        self.0.as_mut().map(|pending| pending)
    }

    pub fn into_inner(self) -> Option<PendingSymbol> {
        self.0
    }

    pub(crate) fn swap(&mut self, other: &mut Self) {
        std::mem::swap(&mut self.0, &mut other.0);
    }
}
