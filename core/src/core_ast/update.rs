//! This module provides traits for incrementally updating AST.
//!
//! The traits defined here enable:
//! - [`Parent`] Injecting parent relationships into symbols.
//! - [`UpdateStatic`] and [`UpdateDynamic`] Performing incremental updates on AST nodes based on offset changes.
//!
//! Note: Still under development.

use std::ops::ControlFlow;

use lsp_types::Diagnostic;
use tree_sitter::InputEdit;

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

pub enum ChangeReport {
    Replace(usize, &'static [&'static str]),
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
            ChangeReport::Replace(index, queries) => {
                log::info!("");
                log::info!(
                    "Update: replace in vec[{}] {}",
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

pub enum UpdateState {
    Found,
    InjectParent(Vec<DynSymbol>),
    Result(Result<(), Diagnostic>),
}

/// Adjusts the range of a symbol based on the given start position and offset.
///
/// - `start`: The position where the edit begins.
/// - `offset`: The amount by which the symbol's range should be adjusted. Positive values
///   extend the range, while negative values shrink it. The range is clamped to avoid
///   negative values.
pub(crate) fn edit_range(data: &mut SymbolData, start: usize, offset: isize) {
    if data.range.start >= start {
        // Entire range is after the offset; shift both start and end
        data.range.start = ((data.range.start as isize + offset).max(0)) as usize;
        data.range.end = ((data.range.end as isize + offset).max(0)) as usize;
    } else if data.range.end > start {
        // The offset occurs within the range; adjust only the end
        data.range.end = ((data.range.end as isize + offset).max(0)) as usize;
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
    fn adjust(
        &mut self,
        edit: Change,
        collect: &mut Vec<DynSymbol>,
        workspace: &mut Workspace,
        document: &Document,
    );

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
    fn adjust(
        &mut self,
        edit: Change,
        collect: &mut Vec<DynSymbol>,
        workspace: &mut Workspace,
        document: &Document,
    ) {
        let mut write = self.write();
        let data = write.get_mut_data();
        edit_range(
            data,
            edit.input_edit.start_byte,
            (edit
                .input_edit
                .new_end_byte
                .wrapping_sub(edit.input_edit.old_end_byte)) as isize,
        );
        if write.is_comment() {
            write.set_comment(None);
        }
        write.adjust(edit, collect, workspace, document);
    }

    fn update(
        &mut self,
        edit: Change,
        collect: &mut Vec<DynSymbol>,
        workspace: &mut Workspace,
        document: &Document,
    ) -> ControlFlow<UpdateState> {
        let read = self.read();
        match read.is_inside_offset(edit.input_edit.start_byte + edit.trim_start) {
            true => {
                drop(read);
                // Checks if no lower level symbols could be updated.
                match self.write().update(edit, collect, workspace, document) {
                    // if result, return
                    ControlFlow::Break(UpdateState::Result(r)) => {
                        ControlFlow::Break(UpdateState::Result(r))
                    }
                    // if inject parent, set parents and return Ok(())
                    ControlFlow::Break(UpdateState::InjectParent(children)) => {
                        children.iter().for_each(|f| {
                            f.write().set_parent(self.to_weak());
                        });
                        ControlFlow::Break(UpdateState::Result(Ok(())))
                    }
                    _ => ControlFlow::Break(UpdateState::Found),
                }
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
    fn adjust(
        &mut self,
        edit: Change,
        collect: &mut Vec<DynSymbol>,
        workspace: &mut Workspace,
        document: &Document,
    ) {
        self.as_mut().map(|f| {
            let mut write = f.write();
            let data = write.get_mut_data();
            edit_range(
                data,
                edit.input_edit.start_byte,
                (edit
                    .input_edit
                    .new_end_byte
                    .wrapping_sub(edit.input_edit.old_end_byte)) as isize,
            );
            if write.is_comment() {
                write.set_comment(None);
            }
            write.adjust(edit, collect, workspace, document)
        });
    }

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

impl<T, Y> UpdateStatic<T, Y> for Vec<Symbol<Y>>
where
    T: Buildable + Queryable,
    Y: AstSymbol
        + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>
        + InvokeParser<T, Y>,
{
    fn adjust(
        &mut self,
        change: Change,
        collect: &mut Vec<DynSymbol>,
        workspace: &mut Workspace,
        document: &Document,
    ) {
        let InputEdit {
            start_byte,
            old_end_byte,
            ..
        } = change.input_edit;

        // Temporary storage for nodes to remove
        let mut to_remove = vec![];

        if !change.is_whitespace {
            // First pass: Identify nodes to remove
            for (index, node) in self.iter_mut().enumerate() {
                let write = node.read(); // Use `read` here because we don't mutate in this loop
                let node_range = write.get_range();

                if node_range.start >= start_byte && node_range.end <= old_end_byte {
                    // Node is within the deletion range
                    to_remove.push(index);
                    collect.push(node.to_dyn()); // Convert node to dynamic symbol
                }
            }

            // Remove all nodes marked for deletion
            for index in to_remove.iter().rev() {
                self.remove(*index);
            }

            for index in to_remove.iter() {
                workspace
                    .changes
                    .push(ChangeReport::Remove(*index, T::QUERY_NAMES));
            }
        }

        // Second pass: Adjust the remaining nodes
        for node in self.iter_mut() {
            let mut write = node.write();
            let data = write.get_mut_data();
            edit_range(
                data,
                change.input_edit.start_byte,
                (change
                    .input_edit
                    .new_end_byte
                    .wrapping_sub(change.input_edit.old_end_byte)) as isize,
            );
            if write.is_comment() {
                write.set_comment(None);
            }
            write.adjust(change, collect, workspace, document);
        }
    }

    fn update(
        &mut self,
        edit: Change,
        collect: &mut Vec<DynSymbol>,
        workspace: &mut Workspace,
        document: &Document,
    ) -> ControlFlow<UpdateState> {
        let mut start_index = None;

        // Find the starting index of the affected range or return early if there's an error
        for (i, symbol) in self.iter_mut().enumerate() {
            match symbol.update(edit, collect, workspace, document) {
                ControlFlow::Continue(_) => continue,
                ControlFlow::Break(UpdateState::Result(result)) => {
                    return ControlFlow::Break(UpdateState::Result(result));
                }
                ControlFlow::Break(UpdateState::InjectParent(parent)) => {
                    return ControlFlow::Break(UpdateState::InjectParent(parent));
                }
                ControlFlow::Break(UpdateState::Found) => {
                    start_index = Some(i);
                    break;
                }
            }
        }

        let start_index = match start_index {
            Some(index) => index,
            None if self.is_empty() => {
                // Special case: No nodes in the vector
                let symbols = match Y::parse_symbols(
                    workspace,
                    document,
                    Some(edit.input_edit.start_byte..edit.input_edit.new_end_byte),
                ) {
                    Ok(symbols) => symbols,
                    Err(err) => return ControlFlow::Break(UpdateState::Result(Err(err))),
                };

                let mut dyn_symbols: Vec<DynSymbol> = vec![];
                symbols.into_iter().enumerate().for_each(|(i, symbol)| {
                    let symbol = Symbol::new_and_check(symbol, workspace);

                    dyn_symbols.push(symbol.to_dyn());

                    self.insert(i, symbol);
                    workspace
                        .changes
                        .push(ChangeReport::Insert(i, T::QUERY_NAMES));
                });

                return ControlFlow::Break(UpdateState::InjectParent(dyn_symbols));
            }
            None => {
                // Special case: Range is after the last node
                let last_node = self.last().unwrap();
                let last_range = last_node.read().get_range();
                let parent = last_node.read().get_parent();

                let start = last_range.end; // Start parsing from after the last node
                let end = edit.input_edit.new_end_byte;

                let symbols = match Y::parse_symbols(workspace, document, Some(start..end)) {
                    Ok(symbols) => symbols,
                    Err(err) => return ControlFlow::Break(UpdateState::Result(Err(err))),
                };

                symbols.into_iter().enumerate().for_each(|(_i, symbol)| {
                    let symbol = Symbol::new_and_check(symbol, workspace);

                    if let Some(parent) = &parent {
                        symbol.write().set_parent(parent.clone());
                    }

                    self.push(symbol); // Append to the vector
                    workspace
                        .changes
                        .push(ChangeReport::Insert(self.len() - 1, T::QUERY_NAMES));
                });

                return ControlFlow::Break(UpdateState::Result(Ok(())));
            }
        };
        let parent = self[start_index].read().get_parent();

        // Determine the number of affected nodes
        let affected_count = self[start_index..]
            .iter()
            .take_while(|symbol| {
                let range = symbol.read().get_range();
                edit.input_edit.start_byte <= range.start
                    && edit.input_edit.new_end_byte >= range.end
            })
            .count()
            .max(1);

        // Calculate affected range
        let start = self[start_index].read().get_range().start;
        let end = self[start_index + affected_count - 1]
            .read()
            .get_range()
            .end;

        // Remove affected nodes
        self.drain(start_index..start_index + affected_count);
        // Parse and insert new symbols
        let symbols = match Y::parse_symbols(workspace, document, Some(start..end)) {
            Ok(symbols) => symbols,
            Err(err) => return ControlFlow::Break(UpdateState::Result(Err(err))),
        };

        symbols.into_iter().enumerate().for_each(|(i, symbol)| {
            let symbol = Symbol::new_and_check(symbol, workspace);
            if let Some(parent) = &parent {
                symbol.write().set_parent(parent.clone());
            }

            self.insert(i + start_index, symbol);
            if (start_index..start_index + affected_count).contains(&(i + start_index)) {
                workspace
                    .changes
                    .push(ChangeReport::Replace(i + start_index, T::QUERY_NAMES));
            } else {
                workspace
                    .changes
                    .push(ChangeReport::Insert(i + start_index, T::QUERY_NAMES));
            }
        });

        ControlFlow::Break(UpdateState::Result(Ok(())))
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

    fn adjust(
        &mut self,
        edit: Change,
        collect: &mut Vec<DynSymbol>,
        workspace: &mut Workspace,
        document: &Document,
    );
}

#[cfg(test)]
mod range_tests {
    use super::{edit_range, SymbolData};
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
        edit_range(&mut data, 5, 5);
        assert_eq!(data.range, std::ops::Range { start: 10, end: 15 });

        let mut data = symbol_data_mock(std::ops::Range { start: 5, end: 10 });

        // start < range.start
        edit_range(&mut data, 2, 5);
        assert_eq!(data.range, std::ops::Range { start: 10, end: 15 });

        let mut data = symbol_data_mock(std::ops::Range { start: 5, end: 10 });

        // start > range.start
        edit_range(&mut data, 6, 5);
        assert_eq!(data.range, std::ops::Range { start: 5, end: 15 });
    }

    #[test]
    fn negative() {
        let mut data = symbol_data_mock(std::ops::Range { start: 5, end: 10 });

        // start == range.start
        edit_range(&mut data, 5, -5);
        assert_eq!(data.range, std::ops::Range { start: 0, end: 5 });

        let mut data = symbol_data_mock(std::ops::Range { start: 5, end: 10 });

        // start < range.start
        edit_range(&mut data, 2, -5);
        assert_eq!(data.range, std::ops::Range { start: 0, end: 5 });

        let mut data = symbol_data_mock(std::ops::Range { start: 5, end: 10 });

        // start > range.start
        edit_range(&mut data, 6, -5);
        assert_eq!(data.range, std::ops::Range { start: 5, end: 5 });
    }

    #[test]
    fn zero_offset() {
        let mut data = symbol_data_mock(std::ops::Range { start: 5, end: 10 });

        // No changes when offset is 0
        edit_range(&mut data, 5, 0);
        assert_eq!(data.range, std::ops::Range { start: 5, end: 10 });
    }

    #[test]
    fn empty_range() {
        let mut data = symbol_data_mock(std::ops::Range { start: 5, end: 5 });

        // Positive offset
        edit_range(&mut data, 5, 5);
        assert_eq!(data.range, std::ops::Range { start: 10, end: 10 });

        // Negative offset
        edit_range(&mut data, 5, -10);
        assert_eq!(data.range, std::ops::Range { start: 0, end: 0 });
    }

    #[test]
    fn extreme_offsets() {
        let mut data = symbol_data_mock(std::ops::Range { start: 5, end: 10 });

        // Very large positive offset
        edit_range(&mut data, 0, 1_000);
        assert_eq!(
            data.range,
            std::ops::Range {
                start: 1_005,
                end: 1_010
            }
        );

        // Very large negative offset, clamped to zero
        edit_range(&mut data, 0, -10_000);
        assert_eq!(data.range, std::ops::Range { start: 0, end: 0 });
    }

    #[test]
    fn overlapping_ranges() {
        let mut data = symbol_data_mock(std::ops::Range { start: 10, end: 20 });

        // Offset within the range
        edit_range(&mut data, 15, 5);
        assert_eq!(data.range, std::ops::Range { start: 10, end: 25 });

        // Offset shrinks the range end
        edit_range(&mut data, 15, -10);
        assert_eq!(data.range, std::ops::Range { start: 10, end: 15 });
    }
}
