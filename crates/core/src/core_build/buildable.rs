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

use super::symbol::{MaybePendingSymbol, PendingSymbol};
use crate::{core_ast::core::AstSymbol, document::Document, errors::AstError, parsers::Parsers};
use downcast_rs::{impl_downcast, Downcast};
use std::{collections::HashMap, sync::Arc};

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

    fn get_id(&self) -> usize;
}

impl_downcast!(Buildable);

/// Trait representing the list of queries associated with a struct or enum.
///
/// - Structs have a single query.
/// - Enums have one query per variant.
pub trait Queryable {
    const QUERY_NAMES: &'static [&'static str];
}

pub type TryFromParams<'a, T> = (
    &'a T,
    &'a Option<usize>,
    &'a Document,
    &'static Parsers,
    &'a HashMap<usize, usize>,
    &'a mut Vec<Arc<dyn AstSymbol>>,
);

/// A trait for adding symbols to builders created by the `#[seq]` macro.
pub trait AddSymbol {
    /// Adds a symbol to the builder.
    ///
    /// This method is invoked for each field in a [`Buildable`] when the [`Buildable::add`] method is called.
    fn add<Y: Buildable + Queryable>(
        &mut self,
        capture: &tree_sitter::QueryCapture,
        parsers: &'static Parsers,
    ) -> Result<Option<PendingSymbol>, AstError>;
}

impl AddSymbol for MaybePendingSymbol {
    fn add<Y: Buildable + Queryable>(
        &mut self,
        capture: &tree_sitter::QueryCapture,
        parsers: &'static Parsers,
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
                        return Ok(self.0.clone());
                    }
                    None => {
                        return Err(AstError::InvalidSymbol {
                            range: std::ops::Range {
                                start: capture.node.start_byte(),
                                end: capture.node.end_byte(),
                            },
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
                        query: parsers.core.capture_names()[capture.index as usize],
                    })
                }
            }
        }
        Ok(None)
    }
}
