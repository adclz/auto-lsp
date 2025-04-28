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

use crate::ast::AstSymbol;

/// Core data of any ast symbol
#[derive(Clone)]
pub struct SymbolData {
    /// The parent id of this symbol
    /// `None` if this is the root symbol
    pub parent: Option<usize>,
    /// The byte range of the symbol in the source code
    pub range: std::ops::Range<usize>,
    /// The id of the symbol
    pub id: usize,
}

impl SymbolData {
    pub fn new(range: std::ops::Range<usize>, id: usize) -> Self {
        Self {
            parent: None,
            range,
            id,
        }
    }
}

/// Trait to read or mutate the core data of an ast symbol
pub trait GetSymbolData {
    /// Get the range of the symbol in the source code
    ///
    /// This is a byte range, not the line and column range
    fn get_range(&self) -> std::ops::Range<usize>;

    /// Get the parent of the symbol
    ///
    /// `None` if this is the root symbol
    fn get_parent<'a>(&self, nodes: &'a [Arc<dyn AstSymbol>]) -> Option<&'a Arc<dyn AstSymbol>>;

    /// Get the id of the symbol
    fn get_id(&self) -> usize;
}

impl GetSymbolData for SymbolData {
    fn get_range(&self) -> std::ops::Range<usize> {
        self.range.clone()
    }

    fn get_parent<'a>(&self, nodes: &'a [Arc<dyn AstSymbol>]) -> Option<&'a Arc<dyn AstSymbol>> {
        self.parent.and_then(|id| nodes.get(id))
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

impl<T: AstSymbol + ?Sized> GetSymbolData for T {
    #[inline]
    fn get_range(&self) -> std::ops::Range<usize> {
        self.get_data().get_range()
    }

    #[inline]
    fn get_parent<'a>(&self, nodes: &'a [Arc<dyn AstSymbol>]) -> Option<&'a Arc<dyn AstSymbol>> {
        self.get_data().get_parent(nodes)
    }

    #[inline]
    fn get_id(&self) -> usize {
        self.get_data().get_id()
    }
}
