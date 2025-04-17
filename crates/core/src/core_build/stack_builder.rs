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

use lsp_types::Diagnostic;
use lsp_types::Url;
use salsa::Accumulator;
use streaming_iterator::StreamingIterator;
use tree_sitter::QueryCapture;

use super::buildable::*;
use super::downcast::*;
use super::symbol::*;
use super::utils::{intersecting_ranges, tree_sitter_range_to_lsp_range};
use crate::document::Document;
use crate::parsers::Parsers;
use crate::salsa::db::BaseDatabase;
use crate::salsa::tracked::DiagnosticAccumulator;
use crate::{builder_error, builder_warning, core_ast::core::AstSymbol};

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
    url: &'a Arc<Url>,
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
        parsers: &'static Parsers,
        url: &'a Arc<Url>,
        document: &'a Document,
    ) -> Self {
        Self {
            _meta: PhantomData,
            db,
            parsers,
            url,
            document,
            roots: vec![],
            stack: vec![],
        }
    }

    /// Creates a symbol of type [`Y`] based on the specified range.
    ///
    /// This method builds the AST for the provided range (if any) and attempts to derive
    /// a symbol from the root node.
    pub fn create_symbol<Y>(&mut self) -> Result<Y, Diagnostic>
    where
        Y: AstSymbol + for<'c> TryFromBuilder<&'c T, Error = lsp_types::Diagnostic>,
    {
        self.build();
        let result = self.get_root_node()?;
        let result = result.0.borrow();
        let result = result
            .downcast_ref::<T>()
            .ok_or(builder_error!(
                result
                    .get_lsp_range(self.document)
                    .expect("Failed to convert LSP range when building root symbol"),
                format!("Internal error: Could not cast {:?}", T::QUERY_NAMES)
            ))?
            .try_into_builder(self.parsers, self.url, self.document)?;
        #[cfg(feature = "log")]
        log::debug!("\n{}", result);
        Ok(result)
    }

    /// Builds the AST based on Tree-sitter query captures.
    ///
    /// If a range is specified, only the portion of the document within that range
    /// is processed. Captures are iterated in the order they appear in the tree.
    fn build(&mut self) -> &mut Self {
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
            let capture_index = capture.index as usize;

            // Current parent
            let mut parent = self.stack.pop();

            loop {
                match &parent {
                    // If there's no parent, create a root node.
                    None => {
                        // There can only be one root node.
                        if self.roots.is_empty() {
                            self.create_root_node(&capture, capture_index);
                            break;
                        } else {
                            return self;
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
        self
    }

    /// Creates the root node of the AST.
    ///
    /// The root node is the top-level symbol in the AST, and only one root node can exist.
    fn create_root_node(&mut self, capture: &QueryCapture, capture_index: usize) {
        let mut node = T::new(&self.parsers.core, capture);

        match node.take() {
            Some(builder) => {
                let node = PendingSymbol::new(builder);
                self.roots.push(node.clone());
                self.stack.push(node);
            }
            None => DiagnosticAccumulator::accumulate(
                builder_warning!(
                    tree_sitter_range_to_lsp_range(&capture.node.range()),
                    format!(
                        "Syntax error: Unexpected {:?}",
                        self.parsers.core.capture_names()[capture_index],
                    )
                )
                .into(),
                self.db,
            ),
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
                DiagnosticAccumulator::accumulate(e.into(), self.db);
            }
            Ok(None) => {
                // Parent did not accept the child node.
                DiagnosticAccumulator::accumulate(
                    builder_warning!(
                        tree_sitter_range_to_lsp_range(&capture.node.range()),
                        format!(
                            "Syntax error: Unexpected {:?} in {:?}",
                            self.parsers.core.capture_names()[capture.index as usize],
                            self.parsers.core.capture_names()[parent.0.borrow().get_query_index()],
                        )
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

    /// Attempt to retrieve root node, initially created with [`Self::create_root_node`].
    ///
    /// If no root node exists, an error is returned indicating the expected query names.
    fn get_root_node(&mut self) -> Result<PendingSymbol, Diagnostic> {
        // Root node is the last node in the stack.
        match self.roots.pop() {
            Some(node) => Ok(node),
            None => {
                let expected = T::QUERY_NAMES
                    .iter()
                    .map(|name| name.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                Err(builder_error!(
                    lsp_types::Range::default(),
                    format!("Expected one of: {}", expected)
                ))
            }
        }
    }
}
