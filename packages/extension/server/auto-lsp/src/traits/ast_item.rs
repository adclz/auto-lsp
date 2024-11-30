use crate::builders::semantic_tokens::SemanticTokensBuilder;
use downcast_rs::{impl_downcast, Downcast};
use lsp_textdocument::FullTextDocument;
use lsp_types::{CompletionItem, Diagnostic, DocumentSymbol, Position, Range, Url};
use parking_lot::RwLock;
use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use super::workspace::WorkspaceContext;

pub trait AstItem:
    Downcast
    + Send
    + Sync
    + DocumentSymbols
    + HoverInfo
    + SemanticTokens
    + InlayHints
    + CodeLens
    + CompletionItems
    + Scope
    + Accessor
{
    fn get_url(&self) -> Arc<Url>;
    fn get_range(&self) -> tree_sitter::Range;
    fn edit_range(&mut self, shift: i32) {
        let mut range = self.get_range();
        range.start_byte += shift as usize;
        range.end_byte += shift as usize;
    }

    fn get_size(&self) -> usize {
        let range = self.get_range();
        range.end_byte - range.start_byte
    }

    fn get_text<'a>(&self, source_code: &'a [u8]) -> &'a str {
        let range = self.get_range();
        std::str::from_utf8(&source_code[range.start_byte..range.end_byte]).unwrap()
    }

    fn get_parent(&self) -> Option<WeakSymbol>;
    fn set_parent(&mut self, parent: WeakSymbol);
    fn inject_parent(&mut self, parent: WeakSymbol);

    fn find_at_offset(&self, offset: &usize) -> Option<DynSymbol>;

    // Accessibility

    fn is_inside_offset(&self, offset: &usize) -> bool {
        let range = self.get_range();
        range.start_byte <= *offset && *offset <= range.end_byte
    }

    fn is_same_text(&mut self, source_code: &[u8], range: &tree_sitter::Range) -> bool {
        self.get_text(source_code)
            == std::str::from_utf8(&source_code[range.start_byte..range.end_byte]).unwrap()
    }
    // LSP

    fn get_start_position(&self, doc: &FullTextDocument) -> Position {
        doc.position_at(self.get_range().start_byte as u32)
    }

    fn get_end_position(&self, doc: &FullTextDocument) -> Position {
        doc.position_at(self.get_range().end_byte as u32)
    }

    fn get_lsp_range(&self, doc: &FullTextDocument) -> Range {
        let start = self.get_start_position(doc);
        let end = self.get_end_position(doc);
        lsp_types::Range { start, end }
    }
}

impl_downcast!(AstItem);

#[derive(Clone)]
pub struct Symbol<T: AstItem>(Arc<RwLock<T>>);

impl<T: AstItem> Symbol<T> {
    pub fn new(symbol: T) -> Self {
        Self(Arc::new(RwLock::new(symbol)))
    }

    pub fn read(&self) -> parking_lot::RwLockReadGuard<T> {
        self.0.read()
    }

    pub fn write(&self) -> parking_lot::RwLockWriteGuard<T> {
        self.0.write()
    }

    pub fn to_dyn(&self) -> DynSymbol {
        DynSymbol::from_symbol(&self)
    }

    pub fn to_weak(&self) -> WeakSymbol {
        WeakSymbol::from_symbol(self)
    }
}

#[derive(Clone)]
pub struct DynSymbol(Arc<RwLock<dyn AstItem>>);

impl DynSymbol {
    pub fn new(symbol: impl AstItem) -> Self {
        Self(Arc::new(parking_lot::RwLock::new(symbol)))
    }

    pub fn from_symbol<T: AstItem>(symbol: &Symbol<T>) -> Self {
        Self(symbol.0.clone())
    }

    pub fn get_arc(&self) -> &Arc<RwLock<dyn AstItem>> {
        &self.0
    }

    pub fn read(&self) -> parking_lot::RwLockReadGuard<dyn AstItem> {
        self.0.read()
    }

    pub fn write(&self) -> parking_lot::RwLockWriteGuard<dyn AstItem> {
        self.0.write()
    }

    pub fn to_weak(&self) -> WeakSymbol {
        WeakSymbol::new(self)
    }
}

#[derive(Clone)]
pub struct WeakSymbol(Weak<RwLock<dyn AstItem>>);

impl WeakSymbol {
    pub fn new(symbol: &DynSymbol) -> Self {
        Self(Arc::downgrade(&symbol.0))
    }

    pub fn from_symbol<T: AstItem>(symbol: &Symbol<T>) -> Self {
        Self(Arc::downgrade(&symbol.0) as _)
    }

    pub fn to_dyn(&self) -> Option<DynSymbol> {
        self.0.upgrade().map(|arc| DynSymbol(arc))
    }
}

pub trait Scope {
    fn is_scope(&self) -> bool;
    fn get_scope_range(&self) -> Vec<[usize; 2]>;
}

pub trait DocumentSymbols {
    fn get_document_symbols(&self, doc: &FullTextDocument) -> Option<DocumentSymbol>;
}

pub trait HoverInfo {
    fn get_hover(&self, doc: &FullTextDocument) -> Option<lsp_types::Hover>;
}

pub trait SemanticTokens {
    fn build_semantic_tokens(&self, builder: &mut SemanticTokensBuilder);
}

pub trait InlayHints {
    fn build_inlay_hint(&self, doc: &FullTextDocument, acc: &mut Vec<lsp_types::InlayHint>);
}

pub trait CodeLens {
    fn build_code_lens(&self, acc: &mut Vec<lsp_types::CodeLens>);
}

pub trait CompletionItems {
    fn build_completion_items(&self, acc: &mut Vec<CompletionItem>, doc: &FullTextDocument);
}

pub trait IsAccessor {
    fn is_accessor(&self) -> &'static bool;
}

pub trait Accessor: IsAccessor {
    fn find(&self, doc: &FullTextDocument, ctx: &dyn WorkspaceContext) -> Result<(), Diagnostic>;
}

pub trait OffsetLocator {
    fn try_locate_at_offset(&self, offset: usize) -> Option<DynSymbol>;
}

impl OffsetLocator for DynSymbol {
    fn try_locate_at_offset(&self, offset: usize) -> Option<DynSymbol> {
        let symbol = self.read();
        if symbol.is_inside_offset(&offset) {
            match symbol.find_at_offset(&offset) {
                Some(symbol) => return Some(symbol),
                None => return Some(self.clone()),
            }
        }
        None
    }
}

impl<T: AstItem> OffsetLocator for Symbol<T> {
    fn try_locate_at_offset(&self, offset: usize) -> Option<DynSymbol> {
        let symbol = self.read();
        if symbol.is_inside_offset(&offset) {
            match symbol.find_at_offset(&offset) {
                Some(symbol) => return Some(symbol),
                None => return Some(self.to_dyn()),
            };
        }
        None
    }
}

impl<T: AstItem> OffsetLocator for Option<Symbol<T>> {
    fn try_locate_at_offset(&self, offset: usize) -> Option<DynSymbol> {
        let symbol = match self.as_ref() {
            Some(symbol) => symbol,
            None => return None,
        };
        symbol.try_locate_at_offset(offset)
    }
}

impl<T: OffsetLocator> OffsetLocator for Vec<T> {
    fn try_locate_at_offset(&self, offset: usize) -> Option<DynSymbol> {
        self.iter()
            .find_map(|symbol| symbol.try_locate_at_offset(offset))
    }
}

impl<T: OffsetLocator> OffsetLocator for HashMap<String, T> {
    fn try_locate_at_offset(&self, offset: usize) -> Option<DynSymbol> {
        self.values()
            .find_map(|symbol| symbol.try_locate_at_offset(offset))
    }
}

pub trait ParentInject {
    fn inject(&mut self, parent: WeakSymbol);
}

impl<T: AstItem> ParentInject for Symbol<T> {
    fn inject(&mut self, parent: WeakSymbol) {
        self.write().set_parent(parent);
    }
}

impl ParentInject for DynSymbol {
    fn inject(&mut self, parent: WeakSymbol) {
        self.write().set_parent(parent);
    }
}

impl<T: AstItem> ParentInject for Option<Symbol<T>> {
    fn inject(&mut self, parent: WeakSymbol) {
        if let Some(symbol) = self.as_mut() {
            symbol.write().set_parent(parent);
        }
    }
}

impl<T: AstItem> ParentInject for Vec<Symbol<T>> {
    fn inject(&mut self, parent: WeakSymbol) {
        for symbol in self.iter_mut() {
            symbol.write().set_parent(parent.clone());
        }
    }
}

impl<T: AstItem> ParentInject for HashMap<String, Symbol<T>> {
    fn inject(&mut self, parent: WeakSymbol) {
        for symbol in self.values_mut() {
            symbol.write().set_parent(parent.clone());
        }
    }
}
