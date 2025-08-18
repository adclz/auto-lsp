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

use crate::ast::AstNodeId;
use crate::errors::ParseErrorAccumulator;
use crate::{ast::AstNode, errors::AstError};
use salsa::Accumulator;
use std::ops::ControlFlow;
use std::sync::Arc;
use tree_sitter::{Node, TreeCursor};

/// Parameters for [`TryFrom`] implementations of AST nodes.
pub type TryFromParams<'from> = (
    &'from Node<'from>,         // Last node to convert
    &'from dyn salsa::Database, // Salsa Database
    &'from mut Builder,         // Builder
    usize,                      // Node ID (incremented by Builder struct)
    Option<usize>,              // Parent ID (if any)
);

/// A builder for creating AST nodes during the parsing process.
///
/// This struct is responsible for assigning unique IDs to nodes and storing them.
/// The ID counter is incremented every time a node is created, which may result in a gap
/// between the number of created IDs and the final number of successfully built nodes.
#[derive(Default)]
pub struct Builder {
    id_ctr: usize,
    nodes: Vec<Arc<dyn AstNode>>,
}

impl Builder {
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn take_nodes(self) -> Vec<Arc<dyn AstNode>> {
        self.nodes
    }

    fn next_id(&mut self) -> usize {
        self.id_ctr += 1;
        self.id_ctr
    }

    /// Creates a new AST node of type `T` from the current cursor position.
    ///
    /// The node is built using the [`AstNode`] try_from implementation for `T`.
    fn create<'db, T: AstNode + for<'from> TryFrom<TryFromParams<'from>, Error = AstError>>(
        &mut self,
        db: &'db dyn salsa::Database,
        cursor: &TreeCursor,
        parent_id: Option<usize>,
    ) -> Result<AstNodeId<T>, AstError> {
        let node = cursor.node();
        // Gets the next ID for the new node
        let id = self.next_id();
        let result = T::try_from((&node, db, self, id, parent_id)).map(Arc::new)?;
        // Stores the node
        self.nodes.push(result.clone());
        Ok(AstNodeId::new(id))
    }

    /// Starts a [`TreeWalk`] traversal using the given closure.
    pub fn builder<'cursor, F>(
        &'cursor mut self,
        db: &'cursor dyn salsa::Database,
        node: &'cursor Node<'cursor>,
        parent: Option<usize>,
        mut f: F,
    ) where
        F: for<'cb> FnMut(
            &'cb mut TreeWalk<'cursor>,
        ) -> ControlFlow<(), &'cb mut TreeWalk<'cursor>>,
    {
        TreeWalk::new(self, db, node, parent).walk(&mut f);
    }
}

/// A struct that walks through the current tree and calls the provided closure while traversing the tree.
pub struct TreeWalk<'cursor> {
    db: &'cursor dyn salsa::Database,
    builder: &'cursor mut Builder, // Builder instance
    cursor: TreeCursor<'cursor>,   // cursor (initialized at the beginning with the root node)
    parent: Option<usize>,
}

impl<'cursor> TreeWalk<'cursor> {
    fn new(
        builder: &'cursor mut Builder,
        db: &'cursor dyn salsa::Database,
        node: &'cursor Node<'cursor>,
        parent: Option<usize>,
    ) -> Self {
        let cursor = node.walk();
        Self {
            builder,
            db,
            cursor,
            parent,
        }
    }

    /// Recursively walks the tree starting from the root node.
    ///
    /// The provided closure is called on each child. If the closure returns
    /// [`ControlFlow::Break`], traversal stops at that node. Otherwise, it continues.
    fn walk<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Self) -> ControlFlow<(), &mut Self>,
    {
        if self.cursor.goto_first_child() {
            let _ = f(self);

            while self.cursor.goto_next_sibling() {
                let _ = f(self);
            }
        }
    }
    /// Attempts to create an AST node of type `T` if the current cursor points to a field with the given ID.
    ///
    /// If the field ID matches, the node is built and stored in `result`. Parsing then stops at this node.
    /// Returns [`ControlFlow::Break`] if a match is found and parsed, or `Continue` otherwise.
    pub fn on_field_id<
        T: AstNode + for<'from> TryFrom<TryFromParams<'from>, Error = AstError>,
        const FIELD_ID: u16,
    >(
        &mut self,
        result: &mut Result<Option<AstNodeId<T>>, AstError>,
    ) -> ControlFlow<(), &mut Self> {
        if let Some(field) = self.cursor.field_id() {
            if field == std::num::NonZero::new(FIELD_ID).expect("FIELD_ID should be non-zero") {
                *result = self
                    .builder
                    .create(self.db, &self.cursor, self.parent)
                    .map(Some);
                return ControlFlow::Break(());
            }
        }
        ControlFlow::Continue(self)
    }

    /// Like [`Self::on_field_id`], but collects all matching nodes into a vector.
    ///
    /// Parsing errors are accumulated into the database via [`ParseErrorAccumulator`] instead of propagating them.
    pub fn on_vec_field_id<
        T: AstNode + for<'from> TryFrom<TryFromParams<'from>, Error = AstError>,
        const FIELD_ID: u16,
    >(
        &mut self,
        result: &mut Vec<AstNodeId<T>>,
    ) -> ControlFlow<(), &mut Self> {
        if let Some(field) = self.cursor.field_id() {
            if field == std::num::NonZero::new(FIELD_ID).expect("FIELD_ID should be non-zero") {
                match self.builder.create(self.db, &self.cursor, self.parent) {
                    Ok(node) => result.push(node),
                    // Instead of propagating the error, we accumulate it in the database.
                    // This way we have a cheap "fault tolerant" parser.
                    Err(e) => ParseErrorAccumulator::accumulate(e.into(), self.db),
                };
                return ControlFlow::Break(());
            }
        }
        ControlFlow::Continue(self)
    }

    /// Attempts to create an AST node of type `T` if the current cursor matches the `T::contains` predicate.
    ///
    /// Useful for children without a specific field ID. The result is stored in `result`.
    pub fn on_children_id<
        T: AstNode + for<'from> TryFrom<TryFromParams<'from>, Error = AstError>,
    >(
        &mut self,
        result: &mut Result<Option<AstNodeId<T>>, AstError>,
    ) -> ControlFlow<(), &mut Self> {
        let node = self.cursor.node();
        if T::contains(&node) {
            *result = self
                .builder
                .create(self.db, &self.cursor, self.parent)
                .map(Some);
            return ControlFlow::Break(());
        }
        ControlFlow::Continue(self)
    }

    /// Like [`Self::on_children_id`], but collects all matching nodes into a vector.
    ///
    /// Errors are accumulated rather than returned, so the walk can continue through the tree.
    pub fn on_vec_children_id<
        T: AstNode + for<'from> TryFrom<TryFromParams<'from>, Error = AstError>,
    >(
        &mut self,
        result: &mut Vec<AstNodeId<T>>,
    ) -> ControlFlow<(), &mut Self> {
        if T::contains(&self.cursor.node()) {
            match self.builder.create(self.db, &self.cursor, self.parent) {
                Ok(node) => result.push(node),
                // Instead of propagating the error, we accumulate it in the database.
                // This way we have a cheap "fault tolerant" parser.
                Err(e) => ParseErrorAccumulator::accumulate(e.into(), self.db),
            };
            return ControlFlow::Break(());
        }
        ControlFlow::Continue(self)
    }
}
