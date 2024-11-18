use crate::builders::semantic_tokens::SemanticTokensBuilder;
use downcast_rs::{impl_downcast, Downcast};
use lsp_types::{CompletionItem, DocumentSymbol, Url};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock, Weak};

use super::ast_item_builder::AstItemBuilder;

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

    fn get_parent(&self) -> Option<Weak<RwLock<dyn AstItem>>>;
    fn set_parent(&mut self, parent: Weak<RwLock<dyn AstItem>>);
    fn inject_parent(&mut self, parent: Weak<RwLock<dyn AstItem>>);
    fn get_highest_parent(&self) -> Weak<RwLock<dyn AstItem>> {
        let mut parent = self.get_parent();
        while let Some(p) = parent {
            parent = p.upgrade().unwrap().get_parent();
        }
        parent.unwrap()
    }

    fn is_scope(&self) -> bool {
        false
    }

    fn get_scope_range(&self) -> [usize; 2] {
        [0, 0]
    }

    fn find_at_offset(&self, offset: &usize) -> Option<Arc<RwLock<dyn AstItem>>>;

    // Accessibility

    fn is_inside_offset(&self, offset: &usize) -> bool {
        let range = self.get_range();
        range.start_byte <= *offset && *offset <= range.end_byte
    }

    fn is_same_text(&mut self, source_code: &[u8], range: &tree_sitter::Range) -> bool {
        self.get_text(source_code)
            == std::str::from_utf8(&source_code[range.start_byte..range.end_byte]).unwrap()
    }

    fn accept_reference(&self, _other: &dyn AstItem) -> bool {
        false
    }

    // Memory

    fn swap_at_offset(&mut self, offset: &usize, item: &Rc<RefCell<dyn AstItemBuilder>>);

    // LSP

    fn get_start_position(&self, doc: &lsp_textdocument::FullTextDocument) -> lsp_types::Position {
        doc.position_at(self.get_range().start_byte as u32)
    }

    fn get_end_position(&self, doc: &lsp_textdocument::FullTextDocument) -> lsp_types::Position {
        doc.position_at(self.get_range().end_byte as u32)
    }

    fn get_lsp_range(&self, doc: &lsp_textdocument::FullTextDocument) -> lsp_types::Range {
        let start = self.get_start_position(doc);
        let end = self.get_end_position(doc);
        lsp_types::Range { start, end }
    }
}

impl_downcast!(AstItem);

pub trait DocumentSymbols {
    fn get_document_symbols(
        &self,
        doc: &lsp_textdocument::FullTextDocument,
    ) -> Option<DocumentSymbol>;
}

pub trait HoverInfo {
    fn get_hover(&self, doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::Hover>;
}

pub trait SemanticTokens {
    fn build_semantic_tokens(&self, builder: &mut SemanticTokensBuilder);
}

pub trait InlayHints {
    fn build_inlay_hint(&self, acc: &mut Vec<lsp_types::InlayHint>);
}

pub trait CodeLens {
    fn build_code_lens(&self, acc: &mut Vec<lsp_types::CodeLens>);
}

pub trait CompletionItems {
    fn build_completion_items(
        &self,
        acc: &mut Vec<CompletionItem>,
        doc: &lsp_textdocument::FullTextDocument,
    );
}

impl AstItem for Arc<RwLock<dyn AstItem>> {
    fn get_url(&self) -> Arc<Url> {
        self.read().unwrap().get_url()
    }

    fn get_range(&self) -> tree_sitter::Range {
        self.read().unwrap().get_range()
    }

    fn get_parent(&self) -> Option<Weak<RwLock<dyn AstItem>>> {
        self.read().unwrap().get_parent()
    }

    fn set_parent(&mut self, parent: Weak<RwLock<dyn AstItem>>) {
        self.write().unwrap().set_parent(parent)
    }

    fn inject_parent(&mut self, parent: Weak<RwLock<dyn AstItem>>) {
        self.write().unwrap().inject_parent(parent)
    }

    fn find_at_offset(&self, offset: &usize) -> Option<Arc<RwLock<dyn AstItem>>> {
        self.read().unwrap().find_at_offset(offset)
    }

    fn swap_at_offset(&mut self, offset: &usize, item: &Rc<RefCell<dyn AstItemBuilder>>) {
        self.write().unwrap().swap_at_offset(offset, item)
    }

    fn is_scope(&self) -> bool {
        self.read().unwrap().is_scope()
    }

    fn get_scope_range(&self) -> [usize; 2] {
        self.read().unwrap().get_scope_range()
    }
}

impl DocumentSymbols for Arc<RwLock<dyn AstItem>> {
    fn get_document_symbols(
        &self,
        doc: &lsp_textdocument::FullTextDocument,
    ) -> Option<DocumentSymbol> {
        self.read().unwrap().get_document_symbols(doc)
    }
}

impl HoverInfo for Arc<RwLock<dyn AstItem>> {
    fn get_hover(&self, doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::Hover> {
        self.read().unwrap().get_hover(doc)
    }
}

impl SemanticTokens for Arc<RwLock<dyn AstItem>> {
    fn build_semantic_tokens(&self, builder: &mut SemanticTokensBuilder) {
        self.read().unwrap().build_semantic_tokens(builder)
    }
}

impl InlayHints for Arc<RwLock<dyn AstItem>> {
    fn build_inlay_hint(&self, acc: &mut Vec<lsp_types::InlayHint>) {
        self.read().unwrap().build_inlay_hint(acc)
    }
}

impl CodeLens for Arc<RwLock<dyn AstItem>> {
    fn build_code_lens(&self, acc: &mut Vec<lsp_types::CodeLens>) {
        self.read().unwrap().build_code_lens(acc)
    }
}

impl CompletionItems for Arc<RwLock<dyn AstItem>> {
    fn build_completion_items(
        &self,
        acc: &mut Vec<CompletionItem>,
        doc: &lsp_textdocument::FullTextDocument,
    ) {
        self.read().unwrap().build_completion_items(acc, doc)
    }
}
