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

use std::cell::RefCell;
use std::rc::Rc;

use super::buildable::*;

// [`PendingSymbol`] and [`MaybePendingSymbol`] represent symbols being built during the construction process.
//
// These symbols exist only temporarily while building and are not part of the final AST.

/// A wrapper for a shared, mutable [`Buildable`] object.
#[derive(Clone)]
pub struct PendingSymbol(pub(crate) Rc<RefCell<dyn Buildable>>);

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
pub struct MaybePendingSymbol(pub(crate) Option<PendingSymbol>);

impl MaybePendingSymbol {
    pub fn none() -> Self {
        MaybePendingSymbol(None)
    }

    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    pub(crate) fn swap(&mut self, other: &mut Self) {
        std::mem::swap(&mut self.0, &mut other.0);
    }
}

impl<T: Buildable> From<T> for PendingSymbol {
    fn from(value: T) -> Self {
        PendingSymbol::new(value)
    }
}

impl<T: Buildable> From<T> for MaybePendingSymbol {
    fn from(value: T) -> Self {
        Self(Some(PendingSymbol::new(value)))
    }
}

impl AsRef<Option<PendingSymbol>> for MaybePendingSymbol {
    fn as_ref(&self) -> &Option<PendingSymbol> {
        &self.0
    }
}

impl From<PendingSymbol> for MaybePendingSymbol {
    fn from(value: PendingSymbol) -> Self {
        Self(Some(value))
    }
}
