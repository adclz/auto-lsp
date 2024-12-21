use crate::{
    builders::{BuilderParams, StaticBuilder},
    convert::TryFromBuilder,
    pending_symbol::AstBuilder,
    semantic_tokens::SemanticTokensBuilder,
};
use downcast_rs::{impl_downcast, Downcast};
use lsp_textdocument::FullTextDocument;
use lsp_types::{
    request::GotoDeclarationResponse, CompletionItem, Diagnostic, DocumentSymbol,
    GotoDefinitionResponse, Position, Range, Url,
};
use parking_lot::RwLock;
use std::sync::{Arc, Weak};

use super::workspace::WorkspaceContext;

#[derive(Clone)]
pub struct AstSymbolData {
    pub url: Arc<Url>,
    pub parent: Option<WeakSymbol>,
    pub referrers: Option<Referrers>,
    pub target: Option<WeakSymbol>,
    pub range: tree_sitter::Range,
}

impl AstSymbolData {
    pub fn new(url: Arc<Url>, range: tree_sitter::Range) -> Self {
        Self {
            url,
            parent: None,
            referrers: None,
            target: None,
            range,
        }
    }
}

pub trait SymbolData {
    fn get_url(&self) -> Arc<Url>;
    fn get_range(&self) -> tree_sitter::Range;
    fn get_parent(&self) -> Option<WeakSymbol>;
    fn set_parent(&mut self, parent: WeakSymbol);
    fn get_target(&self) -> Option<&WeakSymbol>;
    fn set_target(&mut self, target: WeakSymbol);
    fn reset_target(&mut self);
    fn get_referrers(&self) -> &Option<Referrers>;
    fn get_mut_referrers(&mut self) -> &mut Referrers;
    fn drop_referrers(&self) {
        self.get_referrers().as_ref().map(|r| {
            r.0.iter().for_each(|r| {
                r.to_dyn().map(|r| {
                    r.write().reset_accessor();
                });
            });
        });
    }
}

impl<T: AstSymbol> Drop for Symbol<T> {
    fn drop(&mut self) {
        let write = self.write();
        write.drop_referrers();

        if write.is_accessor() {
            write.get_accessor().map(|a| {
                a.to_dyn().map(|a| {
                    a.write().get_mut_referrers().remove_empty_references();
                })
            });
        }
    }
}

impl SymbolData for AstSymbolData {
    fn get_url(&self) -> Arc<Url> {
        self.url.clone()
    }

    fn get_range(&self) -> tree_sitter::Range {
        self.range
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

    fn set_target(&mut self, target: WeakSymbol) {
        self.target = Some(target);
    }

    fn reset_target(&mut self) {
        self.target = None;
    }

    fn get_referrers(&self) -> &Option<Referrers> {
        &self.referrers
    }

    fn get_mut_referrers(&mut self) -> &mut Referrers {
        self.referrers.get_or_insert_default()
    }
}

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
    + GoToDefinition
    + GoToDeclaration
    + Scope
    + Accessor
    + Locator
    + Parent
    + Check
{
    fn get_data(&self) -> &AstSymbolData;
    fn get_mut_data(&mut self) -> &mut AstSymbolData;

    fn get_size(&self) -> usize {
        let range = self.get_data().get_range();
        range.end_byte - range.start_byte
    }

    fn get_text<'a>(&self, source_code: &'a [u8]) -> &'a str {
        let range = self.get_data().get_range();
        std::str::from_utf8(&source_code[range.start_byte..range.end_byte]).unwrap()
    }

    fn get_parent_scope(&self) -> Option<DynSymbol> {
        let mut parent = self.get_data().get_parent();
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
        let range = self.get_data().get_range();
        range.start_byte <= offset && offset <= range.end_byte
    }
    // LSP

    fn get_start_position(&self, doc: &FullTextDocument) -> Position {
        doc.position_at(self.get_data().get_range().start_byte as u32)
    }

    fn get_end_position(&self, doc: &FullTextDocument) -> Position {
        doc.position_at(self.get_data().get_range().end_byte as u32)
    }

    fn get_lsp_range(&self, doc: &FullTextDocument) -> Range {
        let start = self.get_start_position(doc);
        let end = self.get_end_position(doc);
        lsp_types::Range { start, end }
    }
}

impl_downcast!(AstSymbol);

impl<T: AstSymbol + ?Sized> SymbolData for T {
    fn get_url(&self) -> Arc<Url> {
        self.get_data().get_url()
    }

    fn get_range(&self) -> tree_sitter::Range {
        self.get_data().get_range()
    }

    fn get_parent(&self) -> Option<WeakSymbol> {
        self.get_data().get_parent()
    }

    fn set_parent(&mut self, parent: WeakSymbol) {
        self.get_mut_data().set_parent(parent)
    }

    fn get_target(&self) -> Option<&WeakSymbol> {
        self.get_data().get_target()
    }

    fn set_target(&mut self, target: WeakSymbol) {
        self.get_mut_data().set_target(target)
    }

    fn reset_target(&mut self) {
        self.get_mut_data().reset_target();
    }

    fn get_referrers(&self) -> &Option<Referrers> {
        self.get_data().get_referrers()
    }

    fn get_mut_referrers(&mut self) -> &mut Referrers {
        self.get_mut_data().get_mut_referrers()
    }
}

#[derive(Clone)]
pub struct Referrers(Vec<WeakSymbol>);

impl Default for Referrers {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl Referrers {
    pub fn add_reference(&mut self, symbol: WeakSymbol) {
        self.0.push(symbol);
    }

    pub fn remove_empty_references(&mut self) {
        self.0.retain(|f| f.0.weak_count() > 0);
    }

    pub fn get_references(&self) -> &Vec<WeakSymbol> {
        &self.0
    }
}

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

    pub fn replace(&mut self, symbol: Symbol<T>) {
        *self = symbol;
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
    fn is_scope(&self) -> bool {
        false
    }
    fn get_scope_range(&self) -> Vec<[usize; 2]> {
        Vec::new()
    }
}

pub trait DocumentSymbols {
    fn get_document_symbols(&self, _doc: &FullTextDocument) -> Option<DocumentSymbol> {
        None
    }
}

pub trait HoverInfo {
    fn get_hover(&self, _doc: &FullTextDocument) -> Option<lsp_types::Hover> {
        None
    }
}

pub trait GoToDefinition {
    fn go_to_definition(&self, _doc: &FullTextDocument) -> Option<GotoDefinitionResponse> {
        None
    }
}

pub trait GoToDeclaration {
    fn go_to_declaration(&self, _doc: &FullTextDocument) -> Option<GotoDeclarationResponse> {
        None
    }
}

pub trait SemanticTokens {
    fn build_semantic_tokens(&self, _builder: &mut SemanticTokensBuilder) {}
}

pub trait InlayHints {
    fn build_inlay_hint(&self, _doc: &FullTextDocument, _acc: &mut Vec<lsp_types::InlayHint>) {}
}

pub trait CodeLens {
    fn build_code_lens(&self, _acc: &mut Vec<lsp_types::CodeLens>) {}
}

pub trait CompletionItems {
    fn build_completion_items(&self, _acc: &mut Vec<CompletionItem>, _doc: &FullTextDocument) {}
}

macro_rules! impl_build {
    ($trait:ident, $fn_name:ident(&self, $($param_name:ident: $param_type:ty),*)) => {
        impl $trait for DynSymbol {
            fn $fn_name(&self, $($param_name: $param_type),*) {
                self.read().$fn_name($($param_name),*)
            }
        }

        impl<T: AstSymbol> $trait for Option<Symbol<T>> {
            fn $fn_name(&self, $($param_name: $param_type),*) {
                if let Some(node) = self.as_ref() {
                    node.read().$fn_name($($param_name),*)
                }
            }
        }

        impl<T: AstSymbol> $trait for Vec<Symbol<T>> {
            fn $fn_name(&self, $($param_name: $param_type),*) {
                for symbol in self.iter() {
                    symbol.read().$fn_name($($param_name),*)
                }
            }
        }
    };
}

impl_build!(SemanticTokens, build_semantic_tokens(&self, builder: &mut SemanticTokensBuilder));
impl_build!(InlayHints, build_inlay_hint(&self, doc: &FullTextDocument, acc: &mut Vec<lsp_types::InlayHint>));
impl_build!(CodeLens, build_code_lens(&self, acc: &mut Vec<lsp_types::CodeLens>));
impl_build!(CompletionItems, build_completion_items(&self, acc: &mut Vec<CompletionItem>, doc: &FullTextDocument));

pub trait IsAccessor {
    fn is_accessor(&self) -> bool {
        false
    }
    fn get_accessor(&self) -> Option<&WeakSymbol> {
        None
    }
    fn set_accessor(&mut self, _accessor: WeakSymbol) {}
    fn reset_accessor(&mut self) {}
}

pub trait Accessor: IsAccessor {
    fn find(
        &self,
        _doc: &FullTextDocument,
        _ctx: &dyn WorkspaceContext,
    ) -> Result<Option<DynSymbol>, Diagnostic> {
        Ok(None)
    }
}

pub trait Locator {
    fn find_at_offset(&self, offset: usize) -> Option<DynSymbol>;
}

impl Locator for DynSymbol {
    fn find_at_offset(&self, offset: usize) -> Option<DynSymbol> {
        let symbol = self.read();
        match symbol.is_inside_offset(offset) {
            true => symbol.find_at_offset(offset).or_else(|| Some(self.clone())),
            false => None,
        }
    }
}

impl<T: AstSymbol> Locator for Symbol<T> {
    fn find_at_offset(&self, offset: usize) -> Option<DynSymbol> {
        let symbol = self.read();
        match symbol.is_inside_offset(offset) {
            true => symbol
                .find_at_offset(offset)
                .or_else(|| Some(self.to_dyn())),
            false => None,
        }
    }
}

impl<T: AstSymbol> Locator for Option<Symbol<T>> {
    fn find_at_offset(&self, offset: usize) -> Option<DynSymbol> {
        self.as_ref()?.find_at_offset(offset)
    }
}

impl<T: Locator> Locator for Vec<T> {
    fn find_at_offset(&self, offset: usize) -> Option<DynSymbol> {
        self.iter().find_map(|symbol| symbol.find_at_offset(offset))
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

pub trait Check {
    fn must_check(&self) -> bool {
        false
    }
    fn check(&self, _doc: &FullTextDocument, _diagnostics: &mut Vec<Diagnostic>) {}
}

impl BuilderParams<'_> {
    pub fn new<'a>(
        ctx: &'a dyn WorkspaceContext,
        query: &'a tree_sitter::Query,
        root_node: tree_sitter::Node<'a>,
        doc: &'static FullTextDocument,
        url: Arc<Url>,
        diagnostics: &'a mut Vec<Diagnostic>,
    ) -> BuilderParams<'a> {
        BuilderParams {
            ctx,
            query,
            root_node,
            doc,
            url,
            diagnostics,
        }
    }
}

pub trait Swap<T, Y>
where
    T: AstBuilder,
    Y: AstSymbol
        + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>
        + StaticBuilder<T, Y>,
{
    fn to_swap<'a>(
        &mut self,
        offset: usize,
        builder_params: &'a mut BuilderParams<'a>,
    ) -> Result<(), Diagnostic>;
}

impl<T, Y> Swap<T, Y> for Symbol<Y>
where
    T: AstBuilder,
    Y: AstSymbol
        + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>
        + StaticBuilder<T, Y>,
{
    fn to_swap<'a>(
        &mut self,
        offset: usize,
        builder_params: &'a mut BuilderParams<'a>,
    ) -> Result<(), Diagnostic> {
        let symbol = Y::static_build(
            builder_params,
            Some(std::ops::Range {
                start: offset,
                end: offset,
            }),
        );
        *self = symbol?;
        Ok(())
    }
}

impl<T, Y> Swap<T, Y> for Vec<Symbol<Y>>
where
    T: AstBuilder,
    Y: AstSymbol
        + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>
        + StaticBuilder<T, Y>,
{
    fn to_swap<'a>(
        &mut self,
        offset: usize,
        builder_params: &'a mut BuilderParams<'a>,
    ) -> Result<(), Diagnostic> {
        let symbol = Y::static_build(
            builder_params,
            Some(std::ops::Range {
                start: offset,
                end: offset,
            }),
        )?;

        for existing_symbol in self.iter_mut() {
            if existing_symbol.write().is_inside_offset(offset) {
                *existing_symbol = symbol;
                return Ok(());
            }
        }

        let insert_index = match self.binary_search_by(|existing_symbol| {
            existing_symbol.read().get_range().start_byte.cmp(&offset)
        }) {
            Ok(index) | Err(index) => index,
        };
        self.insert(insert_index, symbol);

        Ok(())
    }
}
