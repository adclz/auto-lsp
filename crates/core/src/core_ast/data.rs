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

use std::sync::Arc;

use super::symbol::*;
use lsp_types::Url;

/// Core data of any ast symbol
#[derive(Clone)]
pub struct SymbolData {
    /// The parent of the symbol
    pub parent: Option<WeakSymbol>,
    /// The byte range of the symbol in the source code
    pub range: std::ops::Range<usize>,
}

impl SymbolData {
    pub fn new(range: std::ops::Range<usize>) -> Self {
        Self {
            parent: None,
            range,
        }
    }
}

/// Trait to read or mutate the core data of an ast symbol
pub trait GetSymbolData {
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
