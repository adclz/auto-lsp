use std::ops::ControlFlow;

use lsp_types::Diagnostic;

use crate::core_build::buildable::Buildable;
use crate::core_build::buildable::Queryable;
use crate::core_build::buildable::StaticBuildable;
use crate::core_build::downcast::TryFromBuilder;
use crate::core_build::main_builder::MainBuilder;

use super::core::AstSymbol;
use super::data::*;
use super::symbol::*;

///
pub trait CollectReferences {
    fn collect_references(&self, params: &mut MainBuilder);
}

impl<T: AstSymbol> CollectReferences for Symbol<T> {
    fn collect_references(&self, params: &mut MainBuilder) {
        if let Some(target) = &self.read().get_data().target {
            params.unsolved_references.push(target.clone());
        }
    }
}

impl<T: AstSymbol> CollectReferences for Option<Symbol<T>> {
    fn collect_references(&self, params: &mut MainBuilder) {
        if let Some(symbol) = self.as_ref() {
            symbol.collect_references(params);
        }
    }
}

impl<T: AstSymbol> CollectReferences for Vec<Symbol<T>> {
    fn collect_references(&self, params: &mut MainBuilder) {
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

pub trait UpdateRange {
    fn edit_range(&self, start: usize, offset: isize);
}

fn edit(data: &mut SymbolData, start: usize, offset: isize) {
    if data.range.start >= start {
        // Entire range is after the offset; shift both start and end
        data.range.start = (data.range.start as isize + offset) as usize;
        data.range.end = (data.range.end as isize + offset) as usize;
    } else if data.range.end >= start {
        // The offset occurs within the range; adjust only the end
        data.range.end = (data.range.end as isize + offset) as usize;
    }
}

impl UpdateRange for DynSymbol {
    fn edit_range(&self, start: usize, offset: isize) {
        let mut write = self.write();
        let data = write.get_mut_data();
        edit(data, start, offset);
        write.edit_range(start, offset);
    }
}

impl<T: AstSymbol> UpdateRange for Symbol<T> {
    fn edit_range(&self, start: usize, offset: isize) {
        let mut write = self.write();
        let data = write.get_mut_data();
        edit(data, start, offset);
        write.edit_range(start, offset);
    }
}

impl<T: AstSymbol> UpdateRange for Option<Symbol<T>> {
    fn edit_range(&self, start: usize, offset: isize) {
        if let Some(symbol) = self.as_ref() {
            let mut write = symbol.write();
            let data = write.get_mut_data();
            edit(data, start, offset);
            write.edit_range(start, offset);
        }
    }
}

impl<T: AstSymbol> UpdateRange for Vec<Symbol<T>> {
    fn edit_range(&self, start: usize, offset: isize) {
        for symbol in self.iter() {
            let mut write = symbol.write();
            let data = write.get_mut_data();
            edit(data, start, offset);
            write.edit_range(start, offset);
        }
    }
}

pub trait DynamicUpdate {
    fn dyn_swap<'a>(
        &mut self,
        start: usize,
        offset: isize,
        builder_params: &'a mut MainBuilder,
    ) -> ControlFlow<Result<usize, Diagnostic>, ()>;
}

pub trait StaticUpdate<T, Y>
where
    T: Buildable + Queryable,
    Y: AstSymbol,
{
    fn update<'a>(
        &mut self,
        start: usize,
        offset: isize,
        builder_params: &'a mut MainBuilder,
    ) -> ControlFlow<Result<usize, Diagnostic>, ()>;
}

impl<T, Y> StaticUpdate<T, Y> for Symbol<Y>
where
    T: Buildable + Queryable,
    Y: AstSymbol
        + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>
        + StaticBuildable<T, Y>
        + CollectReferences,
{
    fn update<'a>(
        &mut self,
        start: usize,
        offset: isize,
        builder_params: &'a mut MainBuilder,
    ) -> ControlFlow<Result<usize, Diagnostic>, ()> {
        let read = self.read();
        match read.is_inside_offset(start) {
            true => {
                drop(read);
                self.write().dyn_swap(start, offset, builder_params)?;
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

impl<T, Y> StaticUpdate<T, Y> for Option<Symbol<Y>>
where
    T: Buildable + Queryable,
    Y: AstSymbol
        + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>
        + StaticBuildable<T, Y>
        + CollectReferences,
{
    fn update<'a>(
        &mut self,
        start: usize,
        offset: isize,
        builder_params: &'a mut MainBuilder,
    ) -> ControlFlow<Result<usize, Diagnostic>, ()> {
        match self {
            Some(symbol) => symbol.update(start, offset, builder_params),
            None => ControlFlow::Continue(()),
        }
    }
}

impl<T, Y> StaticUpdate<T, Y> for Vec<Symbol<Y>>
where
    T: Buildable + Queryable,
    Y: AstSymbol
        + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>
        + StaticBuildable<T, Y>
        + CollectReferences,
{
    fn update<'a>(
        &mut self,
        start: usize,
        offset: isize,
        builder_params: &'a mut MainBuilder,
    ) -> ControlFlow<Result<usize, Diagnostic>, ()> {
        for symbol in self.iter_mut() {
            match symbol.update(start, offset, builder_params) {
                ControlFlow::Break(result) => return ControlFlow::Break(result),
                ControlFlow::Continue(()) => continue,
            }
        }
        ControlFlow::Continue(())
    }
}
