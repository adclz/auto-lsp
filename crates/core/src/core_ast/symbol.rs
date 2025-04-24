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

use std::{
    fmt::Debug,
    ops::Deref,
    sync::{Arc, Weak},
};

use super::core::AstSymbol;

/// Generic thread-safe wrapper around [AstSymbol] using [Arc] and [parking_lot::RwLock]
///
/// Provides methods to read and write to the underlying [AstSymbol]
///
/// [`Symbol<T>`] also provides methods to convert to [DynSymbol] and [WeakSymbol]
#[derive(Clone)]
pub struct Symbol<T: AstSymbol>(pub(crate) Arc<T>);

impl<T: AstSymbol> Deref for Symbol<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T: AstSymbol> AsRef<T> for Symbol<T> {
    fn as_ref(&self) -> &T {
        self.0.as_ref()
    }
}

/// Generic Thread-safe wrapper around an [AstSymbol] trait object using [Arc] and [parking_lot::RwLock]
#[derive(Clone)]
pub struct DynSymbol(pub(crate) Arc<dyn AstSymbol>);

impl DynSymbol {
    pub fn new(symbol: impl AstSymbol) -> Self {
        Self(Arc::new(symbol))
    }
}

impl Deref for DynSymbol {
    type Target = dyn AstSymbol;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl Debug for DynSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Symbol")
            .field("range", &self.0.get_range())
            .finish()
    }
}

impl std::fmt::Display for DynSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Generic Thread-safe wrapper around a [Weak] reference to an [AstSymbol] using [Weak] and [parking_lot::RwLock]
///
/// Must be upgraded to a [DynSymbol] before use
#[derive(Debug, Clone)]
pub struct WeakSymbol(pub(crate) Weak<dyn AstSymbol>);

impl<T: AstSymbol> From<T> for Symbol<T> {
    fn from(value: T) -> Self {
        Self(Arc::new(value))
    }
}

impl<T: AstSymbol> From<&Symbol<T>> for DynSymbol {
    fn from(value: &Symbol<T>) -> Self {
        Self(value.0.clone())
    }
}

impl<T: AstSymbol> From<Symbol<T>> for DynSymbol {
    fn from(value: Symbol<T>) -> Self {
        Self(value.0.clone())
    }
}

impl<T: AstSymbol> From<&Symbol<T>> for WeakSymbol {
    fn from(value: &Symbol<T>) -> Self {
        Self(Arc::downgrade(&value.0) as _)
    }
}

impl<T: AstSymbol> From<Symbol<T>> for WeakSymbol {
    fn from(value: Symbol<T>) -> Self {
        Self(Arc::downgrade(&value.0) as _)
    }
}

impl From<WeakSymbol> for Option<DynSymbol> {
    fn from(value: WeakSymbol) -> Self {
        value.0.upgrade().map(DynSymbol)
    }
}

impl From<&DynSymbol> for WeakSymbol {
    fn from(value: &DynSymbol) -> Self {
        Self(Arc::downgrade(&value.0))
    }
}
