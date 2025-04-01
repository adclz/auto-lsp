use crate::root::Root;
use std::{
    fmt::Debug,
    sync::{Arc, Weak},
};

use super::core::AstSymbol;
use parking_lot::RwLock;

/// Generic thread-safe wrapper around [AstSymbol] using [Arc] and [parking_lot::RwLock]
///
/// Provides methods to read and write to the underlying [AstSymbol]
///
/// [`Symbol<T>`] also provides methods to convert to [DynSymbol] and [WeakSymbol]
#[derive(Clone)]
pub struct Symbol<T: AstSymbol>(Arc<RwLock<T>>);

impl<T: AstSymbol> Symbol<T> {
    pub fn read(&self) -> parking_lot::RwLockReadGuard<T> {
        self.0.read()
    }

    pub fn write(&self) -> parking_lot::RwLockWriteGuard<T> {
        self.0.write()
    }

    /// Convert the [Symbol] to a [DynSymbol]
    pub fn to_dyn(&self) -> DynSymbol {
        DynSymbol::from_symbol(self)
    }

    /// Convert the [Symbol] to a [WeakSymbol]
    pub fn to_weak(&self) -> WeakSymbol {
        WeakSymbol::from_symbol(self)
    }

    /// Create a new [Symbol]
    ///
    /// Inject itself as the parent of the symbol
    ///
    /// If the symbol is a reference ([`super::capabilities::Reference`]), add it to the unsolved references list
    ///
    /// If the symbol requires checking ([`super::capabilities::Check`]), add it to the unsolved checks list
    pub fn new_and_check(symbol: T, root: &mut Root) -> Self {
        let symbol = Symbol::new(symbol);
        symbol.write().inject_parent(symbol.to_weak());
        symbol
    }

    pub(crate) fn new(symbol: T) -> Self {
        Self(Arc::new(RwLock::new(symbol)))
    }

    pub(crate) fn get_ptr(&self) -> &Arc<RwLock<T>> {
        &self.0
    }
}

/// Generic Thread-safe wrapper around an [AstSymbol] trait object using [Arc] and [parking_lot::RwLock]
#[derive(Clone)]
pub struct DynSymbol(Arc<RwLock<dyn AstSymbol>>);

impl DynSymbol {
    pub fn new(symbol: impl AstSymbol) -> Self {
        Self(Arc::new(parking_lot::RwLock::new(symbol)))
    }

    /// Create a trait object [DynSymbol] from a concrete [Symbol]
    pub fn from_symbol<T: AstSymbol>(symbol: &Symbol<T>) -> Self {
        Self(symbol.0.clone())
    }

    pub fn read(&self) -> parking_lot::RwLockReadGuard<dyn AstSymbol> {
        self.0.read()
    }

    pub fn write(&self) -> parking_lot::RwLockWriteGuard<dyn AstSymbol> {
        self.0.write()
    }

    /// Downgrade a [DynSymbol] to a [WeakSymbol]
    pub(crate) fn to_weak(&self) -> WeakSymbol {
        WeakSymbol::new(self)
    }
}

impl Debug for DynSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Symbol {:?}", self.read().get_range())
    }
}

/// Generic Thread-safe wrapper around a [Weak] reference to an [AstSymbol] using [Weak] and [parking_lot::RwLock]
///
/// Must be upgraded to a [DynSymbol] before use
#[derive(Debug, Clone)]
pub struct WeakSymbol(Weak<RwLock<dyn AstSymbol>>);

impl WeakSymbol {
    pub fn new(symbol: &DynSymbol) -> Self {
        Self(Arc::downgrade(&symbol.0))
    }

    pub fn from_symbol<T: AstSymbol>(symbol: &Symbol<T>) -> Self {
        Self(Arc::downgrade(symbol.get_ptr()) as _)
    }

    /// Upgrade the [WeakSymbol] to a [DynSymbol]
    pub fn to_dyn(&self) -> Option<DynSymbol> {
        self.0.upgrade().map(DynSymbol)
    }

    pub(crate) fn get_ptr(&self) -> &Weak<RwLock<dyn AstSymbol>> {
        &self.0
    }
}
