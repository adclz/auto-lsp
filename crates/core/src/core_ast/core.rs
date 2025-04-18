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

use super::capabilities::*;
use super::data::*;
use super::display::*;
use super::symbol::*;
use crate::build::Parent;
use crate::document::Document;
use crate::errors::PositionError;
use downcast_rs::{impl_downcast, DowncastSync};
use lsp_types::Position;
use lsp_types::Range;
use std::fmt::Display;

/// Core functionality of an AST symbol
///
/// Any struct or enum generated by the `seq` or `choice` macro implements this trait.
///
/// This trait supports downcasting via [downcast_rs].
pub trait AstSymbol:
    DowncastSync
    + Send
    + Sync
    // lsp
    + GetGoToDeclaration
    + GetGoToDefinition
    + GetHover
    + BuildDocumentSymbols
    + BuildCodeLenses
    + BuildCompletionItems
    + BuildTriggeredCompletionItems
    + BuildInlayHints
    + BuildSemanticTokens
    + BuildCodeActions
    // special
    + Traverse
    + Scope
    + GetSymbolData
    + Parent
    + Display
    + IndentedDisplay
    {
    /// Retrieves the data of the symbol.
    fn get_data(&self) -> &SymbolData;

    /// Retrieves the mutable data of the symbol.
    fn get_mut_data(&mut self) -> &mut SymbolData;

    #[inline]
    /// Retrieves the text of the symbol based on its range within the provided source code.
    fn get_text<'a>(&self, source_code: &'a [u8]) -> Result<&'a str, PositionError> {
        let range = self.get_data().get_range();
        match source_code.get(range.start..range.end) {
            Some(text) => match std::str::from_utf8(text) {
                Ok(text) => Ok(text),
                Err(utf8_error) => Err(PositionError::UTF8Error  {
                    range,
                    utf8_error
                }),
            }
            None => Err(PositionError::WrongTextRange { range })
        }
    }

    /// Get the symbol's nearest scope.
    ///
    /// The scope defines the search area for references and completion items.
    fn get_parent_scope(&self) -> Option<DynSymbol> {
        let mut parent = self.get_data().get_parent();
        while let Some(weak) = parent {
            #[allow(clippy::all)]
            let symbol: DynSymbol = match weak.into() {
                Some(symbol) => symbol,
                None => return None
            };

            let read = symbol.read();
            if symbol.read().is_scope() {
                return Some(symbol.clone());
            }
                parent = read.get_parent();
            }
        None
    }

    #[inline]
    /// Checks if the symbol is within the given offset.
    fn is_inside_offset(&self, offset: usize) -> bool {
        let range = self.get_data().get_range();
        range.start <= offset && offset <= range.end
    }

    /// Returns the LSP start position of the symbol.
    fn get_start_position(&self, document: &Document) -> Result<Position, PositionError> {
        document.position_at(self.get_range().start)
    }

    fn get_end_position(&self, document: &Document) -> Result<Position, PositionError> {
        document.position_at(self.get_range().end)
    }

    /// Returns the LSP range (start and end position) of the symbol.
    fn get_lsp_range(&self, document: &Document) -> Result<Range, PositionError> {
        document.range_at(self.get_range())
    }
}

impl_downcast!(AstSymbol);
