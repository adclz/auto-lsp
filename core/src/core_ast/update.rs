//! This module provides traits for incrementally updating AST.
//!
//! The traits defined here enable:
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
use crate::core_build::parse::InvokeParser;
use crate::document::texter_impl::updateable::Change;
use crate::document::Document;
use crate::workspace::Workspace;

use super::core::AstSymbol;
use super::data::*;
use super::symbol::*;

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
///   extend the range, while negative values shrink it. The range is clamped to avoid
///   negative values.
pub(crate) fn edit(data: &mut SymbolData, start: usize, offset: isize) {
    if data.range.start >= start {
        // Entire range is after the offset; shift both start and end
        data.range.start = ((data.range.start as isize + offset).max(0)) as usize;
        data.range.end = ((data.range.end as isize + offset).max(0)) as usize;
    } else if data.range.end >= start {
        // The offset occurs within the range; adjust only the end
        data.range.end = ((data.range.end as isize + offset).max(0)) as usize;
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

pub enum UpdateState {
    Found,
    Result(Result<(), Diagnostic>),
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
    /// ### Tries to locate a symbol and update it.
    /// - The symbol must be within the range of the edit.
    /// - No lower-level symbols (children) have already been updated.
    ///
    /// ### Return Value
    /// The method returns a [ControlFlow] to indicate the result:
    /// - [ControlFlow::Break]: The symbol was successfully updated, with an optional diagnostic.
    /// - [ControlFlow::Continue]: The symbol did not require updating.
    fn update(
        &mut self,
        edit: Change,
        collect: &mut Vec<DynSymbol>,
        workspace: &mut Workspace,
        document: &Document,
    ) -> ControlFlow<UpdateState>;
}

impl<T, Y> UpdateStatic<T, Y> for Symbol<Y>
where
    T: Buildable + Queryable,
    Y: AstSymbol
        + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>
        + InvokeParser<T, Y>,
{
    fn update(
        &mut self,
        edit: Change,
        collect: &mut Vec<DynSymbol>,
        workspace: &mut Workspace,
        document: &Document,
    ) -> ControlFlow<UpdateState> {
        let read = self.read();
        match read.is_inside_offset(edit.input_edit.start_byte) {
            true => {
                drop(read);
                // Checks if no lower level symbols could be updated.
                self.write().update(edit, collect, workspace, document)?;
                // Returns that the symbol was found
                return ControlFlow::Break(UpdateState::Found);
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
        + InvokeParser<T, Y>,
{
    fn update(
        &mut self,
        edit: Change,
        collect: &mut Vec<DynSymbol>,
        workspace: &mut Workspace,
        document: &Document,
    ) -> ControlFlow<UpdateState> {
        match self {
            Some(symbol) => symbol.update(edit, collect, workspace, document),
            None => ControlFlow::Continue(()),
        }
    }
}

pub enum ChangeReport {
    Insert(usize, &'static [&'static str]),
    Remove(usize, &'static [&'static str]),
}

impl ChangeReport {
    #[cfg(feature = "log")]
    pub(crate) fn log(&self) {
        match self {
            ChangeReport::Insert(index, queries) => {
                log::info!("");
                log::info!(
                    "Update: insert in vec[{}] {}",
                    index,
                    match queries.len() > 1 {
                        true => format!("of one of {:?}", queries),
                        false => format!("of {:?}", queries[0]),
                    }
                );
                log::info!("");
            }
            ChangeReport::Remove(index, queries) => {
                log::info!("");
                log::info!(
                    "Update: remove in vec[{}] {}",
                    index,
                    match queries.len() > 1 {
                        true => format!("of one of {:?}", queries),
                        false => format!("of {:?}", queries[0]),
                    }
                );
                log::info!("");
            }
        }
    }
}

impl<T, Y> UpdateStatic<T, Y> for Vec<Symbol<Y>>
where
    T: Buildable + Queryable,
    Y: AstSymbol
        + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>
        + InvokeParser<T, Y>,
{
    fn update(
        &mut self,
        edit: Change,
        collect: &mut Vec<DynSymbol>,
        workspace: &mut Workspace,
        document: &Document,
    ) -> ControlFlow<UpdateState> {
        let mut affected: Option<usize> = None;
        for (i, symbol) in self.iter_mut().enumerate() {
            match symbol.update(edit, collect, workspace, document) {
                ControlFlow::Continue(_) => continue,
                ControlFlow::Break(UpdateState::Result(result)) => {
                    return ControlFlow::Break(UpdateState::Result(result))
                }
                ControlFlow::Break(UpdateState::Found) => {
                    affected = Some(i);
                    break;
                }
            }
        }

        match affected {
            None => ControlFlow::Continue(()),
            Some(start_index) => {
                let parent = self[start_index].read().get_parent();

                // Determine the range of affected nodes starting from start_index
                let mut affected_count = 0;
                for symbol in &self[start_index..] {
                    let range = symbol.read().get_range();
                    if edit.input_edit.start_byte <= range.start
                        && edit.input_edit.new_end_byte >= range.end
                    {
                        affected_count += 1;
                    } else {
                        break;
                    }
                }

                // If no additional nodes are affected, ensure the affected count is at least 1
                if affected_count == 0 {
                    affected_count = 1;
                }

                // Calculate start and end positions of the affected range
                let start = self[start_index].read().get_range().start;
                let end = self[start_index + affected_count - 1]
                    .read()
                    .get_range()
                    .end;

                // Remove deprecated symbols, starting from start_index
                for i in (start_index..start_index + affected_count).rev() {
                    workspace
                        .changes
                        .push(ChangeReport::Remove(i, T::QUERY_NAMES));
                    self.remove(i);
                }

                // Parse and insert new symbols in the affected range
                let symbols = match Y::parse_symbols(
                    workspace,
                    document,
                    Some(std::ops::Range { start, end }),
                ) {
                    Ok(symbols) => symbols,
                    Err(err) => return ControlFlow::Break(UpdateState::Result(Err(err))),
                };

                symbols.into_iter().enumerate().for_each(|(i, symbol)| {
                    let symbol = Symbol::new_and_check(symbol, workspace);
                    if let Some(parent) = &parent {
                        symbol.write().set_parent(parent.clone());
                    }

                    self.insert(i + start_index, symbol);
                    workspace
                        .changes
                        .push(ChangeReport::Insert(i + start_index, T::QUERY_NAMES));
                });

                ControlFlow::Break(UpdateState::Result(Ok(())))
            }
        }
    }
}

/// This trait is similar to [UpdateStatic], but is used for trait objects ([DynSymbol])
pub trait UpdateDynamic {
    fn update(
        &mut self,
        edit: Change,
        collect: &mut Vec<DynSymbol>,
        workspace: &mut Workspace,
        document: &Document,
    ) -> ControlFlow<UpdateState>;
}

#[cfg(test)]
mod range_tests {
    use super::{edit, SymbolData};
    use lsp_types::Url;
    use std::sync::Arc;

    fn symbol_data_mock(range: std::ops::Range<usize>) -> SymbolData {
        SymbolData {
            range,
            url: Arc::new(Url::parse("file:///test").unwrap()),
            parent: None,
            comment: None,
            referrers: None,
            target: None,
            check_pending: false,
        }
    }

    #[test]
    fn positive() {
        let mut data = symbol_data_mock(std::ops::Range { start: 5, end: 10 });

        // start == range.start
        edit(&mut data, 5, 5);
        assert_eq!(data.range, std::ops::Range { start: 10, end: 15 });

        let mut data = symbol_data_mock(std::ops::Range { start: 5, end: 10 });

        // start < range.start
        edit(&mut data, 2, 5);
        assert_eq!(data.range, std::ops::Range { start: 10, end: 15 });

        let mut data = symbol_data_mock(std::ops::Range { start: 5, end: 10 });

        // start > range.start
        edit(&mut data, 6, 5);
        assert_eq!(data.range, std::ops::Range { start: 5, end: 15 });
    }

    #[test]
    fn negative() {
        let mut data = symbol_data_mock(std::ops::Range { start: 5, end: 10 });

        // start == range.start
        edit(&mut data, 5, -5);
        assert_eq!(data.range, std::ops::Range { start: 0, end: 5 });

        let mut data = symbol_data_mock(std::ops::Range { start: 5, end: 10 });

        // start < range.start
        edit(&mut data, 2, -5);
        assert_eq!(data.range, std::ops::Range { start: 0, end: 5 });

        let mut data = symbol_data_mock(std::ops::Range { start: 5, end: 10 });

        // start > range.start
        edit(&mut data, 6, -5);
        assert_eq!(data.range, std::ops::Range { start: 5, end: 5 });
    }

    #[test]
    fn zero_offset() {
        let mut data = symbol_data_mock(std::ops::Range { start: 5, end: 10 });

        // No changes when offset is 0
        edit(&mut data, 5, 0);
        assert_eq!(data.range, std::ops::Range { start: 5, end: 10 });
    }

    #[test]
    fn empty_range() {
        let mut data = symbol_data_mock(std::ops::Range { start: 5, end: 5 });

        // Positive offset
        edit(&mut data, 5, 5);
        assert_eq!(data.range, std::ops::Range { start: 10, end: 10 });

        // Negative offset
        edit(&mut data, 5, -10);
        assert_eq!(data.range, std::ops::Range { start: 0, end: 0 });
    }

    #[test]
    fn extreme_offsets() {
        let mut data = symbol_data_mock(std::ops::Range { start: 5, end: 10 });

        // Very large positive offset
        edit(&mut data, 0, 1_000);
        assert_eq!(
            data.range,
            std::ops::Range {
                start: 1_005,
                end: 1_010
            }
        );

        // Very large negative offset, clamped to zero
        edit(&mut data, 0, -10_000);
        assert_eq!(data.range, std::ops::Range { start: 0, end: 0 });
    }

    #[test]
    fn overlapping_ranges() {
        let mut data = symbol_data_mock(std::ops::Range { start: 10, end: 20 });

        // Offset within the range
        edit(&mut data, 15, 5);
        assert_eq!(data.range, std::ops::Range { start: 10, end: 25 });

        // Offset shrinks the range end
        edit(&mut data, 15, -10);
        assert_eq!(data.range, std::ops::Range { start: 10, end: 15 });
    }
}
