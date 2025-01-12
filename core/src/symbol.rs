use crate::{
    builders::{BuilderParams, StaticBuilder},
    convert::TryFromBuilder,
    pending_symbol::AstBuilder,
    queryable::Queryable,
    semantic_tokens::SemanticTokensBuilder,
    workspace::Document,
};
use downcast_rs::{impl_downcast, Downcast};
use lsp_types::{
    request::GotoDeclarationResponse, CompletionItem, Diagnostic, DocumentSymbol,
    GotoDefinitionResponse, Position, Range, Url,
};
use parking_lot::RwLock;
use std::{
    ops::ControlFlow,
    sync::{Arc, Weak},
};

#[derive(Clone)]
pub struct AstSymbolData {
    pub url: Arc<Url>,
    pub parent: Option<WeakSymbol>,
    pub comment: Option<std::ops::Range<usize>>,
    pub referrers: Option<Referrers>,
    pub target: Option<WeakSymbol>,
    pub range: std::ops::Range<usize>,
}

impl AstSymbolData {
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
    /// If the referrer was not dropped, add it to the unsolved checks field of [`BuilderParams`]
    fn drop_referrers(&mut self, params: &mut BuilderParams);
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
        self.get_mut_referrers().0.retain(|r| r.0.weak_count() > 0);
    }

    fn drop_referrers(&mut self, params: &mut BuilderParams) {
        self.get_mut_referrers().0.retain(|r| {
            if let Some(symbol) = r.to_dyn() {
                let read = symbol.read();
                if read.get_target().is_some() {
                    drop(read);
                    symbol.write().reset_target();
                    params.unsolved_references.push(r.clone());
                }
            }
            false
        });
    }
}

pub trait SymbolData {
    fn get_url(&self) -> Arc<Url>;
    fn get_range(&self) -> std::ops::Range<usize>;
    fn get_parent(&self) -> Option<WeakSymbol>;
    fn set_parent(&mut self, parent: WeakSymbol);
    fn get_comment<'a>(&self, source_code: &'a [u8]) -> Option<&'a str>;
    fn set_comment(&mut self, range: Option<std::ops::Range<usize>>);
    fn get_target(&self) -> Option<&WeakSymbol>;
    fn set_target(&mut self, target: WeakSymbol);
    fn reset_target(&mut self);
    fn get_referrers(&self) -> &Option<Referrers>;
    fn get_mut_referrers(&mut self) -> &mut Referrers;
}

impl SymbolData for AstSymbolData {
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
    + IsComment
    + Scope
    + Accessor
    + Locator
    + Parent
    + Check
    + DynamicSwap
    + EditRange
    + CollectReferences
{
    fn get_data(&self) -> &AstSymbolData;
    fn get_mut_data(&mut self) -> &mut AstSymbolData;

    fn get_text<'a>(&self, source_code: &'a [u8]) -> Option<&'a str> {
        let range = self.get_data().get_range();
        // Check if the range is within bounds and valid
        if range.start <= range.end && range.end <= source_code.len() {
            std::str::from_utf8(&source_code[range.start..range.end]).ok()
        } else {
            None
        }
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
        range.start <= offset && offset <= range.end
    }
    // LSP

    fn get_start_position(&self, workspace: &Document) -> Position {
        let range = self.get_data().get_range();
        let node = workspace
            .cst
            .root_node()
            .descendant_for_byte_range(range.start, range.start)
            .unwrap();

        Position {
            line: node.start_position().row as u32,
            character: node.start_position().column as u32,
        }
    }

    fn get_end_position(&self, workspace: &Document) -> Position {
        let range = self.get_data().get_range();
        let node = workspace
            .cst
            .root_node()
            .descendant_for_byte_range(range.end, range.end)
            .unwrap();

        Position {
            line: node.start_position().row as u32,
            character: node.start_position().column as u32,
        }
    }

    fn get_lsp_range(&self, workspace: &Document) -> Range {
        let range = self.get_data().get_range();
        let node = workspace
            .cst
            .root_node()
            .descendant_for_byte_range(range.start, range.end)
            .unwrap();

        lsp_types::Range {
            start: Position {
                line: node.start_position().row as u32,
                character: node.start_position().column as u32,
            },
            end: Position {
                line: node.end_position().row as u32,
                character: node.end_position().column as u32,
            },
        }
    }
}

impl_downcast!(AstSymbol);

impl<T: AstSymbol + ?Sized> SymbolData for T {
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

    fn get_comment<'a>(&self, source_code: &'a [u8]) -> Option<&'a str> {
        self.get_data().get_comment(source_code)
    }

    fn set_comment(&mut self, range: Option<std::ops::Range<usize>>) {
        self.get_mut_data().set_comment(range)
    }
}

#[derive(Clone)]
pub struct Symbol<T: AstSymbol>(Arc<RwLock<T>>);

impl<T: AstSymbol> Symbol<T> {
    fn new(symbol: T) -> Self {
        Self(Arc::new(RwLock::new(symbol)))
    }

    pub fn new_and_check(symbol: T, params: &mut BuilderParams) -> Self {
        let arc = Symbol::new(symbol);
        let read = arc.read();
        if read.is_accessor() {
            params.unsolved_references.push(arc.to_weak());
        }
        if read.must_check() {
            params.unsolved_checks.push(arc.to_weak());
        }
        drop(read);
        arc.write().inject_parent(arc.to_weak());
        arc
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

    pub fn read(&self) -> parking_lot::RwLockReadGuard<dyn AstSymbol> {
        self.0.read()
    }

    pub fn write(&self) -> parking_lot::RwLockWriteGuard<dyn AstSymbol> {
        self.0.write()
    }

    pub fn to_weak(&self) -> WeakSymbol {
        WeakSymbol::new(self)
    }

    pub fn swap(&mut self, other: &mut Self) {
        std::mem::swap(&mut self.0, &mut other.0);
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

pub trait IsScope {
    fn is_scope(&self) -> bool {
        false
    }
}

pub trait Scope: IsScope {
    fn get_scope_range(&self) -> Vec<[usize; 2]> {
        Vec::new()
    }
}

pub trait IsComment {
    fn is_comment(&self) -> bool {
        false
    }
}

pub trait DocumentSymbols {
    fn get_document_symbols(&self, _doc: &Document) -> Option<DocumentSymbol> {
        None
    }
}

pub trait HoverInfo {
    fn get_hover(&self, _doc: &Document) -> Option<lsp_types::Hover> {
        None
    }
}

pub trait GoToDefinition {
    fn go_to_definition(&self, _doc: &Document) -> Option<GotoDefinitionResponse> {
        None
    }
}

pub trait GoToDeclaration {
    fn go_to_declaration(&self, _doc: &Document) -> Option<GotoDeclarationResponse> {
        None
    }
}

pub trait SemanticTokens {
    fn build_semantic_tokens(&self, _builder: &mut SemanticTokensBuilder) {}
}

pub trait InlayHints {
    fn build_inlay_hint(&self, _doc: &Document, _acc: &mut Vec<lsp_types::InlayHint>) {}
}

pub trait CodeLens {
    fn build_code_lens(&self, _acc: &mut Vec<lsp_types::CodeLens>) {}
}

pub trait CompletionItems {
    fn build_completion_items(&self, _acc: &mut Vec<CompletionItem>, _doc: &Document) {}
}

macro_rules! impl_build {
    ($trait:ident, $fn_name:ident(&self, $($param_name:ident: $param_type:ty),*)) => {
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
impl_build!(InlayHints, build_inlay_hint(&self, doc: &Document, acc: &mut Vec<lsp_types::InlayHint>));
impl_build!(CodeLens, build_code_lens(&self, acc: &mut Vec<lsp_types::CodeLens>));
impl_build!(CompletionItems, build_completion_items(&self, acc: &mut Vec<CompletionItem>, doc: &Document));

pub trait IsAccessor: SymbolData {
    fn is_accessor(&self) -> bool {
        false
    }
}

pub trait Accessor: IsAccessor {
    fn find(&self, _doc: &Document) -> Result<Option<DynSymbol>, Diagnostic> {
        Ok(None)
    }
}

pub trait Locator {
    fn find_at_offset(&self, offset: usize) -> Option<DynSymbol>;
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

impl<T: AstSymbol> Locator for Vec<Symbol<T>> {
    fn find_at_offset(&self, offset: usize) -> Option<DynSymbol> {
        self.iter().find_map(|symbol| symbol.find_at_offset(offset))
    }
}

pub trait EditRange {
    fn edit_range(&self, start: usize, offset: isize);
}

fn edit(data: &mut AstSymbolData, start: usize, offset: isize) {
    if data.range.start >= start {
        // Entire range is after the offset; shift both start and end
        data.range.start = (data.range.start as isize + offset) as usize;
        data.range.end = (data.range.end as isize + offset) as usize;
    } else if data.range.end >= start {
        // The offset occurs within the range; adjust only the end
        data.range.end = (data.range.end as isize + offset) as usize;
    }
}

impl EditRange for DynSymbol {
    fn edit_range(&self, start: usize, offset: isize) {
        let mut write = self.write();
        let data = write.get_mut_data();
        edit(data, start, offset);
        write.edit_range(start, offset);
    }
}

impl<T: AstSymbol> EditRange for Symbol<T> {
    fn edit_range(&self, start: usize, offset: isize) {
        let mut write = self.write();
        let data = write.get_mut_data();
        edit(data, start, offset);
        write.edit_range(start, offset);
    }
}

impl<T: AstSymbol> EditRange for Option<Symbol<T>> {
    fn edit_range(&self, start: usize, offset: isize) {
        if let Some(symbol) = self.as_ref() {
            let mut write = symbol.write();
            let data = write.get_mut_data();
            edit(data, start, offset);
            write.edit_range(start, offset);
        }
    }
}

impl<T: AstSymbol> EditRange for Vec<Symbol<T>> {
    fn edit_range(&self, start: usize, offset: isize) {
        for symbol in self.iter() {
            let mut write = symbol.write();
            let data = write.get_mut_data();
            edit(data, start, offset);
            write.edit_range(start, offset);
        }
    }
}

pub trait CollectReferences {
    fn collect_references(&self, params: &mut BuilderParams);
}

impl<T: AstSymbol> CollectReferences for Symbol<T> {
    fn collect_references(&self, params: &mut BuilderParams) {
        if let Some(target) = &self.read().get_data().target {
            params.unsolved_references.push(target.clone());
        }
    }
}

impl<T: AstSymbol> CollectReferences for Option<Symbol<T>> {
    fn collect_references(&self, params: &mut BuilderParams) {
        if let Some(symbol) = self.as_ref() {
            symbol.collect_references(params);
        }
    }
}

impl<T: AstSymbol> CollectReferences for Vec<Symbol<T>> {
    fn collect_references(&self, params: &mut BuilderParams) {
        for symbol in self.iter() {
            symbol.collect_references(params);
        }
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

pub trait IsCheck {
    fn must_check(&self) -> bool {
        false
    }
}

pub trait Check: IsCheck {
    fn check(&self, _doc: &Document, _diagnostics: &mut Vec<Diagnostic>) -> Result<(), ()> {
        Ok(())
    }
}

pub trait DynamicSwap {
    fn dyn_swap<'a>(
        &mut self,
        start: usize,
        offset: isize,
        builder_params: &'a mut BuilderParams,
    ) -> ControlFlow<Result<usize, Diagnostic>, ()>;
}

pub trait StaticSwap<T, Y>
where
    T: AstBuilder + Queryable,
    Y: AstSymbol,
{
    fn to_swap<'a>(
        &mut self,
        start: usize,
        offset: isize,
        builder_params: &'a mut BuilderParams,
    ) -> ControlFlow<Result<usize, Diagnostic>, ()>;
}

impl<T, Y> StaticSwap<T, Y> for Symbol<Y>
where
    T: AstBuilder + Queryable,
    Y: AstSymbol
        + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>
        + StaticBuilder<T, Y>
        + CollectReferences,
{
    fn to_swap<'a>(
        &mut self,
        start: usize,
        _offset: isize,
        builder_params: &'a mut BuilderParams,
    ) -> ControlFlow<Result<usize, Diagnostic>, ()> {
        let read = self.read();
        match read.is_inside_offset(start) {
            true => {
                drop(read);
                let parent = self.read().get_parent();
                let range = self.read().get_range();
                log::info!("");
                log::info!("Incremental update at {:?}", range);
                log::info!("");
                let symbol = Symbol::new_and_check(
                    match Y::static_build(builder_params, Some(range)) {
                        Ok(symbol) => symbol,
                        Err(err) => return ControlFlow::Break(Err(err)),
                    },
                    builder_params,
                );
                self.collect_references(builder_params);
                if let Some(parent) = parent {
                    symbol.write().set_parent(parent);
                }
                *self = symbol;
                ControlFlow::Break(Ok(self.read().get_range().start))
            }
            false => ControlFlow::Continue(()),
        }
    }
}

impl<T, Y> StaticSwap<T, Y> for Option<Symbol<Y>>
where
    T: AstBuilder + Queryable,
    Y: AstSymbol
        + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>
        + StaticBuilder<T, Y>
        + CollectReferences,
{
    fn to_swap<'a>(
        &mut self,
        start: usize,
        offset: isize,
        builder_params: &'a mut BuilderParams,
    ) -> ControlFlow<Result<usize, Diagnostic>, ()> {
        match self {
            Some(symbol) => symbol.to_swap(start, offset, builder_params),
            None => ControlFlow::Continue(()),
        }
    }
}

impl<T, Y> StaticSwap<T, Y> for Vec<Symbol<Y>>
where
    T: AstBuilder + Queryable,
    Y: AstSymbol
        + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>
        + StaticBuilder<T, Y>
        + CollectReferences,
{
    fn to_swap<'a>(
        &mut self,
        start: usize,
        offset: isize,
        builder_params: &'a mut BuilderParams,
    ) -> ControlFlow<Result<usize, Diagnostic>, ()> {
        for symbol in self.iter_mut() {
            match symbol.to_swap(start, offset, builder_params) {
                ControlFlow::Break(result) => return ControlFlow::Break(result),
                ControlFlow::Continue(()) => continue,
            }
        }
        ControlFlow::Continue(())
    }
}
