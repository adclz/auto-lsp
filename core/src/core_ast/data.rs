use std::sync::Arc;

use crate::core_build::main_builder::MainBuilder;

use super::core::AstSymbol;
use super::symbol::*;
use lsp_types::Url;

/// Core data of any ast symbol
#[derive(Clone)]
pub struct SymbolData {
    /// The workspace url of the symbol
    pub url: Arc<Url>,
    /// The parent of the symbol
    pub parent: Option<WeakSymbol>,
    /// The comment's byte range in the source code
    pub comment: Option<std::ops::Range<usize>>,
    /// The referrers of the symbol (symbols that refer to this symbol)
    pub referrers: Option<Referrers>,
    /// The target this symbol refers to
    pub target: Option<WeakSymbol>,
    /// The byte range of the symbol in the source code
    pub range: std::ops::Range<usize>,
}

impl SymbolData {
    pub fn new(url: Arc<Url>, range: std::ops::Range<usize>) -> Self {
        Self {
            url,
            parent: None,
            comment: None,
            referrers: None,
            target: None,
            range,
        }
    }
}

/// Trait to read or mutate the core data of an ast symbol
pub trait GetSymbolData {
    /// Get the workspace url of the symbol
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
    /// Note that this only works if the symbol implements [Reference] trait
    fn get_target(&self) -> Option<&WeakSymbol>;
    /// Set the target of the symbol
    ///
    /// Note that this only works if the symbol implements [Reference] trait
    fn set_target_reference(&mut self, target: WeakSymbol);
    /// Reset the target of the symbol
    fn reset_target_reference_reference(&mut self);
    /// Get the referrers of the symbol
    ///
    /// Referrers are symbols that refer to this symbol
    fn get_referrers(&self) -> &Option<Referrers>;
    /// Get a mutable reference to the referrers of the symbol
    ///
    /// Referrers are symbols that refer to this symbol
    fn get_mut_referrers(&mut self) -> &mut Referrers;
}

impl GetSymbolData for SymbolData {
    fn get_url(&self) -> Arc<Url> {
        self.url.clone()
    }

    fn get_range(&self) -> std::ops::Range<usize> {
        self.range.clone()
    }

    fn get_parent(&self) -> Option<WeakSymbol> {
        self.parent.as_ref().map(|p| p.clone())
    }

    fn set_parent(&mut self, parent: WeakSymbol) {
        self.parent = Some(parent);
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

    fn get_referrers(&self) -> &Option<Referrers> {
        &self.referrers
    }

    fn get_mut_referrers(&mut self) -> &mut Referrers {
        self.referrers.get_or_insert_default()
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
}

/// List of weak symbols that refer to this symbol
#[derive(Default, Clone)]
pub struct Referrers(Vec<WeakSymbol>);

/// Trait for managing [`Referrers`]
pub trait ReferrersTrait {
    /// Add a referrer to the symbol list
    fn add_referrer(&mut self, symbol: WeakSymbol);

    /// Clean up any null referrers
    fn clean_null_referrers(&mut self);

    /// Drop any referrers that have an accessor
    ///
    /// If the referrer was not dropped, add it to the unsolved checks field of [`MainBuilder`]
    fn drop_referrers(&mut self, params: &mut MainBuilder);
}

impl<'a> IntoIterator for &'a Referrers {
    type Item = &'a WeakSymbol;
    type IntoIter = std::slice::Iter<'a, WeakSymbol>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<T: AstSymbol + ?Sized> ReferrersTrait for T {
    fn add_referrer(&mut self, symbol: WeakSymbol) {
        self.get_mut_referrers().0.push(symbol);
    }

    fn clean_null_referrers(&mut self) {
        self.get_mut_referrers()
            .0
            .retain(|r| r.get_ptr().weak_count() > 0);
    }

    fn drop_referrers(&mut self, params: &mut MainBuilder) {
        self.get_mut_referrers().0.retain(|r| {
            if let Some(symbol) = r.to_dyn() {
                let read = symbol.read();
                if read.get_target().is_some() {
                    drop(read);
                    symbol.write().reset_target_reference_reference();
                    params.unsolved_references.push(r.clone());
                }
            }
            false
        });
    }
}
