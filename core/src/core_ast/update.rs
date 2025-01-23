//! This module provides traits for incrementally updating AST.
//!
//! The traits defined here enable:
//! - [`CollectReferences`] Collecting references to AST symbols when a section will be dropped.
//! - [`Parent`] Injecting parent relationships into symbols.
//! - [`UpdateRange`] Modifying the range of symbols in response to edits.
//! - [`UpdateStatic`] and [`UpdateDynamic`] Performing incremental updates on AST nodes based on offset changes.
//!
//! Note: Still under development.

use std::ops::ControlFlow;

use lsp_types::Diagnostic;

use crate::core_build::buildable::Buildable;
use crate::core_build::buildable::Queryable;
use crate::core_build::downcast::TryFromBuilder;
use crate::core_build::stack_builder::InvokeStackBuilder;
use crate::document::Document;
use crate::workspace::Workspace;

use super::core::AstSymbol;
use super::data::*;
use super::symbol::*;

/// A trait for collecting references to an AST symbol.
///
/// This trait is used when a section of the AST is being deleted.
/// It ensures that any references to symbols within the deleted section are collected
/// and stored in [`Workspace`] for further processing.
pub trait CollectReferences {
    fn collect_references(&self, workspace: &mut Workspace);
}

impl<T: AstSymbol> CollectReferences for Symbol<T> {
    fn collect_references(&self, workspace: &mut Workspace) {
        if let Some(target) = &self.read().get_data().target {
            workspace.add_unsolved_reference(&target.to_dyn().unwrap());
        }
    }
}

impl<T: AstSymbol> CollectReferences for Option<Symbol<T>> {
    fn collect_references(&self, workspace: &mut Workspace) {
        if let Some(symbol) = self.as_ref() {
            symbol.collect_references(workspace);
        }
    }
}

impl<T: AstSymbol> CollectReferences for Vec<Symbol<T>> {
    fn collect_references(&self, workspace: &mut Workspace) {
        for symbol in self.iter() {
            symbol.collect_references(workspace);
        }
    }
}

/// A trait for injecting a parent relationship into an AST symbol.
///
/// This trait is used to establish parent-child relationships in the AST,
/// ensuring that newly created symbols are properly linked to their parent nodes.
///
/// // TODO: Move to core_build/buildable.rs
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

/// A trait for updating the range of an AST symbol in response to edits.
///
/// The range represents the start and end positions of the symbol within the source code.
/// This trait ensures that symbols remain consistent with the edited code by adjusting
/// their ranges based on the starting position and the offset of the edit.
pub trait UpdateRange {
    fn edit_range(&self, start: usize, offset: isize);
}

/// Adjusts the range of a symbol based on the given start position and offset.
///
/// - `start`: The position where the edit begins.
/// - `offset`: The amount by which the symbol's range should be adjusted. Positive values
///   extend the range, while negative values shrink it.
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

/// Trait to update an ast symbol incrementally
///
/// This trait is implemented on all symbols.
///
/// To update a symbol, the following conditions must be met:
/// - The symbol is inside the range of the update
/// - No lower level symbols have been updated
pub trait UpdateStatic<T, Y>
where
    T: Buildable + Queryable,
    Y: AstSymbol,
{
    /// ### Conditions for Symbol Updates
    /// - The symbol must be within the range of the edit.
    /// - No lower-level symbols (children) have already been updated.
    ///
    /// ### Return Value
    /// The method returns a [ControlFlow] to indicate the result:
    /// - [ControlFlow::Break]: The symbol was successfully updated, with an optional diagnostic.
    /// - [ControlFlow::Continue]: The symbol did not require updating.
    fn update(
        &mut self,
        start: usize,
        offset: isize,
        parent_check: Option<WeakSymbol>,
        workspace: &mut Workspace,
        document: &Document,
    ) -> ControlFlow<Result<(), Diagnostic>, ()>;
}

impl<T, Y> UpdateStatic<T, Y> for Symbol<Y>
where
    T: Buildable + Queryable,
    Y: AstSymbol
        + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>
        + InvokeStackBuilder<T, Y>
        + CollectReferences,
{
    fn update(
        &mut self,
        start: usize,
        offset: isize,
        parent_check: Option<WeakSymbol>,
        workspace: &mut Workspace,
        document: &Document,
    ) -> ControlFlow<Result<(), Diagnostic>, ()> {
        let read = self.read();
        match read.is_inside_offset(start) {
            true => {
                let check = match read.must_check() {
                    true => Some(self.to_weak()),
                    false => None,
                };
                drop(read);
                // Check if no lower level symbols could be updated.

                self.write()
                    .dyn_update(start, offset, check, workspace, document)?;
                let parent = self.read().get_parent();
                let range = self.read().get_range();
                log::info!("");
                log::info!("Incremental update at {:?}", range);
                log::info!("");

                // Create the symbol
                let symbol = Symbol::new_and_check(
                    match Y::create_symbol(workspace, document, Some(range)) {
                        Ok(symbol) => symbol,
                        Err(err) => return ControlFlow::Break(Err(err)),
                    },
                    workspace,
                );
                // One of the parent must be checked
                if let Some(parent_check) = parent_check {
                    workspace.add_unsolved_check(&parent_check.to_dyn().unwrap());
                }

                // Collect all references that are about to be dropped

                self.collect_references(workspace);
                if let Some(parent) = parent {
                    symbol.write().set_parent(parent);
                }

                // Update the symbol
                *self = symbol;
                ControlFlow::Break(Ok(()))
            }
            false => ControlFlow::Continue(()),
        }
    }
}

impl<T, Y> UpdateStatic<T, Y> for Option<Symbol<Y>>
where
    T: Buildable + Queryable,
    Y: AstSymbol
        + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>
        + InvokeStackBuilder<T, Y>
        + CollectReferences,
{
    fn update<'a>(
        &'a mut self,
        start: usize,
        offset: isize,
        parent_check: Option<WeakSymbol>,
        workspace: &'a mut Workspace,
        document: &Document,
    ) -> ControlFlow<Result<(), Diagnostic>, ()> {
        match self {
            Some(symbol) => symbol.update(start, offset, parent_check, workspace, document),
            None => ControlFlow::Continue(()),
        }
    }
}

impl<T, Y> UpdateStatic<T, Y> for Vec<Symbol<Y>>
where
    T: Buildable + Queryable,
    Y: AstSymbol
        + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>
        + InvokeStackBuilder<T, Y>
        + CollectReferences,
{
    fn update<'a>(
        &mut self,
        start: usize,
        offset: isize,
        parent_check: Option<WeakSymbol>,
        workspace: &'a mut Workspace,
        document: &Document,
    ) -> ControlFlow<Result<(), Diagnostic>, ()> {
        for symbol in self.iter_mut() {
            match symbol.update(start, offset, parent_check.clone(), workspace, document) {
                ControlFlow::Break(result) => return ControlFlow::Break(result),
                ControlFlow::Continue(()) => continue,
            }
        }
        ControlFlow::Continue(())
    }
}

/// This trait is similar to [UpdateStatic], but is used for trait objects ([DynSymbol])
pub trait UpdateDynamic {
    fn dyn_update(
        &mut self,
        start: usize,
        offset: isize,
        parent_check: Option<WeakSymbol>,
        workspace: &mut Workspace,
        document: &Document,
    ) -> ControlFlow<Result<(), Diagnostic>, ()>;
}
