use super::capabilities::*;
use super::data::*;
use super::display::*;
use super::symbol::*;
use crate::build::Parent;
use crate::document::Document;
use downcast_rs::{impl_downcast, DowncastSync};
use lsp_types::Position;
use lsp_types::Range;
use lsp_types::Url;
use std::fmt::Display;
use std::sync::Arc;

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

    /// Retrieves the text of the symbol based on its range within the provided source code.
    fn get_text<'a>(&self, source_code: &'a [u8]) -> Option<&'a str> {
        let range = self.get_data().get_range();
        // Check if the range is within bounds and valid
        if range.start <= range.end && range.end <= source_code.len() {
            std::str::from_utf8(&source_code[range.start..range.end]).ok()
        } else {
            None
        }
    }

    /// Get the symbol's nearest scope.
    ///
    /// The scope defines the search area for references and completion items.
    fn get_parent_scope(&self) -> Option<DynSymbol> {
        let mut parent = self.get_data().get_parent();
        while let Some(weak) = parent {
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

    /// Checks if the symbol is within the given offset.
    fn is_inside_offset(&self, offset: usize) -> bool {
        let range = self.get_data().get_range();
        range.start <= offset && offset <= range.end
    }

    /// Returns the LSP start position of the symbol.
    fn get_start_position(&self, document: &Document) -> Position {
        document.position_at(self.get_range().start).unwrap()
    }

    fn get_end_position(&self, document: &Document) -> Position {
        document.position_at(self.get_range().end).unwrap()
    }

    /// Returns the LSP range (start and end position) of the symbol.
    fn get_lsp_range(&self, document: &Document) -> Range {
        document.range_at(self.get_range()).unwrap()
    }
}

impl_downcast!(AstSymbol);

impl<T: AstSymbol + ?Sized> GetSymbolData for T {
    fn get_url(&self) -> Arc<Url> {
        self.get_data().get_url()
    }

    fn get_range(&self) -> std::ops::Range<usize> {
        self.get_data().get_range()
    }

    fn get_parent(&self) -> Option<WeakSymbol> {
        self.get_data().get_parent()
    }

    fn set_parent(&mut self, parent: WeakSymbol) {
        self.get_mut_data().set_parent(parent)
    }
}
