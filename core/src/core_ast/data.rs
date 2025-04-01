use std::sync::Arc;

use super::symbol::*;
use lsp_types::Url;

/// Core data of any ast symbol
#[derive(Clone)]
pub struct SymbolData {
    /// The root url of the symbol
    pub url: Arc<Url>,
    /// The parent of the symbol
    pub parent: Option<WeakSymbol>,
    /// The byte range of the symbol in the source code
    pub range: std::ops::Range<usize>,
}

impl SymbolData {
    pub fn new(url: Arc<Url>, range: std::ops::Range<usize>) -> Self {
        Self {
            url,
            parent: None,
            range,
        }
    }
}

/// Trait to read or mutate the core data of an ast symbol
pub trait GetSymbolData {
    /// Get the root url of the symbol
    fn get_url(&self) -> Arc<Url>;
    /// Get the range of the symbol in the source code
    ///
    /// This is a byte range, not the line and column range
    fn get_range(&self) -> std::ops::Range<usize>;
    /// Get the parent of the symbol (if any)
    fn get_parent(&self) -> Option<WeakSymbol>;
    /// Set the parent of the symbol
    fn set_parent(&mut self, parent: WeakSymbol);
}

impl GetSymbolData for SymbolData {
    fn get_url(&self) -> Arc<Url> {
        self.url.clone()
    }

    fn get_range(&self) -> std::ops::Range<usize> {
        self.range.clone()
    }

    fn get_parent(&self) -> Option<WeakSymbol> {
        self.parent.clone()
    }

    fn set_parent(&mut self, parent: WeakSymbol) {
        self.parent = Some(parent);
    }
}
