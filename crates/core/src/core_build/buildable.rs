/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/

use downcast_rs::{impl_downcast, Downcast};

use crate::{
    ast::WeakSymbol,
    core_ast::{core::AstSymbol, symbol::Symbol},
    document::Document,
    errors::AstError,
    parsers::Parsers,
};

use super::symbol::{MaybePendingSymbol, PendingSymbol};

/// Trait implemented by all builders created with the seq macro.
pub trait Buildable: Downcast {
    /// Creates a new instance of the builder.
    ///
    /// # Returns
    /// - `Some(Self)` if a valid builder can be created for the given capture.
    /// - `None` for enums if the corresponding variant is not found.
    fn new(query: &tree_sitter::Query, capture: &tree_sitter::QueryCapture) -> Option<Self>
    where
        Self: Sized;

    /// Attempts to add a symbol to the current builder using the provided capture.
    ///
    /// # Returns
    /// - `Ok(Some([PendingSymbol]))` if a symbol is successfully added.
    /// - `Ok(None)` if the capture does not match the expected query name.
    /// - `Err(AstError)` if an error occurs.
    fn add(
        &mut self,
        capture: &tree_sitter::QueryCapture,
        parsers: &'static Parsers,
        document: &Document,
    ) -> Result<Option<PendingSymbol>, AstError>;

    fn get_range(&self) -> std::ops::Range<usize>;

    fn get_query_index(&self) -> usize;
}

impl_downcast!(Buildable);

/// Trait representing the list of queries associated with a struct or enum.
///
/// - Structs have a single query.
/// - Enums have one query per variant.
pub trait Queryable {
    const QUERY_NAMES: &'static [&'static str];
}

/// A trait for injecting a parent relationship into an AST symbol.
///
/// This trait is used to establish parent-child relationships in the AST,
/// ensuring that newly created symbols are properly linked to their parent nodes.
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

/// A trait for adding symbols to builders created by the `#[seq]` macro.
pub trait AddSymbol {
    /// Adds a symbol to the builder.
    ///
    /// This method is invoked for each field in a [`Buildable`] when the [`Buildable::add`] method is called.
    fn add<Y: Buildable + Queryable>(
        &mut self,
        capture: &tree_sitter::QueryCapture,
        parsers: &'static Parsers,
        parent_name: &str,
        field_name: &str,
    ) -> Result<Option<PendingSymbol>, AstError>;
}

impl AddSymbol for MaybePendingSymbol {
    fn add<Y: Buildable + Queryable>(
        &mut self,
        capture: &tree_sitter::QueryCapture,
        parsers: &'static Parsers,
        parent_name: &str,
        field_name: &str,
    ) -> Result<Option<PendingSymbol>, AstError> {
        if self.is_some() {
            return Ok(None);
        }
        let name = parsers.core.capture_names()[capture.index as usize];
        if Y::QUERY_NAMES.contains(&name) {
            match self.as_ref() {
                Some(_) => {
                    return Ok(None);
                }
                None => match Y::new(&parsers.core, capture) {
                    Some(node) => {
                        self.swap(&mut node.into());
                        return Ok(self.as_ref().clone());
                    }
                    None => {
                        return Err(AstError::InvalidSymbol {
                            range: std::ops::Range {
                                start: capture.node.start_byte(),
                                end: capture.node.end_byte(),
                            },
                            field_name: field_name.to_string(),
                            parent_name: parent_name.to_string(),
                            query: parsers.core.capture_names()[capture.index as usize],
                        })
                    }
                },
            }
        }
        Ok(None)
    }
}

impl AddSymbol for Vec<PendingSymbol> {
    fn add<Y: Buildable + Queryable>(
        &mut self,
        capture: &tree_sitter::QueryCapture,
        parsers: &'static Parsers,
        parent_name: &str,
        field_name: &str,
    ) -> Result<Option<PendingSymbol>, AstError> {
        let name = parsers.core.capture_names()[capture.index as usize];
        if Y::QUERY_NAMES.contains(&name) {
            match Y::new(&parsers.core, capture) {
                Some(node) => {
                    let node = PendingSymbol::new(node);
                    self.push(node.clone());
                    return Ok(Some(node));
                }
                None => {
                    return Err(AstError::InvalidSymbol {
                        range: std::ops::Range {
                            start: capture.node.start_byte(),
                            end: capture.node.end_byte(),
                        },
                        field_name: field_name.to_string(),
                        parent_name: parent_name.to_string(),
                        query: parsers.core.capture_names()[capture.index as usize],
                    })
                }
            }
        }
        Ok(None)
    }
}

/// Finalize trait to convert AST symbols to [`Symbol`]s.
pub trait Finalize<T: AstSymbol> {
    type Output;

    /// Converts the AST symbol to a [`Symbol`].
    fn finalize(self) -> Self::Output;
}

impl<T: AstSymbol> Finalize<T> for T {
    type Output = Symbol<T>;

    fn finalize(self) -> Self::Output {
        Symbol::from(self)
    }
}

impl<T: AstSymbol> Finalize<T> for Option<T> {
    type Output = Option<Symbol<T>>;

    fn finalize(self) -> Self::Output {
        self.map(|symbol| Symbol::from(symbol))
    }
}

impl<T: AstSymbol> Finalize<T> for Vec<T> {
    type Output = Vec<Symbol<T>>;

    fn finalize(self) -> Self::Output {
        self.into_iter().map(|f| Symbol::from(f)).collect()
    }
}
