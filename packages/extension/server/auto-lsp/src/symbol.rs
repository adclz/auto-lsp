use crate::semantic_tokens::SemanticTokensBuilder;
use downcast_rs::{impl_downcast, Downcast};
use lsp_textdocument::FullTextDocument;
use lsp_types::{CompletionItem, Diagnostic, DocumentSymbol, Position, Range, Url};
use parking_lot::RwLock;
use std::{
    collections::HashSet,
    sync::{Arc, Weak},
};

use super::workspace::WorkspaceContext;

pub trait AstSymbol:
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
    + Locator
    + Parent
    + CheckDuplicate
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

    fn get_parent_scope(&self) -> Option<DynSymbol> {
        let mut parent = self.get_parent();
        while let Some(weak) = parent {
            let symbol = match weak.to_dyn() {
                Some(weak) => weak,
                None => return None,
            };
            let read = symbol.read();
            if read.is_scope() {
                return Some(symbol.clone());
            }
            parent = read.get_parent();
        }
        None
    }

    // Accessibility

    fn is_inside_offset(&self, offset: usize) -> bool {
        let range = self.get_range();
        range.start_byte <= offset && offset <= range.end_byte
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

impl_downcast!(AstSymbol);

#[derive(Clone)]
pub struct Symbol<T: AstSymbol>(Arc<RwLock<T>>);

impl<T: AstSymbol> Symbol<T> {
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
pub struct DynSymbol(Arc<RwLock<dyn AstSymbol>>);

impl DynSymbol {
    pub fn new(symbol: impl AstSymbol) -> Self {
        Self(Arc::new(parking_lot::RwLock::new(symbol)))
    }

    pub fn from_symbol<T: AstSymbol>(symbol: &Symbol<T>) -> Self {
        Self(symbol.0.clone())
    }

    pub fn get_arc(&self) -> &Arc<RwLock<dyn AstSymbol>> {
        &self.0
    }

    pub fn read(&self) -> parking_lot::RwLockReadGuard<dyn AstSymbol> {
        self.0.read()
    }

    pub fn write(&self) -> parking_lot::RwLockWriteGuard<dyn AstSymbol> {
        self.0.write()
    }

    pub fn to_weak(&self) -> WeakSymbol {
        WeakSymbol::new(self)
    }

    pub fn downcast<T: AstSymbol + Clone>(&self) -> Option<Symbol<T>> {
        Some(Symbol::new(self.0.read().downcast_ref::<T>().cloned()?))
    }
}

#[derive(Clone)]
pub struct WeakSymbol(Weak<RwLock<dyn AstSymbol>>);

impl WeakSymbol {
    pub fn new(symbol: &DynSymbol) -> Self {
        Self(Arc::downgrade(&symbol.0))
    }

    pub fn from_symbol<T: AstSymbol>(symbol: &Symbol<T>) -> Self {
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
    fn is_accessor(&self) -> bool;
    fn set_accessor(&mut self, accessor: WeakSymbol);
}

pub trait Accessor: IsAccessor {
    fn find(
        &self,
        doc: &FullTextDocument,
        ctx: &dyn WorkspaceContext,
    ) -> Result<Option<WeakSymbol>, Diagnostic>;
}

pub trait Locator {
    fn find_at_offset(&self, offset: usize) -> Option<DynSymbol>;
}

impl Locator for DynSymbol {
    fn find_at_offset(&self, offset: usize) -> Option<DynSymbol> {
        let symbol = self.read();
        if symbol.is_inside_offset(offset) {
            match symbol.find_at_offset(offset) {
                Some(symbol) => return Some(symbol),
                None => return Some(self.clone()),
            }
        }
        None
    }
}

impl<T: AstSymbol> Locator for Symbol<T> {
    fn find_at_offset(&self, offset: usize) -> Option<DynSymbol> {
        let symbol = self.read();
        if symbol.is_inside_offset(offset) {
            match symbol.find_at_offset(offset) {
                Some(symbol) => return Some(symbol),
                None => return Some(self.to_dyn()),
            };
        }
        None
    }
}

impl<T: AstSymbol> Locator for Option<Symbol<T>> {
    fn find_at_offset(&self, offset: usize) -> Option<DynSymbol> {
        match self.as_ref() {
            Some(symbol) => symbol.find_at_offset(offset),
            None => None,
        }
    }
}

impl<T: Locator> Locator for Vec<T> {
    fn find_at_offset(&self, offset: usize) -> Option<DynSymbol> {
        self.iter().enumerate().find_map(|(i, symbol)| {
            let h = symbol.find_at_offset(offset);
            if let Some(a) = &h {
                let range = a.read().get_range();
                eprintln!(
                    "Found at index {}: offset {:?}, between {} and {}",
                    i, offset, range.start_byte, range.end_byte
                )
            };
            h
        })
    }
}

pub trait Parent {
    fn inject_parent(&mut self, parent: WeakSymbol);
}

impl<T: AstSymbol> Parent for Symbol<T> {
    fn inject_parent(&mut self, parent: WeakSymbol) {
        self.write().set_parent(parent);
    }
}

impl Parent for DynSymbol {
    fn inject_parent(&mut self, parent: WeakSymbol) {
        self.write().set_parent(parent);
    }
}

impl<T: AstSymbol> Parent for Option<Symbol<T>> {
    fn inject_parent(&mut self, parent: WeakSymbol) {
        if let Some(symbol) = self.as_mut() {
            symbol.write().set_parent(parent);
        }
    }
}

impl<T: AstSymbol> Parent for Vec<Symbol<T>> {
    fn inject_parent(&mut self, parent: WeakSymbol) {
        for symbol in self.iter_mut() {
            symbol.write().set_parent(parent.clone());
        }
    }
}

pub trait CheckDuplicate {
    fn must_check(&self) -> bool;
    fn check(&self, doc: &FullTextDocument, diagnostics: &mut Vec<Diagnostic>);
}

pub trait CheckDuplicateVec<'a, T: AstSymbol> {
    fn check<F>(
        &self,
        f: F,
        doc: &'a FullTextDocument,
        hash_set: &mut HashSet<&'a str>,
        diagnostics: &mut Vec<Diagnostic>,
    ) where
        F: for<'b> Fn(&T, &'b [u8]) -> &'b str;
}

impl<'a, T: AstSymbol> CheckDuplicateVec<'a, T> for [Symbol<T>] {
    fn check<F>(
        &self,
        f: F,
        doc: &'a FullTextDocument,
        hash_set: &mut HashSet<&'a str>,
        diagnostics: &mut Vec<Diagnostic>,
    ) where
        F: for<'b> Fn(&T, &'b [u8]) -> &'b str,
    {
        if self.is_empty() {
            return;
        }
        let source_code = doc.get_content(None).as_bytes();

        self.iter().for_each(|item| {
            let key = f(&item.read(), source_code);
            if !hash_set.insert(key) {
                diagnostics.push(Diagnostic::new(
                    item.read().get_lsp_range(doc),
                    None,
                    None,
                    None,
                    format!("Duplicate {}", key),
                    None,
                    None,
                ));
            }
        });
    }
}
