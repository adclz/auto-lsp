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

use std::marker::PhantomData;
use std::sync::Arc;

use id_arena::Arena;
use salsa::Accumulator;
use streaming_iterator::StreamingIterator;
use tree_sitter::QueryCapture;

use super::buildable::*;
use super::symbol::*;
use super::utils::intersecting_ranges;
use crate::core_ast::core::AstSymbol;
use crate::document::Document;
use crate::errors::AstError;
use crate::errors::ParseError;
use crate::errors::ParseErrorAccumulator;
use crate::parsers::Parsers;
use crate::salsa::db::BaseDatabase;

/// Stack builder for constructing Abstract Syntax Trees (ASTs).
///
/// This struct is responsible for building ASTs using Tree-sitter queries.
/// Inspired by [vscode anycode](https://github.com/microsoft/vscode-anycode/blob/17190d5b94850095b7ecd7cc37170324dfe08e0e/anycode/server/src/common/features/documentSymbols.ts#L50).
pub struct StackBuilder<'a, T>
where
    T: Buildable + Queryable,
{
    _meta: PhantomData<T>,
    db: &'a dyn BaseDatabase,
    /// Parsers used for building the AST
    parsers: &'static Parsers,
    document: &'a Document,
    /// Symbols created while building the AST
    roots: Vec<PendingSymbol>,
    stack: Vec<PendingSymbol>,
}

impl<'a, T> StackBuilder<'a, T>
where
    T: Buildable + Queryable,
{
    /// Creates a new `StackBuilder` instance.
    pub fn new(
        db: &'a dyn BaseDatabase,
        document: &'a Document,
        parsers: &'static Parsers,
    ) -> Self {
        Self {
            _meta: PhantomData,
            db,
            parsers,
            document,
            roots: vec![],
            stack: vec![],
        }
    }

    /// Creates a symbol of type [`Y`] based on the specified range.
    ///
    /// This method builds the AST for the provided range (if any) and attempts to derive
    /// a symbol from the root node.
    pub fn create_symbol<Y>(&mut self) -> Result<(Y, Arena<Arc<dyn AstSymbol>>), ParseError>
    where
        Y: AstSymbol
            + for<'c> TryFrom<
                (
                    &'c T,
                    &'c Document,
                    &'static Parsers,
                    &'c mut Arena<Arc<dyn AstSymbol>>,
                ),
                Error = AstError,
            >,
    {
        let mut arena = Arena::<Arc<dyn AstSymbol>>::new();
        let result = self.build()?.ok_or::<ParseError>(
            (
                self.document,
                AstError::NoRootNode {
                    range: std::ops::Range {
                        start: 0,
                        end: self.document.texter.text.len(),
                    },
                    query: T::QUERY_NAMES,
                },
            )
                .into(),
        )?;
        let result = Y::try_from((
            result.0.borrow().downcast_ref::<T>().unwrap(),
            self.document,
            self.parsers,
            &mut arena,
        ))
        .map_err(|err| ParseError::from((self.document, err)))?;
        #[cfg(feature = "log")]
        log::debug!("\n{}", result);
        Ok((result, arena))
    }

    /// Builds the AST based on Tree-sitter query captures.
    ///
    /// If a range is specified, only the portion of the document within that range
    /// is processed. Captures are iterated in the order they appear in the tree.
    fn build(&mut self) -> Result<Option<PendingSymbol>, ParseError> {
        let mut cursor = tree_sitter::QueryCursor::new();

        let mut captures = cursor.captures(
            &self.parsers.core,
            self.document.tree.root_node(),
            self.document.texter.text.as_bytes(),
        );

        // Iterate over the captures.
        // Captures are sorted by their location in the tree, not their pattern.
        while let Some((m, capture_index)) = captures.next() {
            let capture = m.captures[*capture_index];

            // Current parent
            let mut parent = self.stack.pop();

            loop {
                match &parent {
                    // If there's no parent, create a root node.
                    None => {
                        // There can only be one root node.
                        if self.roots.is_empty() {
                            self.create_root_node(&capture)?;
                            break;
                        } else {
                            return Ok(self.roots.pop());
                        }
                    }
                    // If there's a parent, checks if the parent's range intersects with the current capture.
                    Some(p) => {
                        if intersecting_ranges(&p.0.borrow().get_range(), &capture.node.range()) {
                            // If it intersects, create a child node.
                            self.create_child_node(p, &capture);
                            break;
                        }
                    }
                }
                parent = self.stack.pop();
            }
        }
        Ok(self.roots.pop())
    }

    /// Creates the root node of the AST.
    ///
    /// The root node is the top-level symbol in the AST, and only one root node can exist.
    fn create_root_node(&mut self, capture: &QueryCapture) -> Result<(), ParseError> {
        let mut node = T::new(&self.parsers.core, capture);

        match node.take() {
            Some(builder) => {
                let node = PendingSymbol::new(builder);
                self.roots.push(node.clone());
                self.stack.push(node);
                Ok(())
            }
            None => Err((
                self.document,
                AstError::NoRootNode {
                    range: std::ops::Range {
                        start: capture.node.start_byte(),
                        end: capture.node.end_byte(),
                    },
                    query: T::QUERY_NAMES,
                },
            )
                .into()),
        }
    }

    /// Creates a child node and tries to add it to the parent node.
    fn create_child_node(&mut self, parent: &PendingSymbol, capture: &QueryCapture) {
        let add = parent
            .0
            .borrow_mut()
            .add(capture, self.parsers, self.document);

        match add {
            Err(e) => {
                // Parent did not accept the child node and returned an error.
                ParseErrorAccumulator::accumulate((self.document, e).into(), self.db);
            }
            Ok(None) => {
                // Parent did not accept the child node.
                ParseErrorAccumulator::accumulate(
                    (
                        self.document,
                        AstError::UnknownSymbol {
                            range: std::ops::Range {
                                start: capture.node.start_byte(),
                                end: capture.node.end_byte(),
                            },
                            symbol: self.parsers.core.capture_names()[capture.index as usize],
                            parent_name: self.parsers.core.capture_names()
                                [parent.0.borrow().get_query_index()],
                        },
                    )
                        .into(),
                    self.db,
                );
            }
            Ok(Some(node)) => {
                self.stack.push(parent.clone());
                self.stack.push(node.clone());
            }
        };
    }
}
