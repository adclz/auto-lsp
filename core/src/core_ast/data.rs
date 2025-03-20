use std::sync::Arc;

use crate::root::Root;

use super::core::AstSymbol;
use super::symbol::*;
use lsp_types::Url;

/// Core data of any ast symbol
#[derive(Clone)]
pub struct SymbolData {
    /// The root url of the symbol
    pub url: Arc<Url>,
    /// The parent of the symbol
    pub parent: Option<WeakSymbol>,
    /// The comment's byte range in the source code
    pub comment: Option<std::ops::Range<usize>>,
    /// The target this symbol refers to
    pub target: Option<WeakSymbol>,
    /// The byte range of the symbol in the source code
    pub range: std::ops::Range<usize>,
    /// Whether the symbol is being checked for errors
    pub check_pending: bool,
}

impl SymbolData {
    pub fn new(url: Arc<Url>, range: std::ops::Range<usize>) -> Self {
        Self {
            url,
            parent: None,
            comment: None,
            target: None,
            range,
            check_pending: false,
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
    /// Get the comment of the symbol (if any)
    ///
    /// Requires the source code to be passed since symobls only store byte ranges
    fn get_comment<'a>(&self, source_code: &'a [u8]) -> Option<&'a str>;
    /// Set the comment of the symbol, where range is the byte range of the comment's text location
    fn set_comment(&mut self, range: Option<std::ops::Range<usize>>);
    /// The target this symbol refers to
    ///
    /// Note that this only works if the symbol implements [`super::capabilities::Reference`] trait
    fn get_target(&self) -> Option<&WeakSymbol>;
    /// Set the target of the symbol
    ///
    /// Note that this only works if the symbol implements [`super::capabilities::Reference`] trait
    fn set_target_reference(&mut self, target: WeakSymbol);
    /// Reset the target of the symbol
    fn reset_target_reference_reference(&mut self);

    /// Get whether the symbol has been checked for errors
    fn has_check_pending(&self) -> bool;
    /// Set whether the symbol has been checked for errors
    fn update_check_pending(&mut self, unchecked: bool);
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

    fn get_comment<'a>(&self, source_code: &'a [u8]) -> Option<&'a str> {
        match self.comment {
            Some(ref range) => {
                // Check if the range is within bounds and valid
                if range.start <= range.end && range.end <= source_code.len() {
                    std::str::from_utf8(&source_code[range.start..range.end]).ok()
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn set_comment(&mut self, range: Option<std::ops::Range<usize>>) {
        self.comment = range;
    }

    fn get_target(&self) -> Option<&WeakSymbol> {
        self.target.as_ref()
    }

    fn set_target_reference(&mut self, target: WeakSymbol) {
        self.target = Some(target);
    }

    fn reset_target_reference_reference(&mut self) {
        self.target = None;
    }

    fn has_check_pending(&self) -> bool {
        self.check_pending
    }

    fn update_check_pending(&mut self, unchecked: bool) {
        self.check_pending = unchecked;
    }
}
