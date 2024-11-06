use crate::builders::semantic_tokens::SemanticTokensBuilder;
use downcast_rs::{impl_downcast, Downcast};
use lsp_types::{CompletionItem, DocumentSymbol};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use super::ast_item_builder::AstItemBuilder;

pub trait AstItem: Downcast {
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

    fn get_parent(&self) -> Option<std::sync::Arc<std::sync::RwLock<dyn AstItem>>>;
    fn set_parent(&mut self, parent: std::sync::Arc<std::sync::RwLock<dyn AstItem>>);
    fn inject_parent(&mut self, parent: std::sync::Arc<std::sync::RwLock<dyn AstItem>>);
    fn get_highest_parent(&self) -> std::sync::Arc<std::sync::RwLock<dyn AstItem>> {
        let mut parent = self.get_parent();
        while let Some(p) = parent {
            parent = p.read().unwrap().get_parent();
        }
        parent.unwrap()
    }

    fn find_at_offset(
        &self,
        offset: &usize,
    ) -> Option<std::sync::Arc<std::sync::RwLock<dyn AstItem>>>;

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

    fn is_borrowable(&self, _other: &dyn AstItem) -> bool {
        false
    }

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

    fn get_document_symbols(
        &self,
        _doc: &lsp_textdocument::FullTextDocument,
    ) -> Option<DocumentSymbol> {
        None
    }

    fn get_hover(&self, _doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::Hover> {
        None
    }

    fn build_semantic_tokens(&self, _builder: &mut SemanticTokensBuilder) {}

    fn build_inlay_hint(&self, _acc: &mut Vec<lsp_types::InlayHint>) {}

    fn build_code_lens(&self, _acc: &mut Vec<lsp_types::CodeLens>) {}

    fn build_completion_items(
        &self,
        _acc: &mut Vec<CompletionItem>,
        _doc: &lsp_textdocument::FullTextDocument,
    ) {
    }
}

impl_downcast!(AstItem);

impl AstItem for Arc<RwLock<dyn AstItem>> {
    fn get_range(&self) -> tree_sitter::Range {
        self.read().unwrap().get_range()
    }

    fn get_parent(&self) -> Option<Arc<RwLock<dyn AstItem>>> {
        self.read().unwrap().get_parent()
    }

    fn set_parent(&mut self, parent: Arc<RwLock<dyn AstItem>>) {
        self.write().unwrap().set_parent(parent)
    }

    fn inject_parent(&mut self, parent: Arc<RwLock<dyn AstItem>>) {
        self.write().unwrap().inject_parent(parent)
    }

    fn find_at_offset(&self, offset: &usize) -> Option<Arc<RwLock<dyn AstItem>>> {
        self.read().unwrap().find_at_offset(offset)
    }

    fn swap_at_offset(&mut self, offset: &usize, item: &Rc<RefCell<dyn AstItemBuilder>>) {
        self.write().unwrap().swap_at_offset(offset, item)
    }

    fn get_document_symbols(
        &self,
        _doc: &lsp_textdocument::FullTextDocument,
    ) -> Option<DocumentSymbol> {
        self.read().unwrap().get_document_symbols(_doc)
    }

    fn get_hover(&self, _doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::Hover> {
        self.read().unwrap().get_hover(_doc)
    }

    fn build_semantic_tokens(&self, builder: &mut SemanticTokensBuilder) {
        self.read().unwrap().build_semantic_tokens(builder)
    }

    fn build_inlay_hint(&self, _acc: &mut Vec<lsp_types::InlayHint>) {
        self.read().unwrap().build_inlay_hint(_acc)
    }

    fn build_code_lens(&self, _acc: &mut Vec<lsp_types::CodeLens>) {
        self.read().unwrap().build_code_lens(_acc)
    }

    fn build_completion_items(
        &self,
        _acc: &mut Vec<CompletionItem>,
        _doc: &lsp_textdocument::FullTextDocument,
    ) {
        self.read().unwrap().build_completion_items(_acc, _doc)
    }
}
