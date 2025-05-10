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

use std::{sync::Arc};
use tree_sitter::{Node, TreeCursor};
use crate::{ast::AstNode, errors::AstError};

pub type TryFromParams<'a> = (&'a Node<'a>, &'a mut Builder, usize, Option<usize>);

#[derive(Default)]
pub struct Builder {
    ids: usize,
    nodes: Vec<Arc<dyn AstNode>>,
}

impl Builder {
    pub fn take_nodes(self) -> Vec<Arc<dyn AstNode>> {
        self.nodes
    }

    fn next_id(&mut self) -> usize {
        self.ids += 1;
        self.ids
    }

    fn create<T: AstNode + for<'a> TryFrom<TryFromParams<'a>, Error = AstError>>(&mut self, node: &Node, parent_id: Option<usize>) -> Result<Arc<T>, AstError> {
        let id = self.next_id();
        let result = Arc::new(T::try_from((&node, self, id, parent_id))?);
        self.nodes.push(result.clone());
        Ok(result)
    }

    pub fn builder<'a>(&'a mut self, node: &'a Node<'a>, parent: Option<usize>) -> NestedBuilder<'a> {
        NestedBuilder::new(self, node, parent)
    }
}

pub struct NestedBuilder<'a> {
    pub parent: Option<usize>,
    pub builder: &'a mut Builder,
    pub node: &'a Node<'a>,
    pub cursor: TreeCursor<'a>,
}

impl<'a> NestedBuilder<'a> {
    pub fn new(builder: &'a mut Builder, node: &'a Node<'a>, parent: Option<usize>) -> Self {
        let mut cursor = node.walk();
        Self { builder, node, cursor, parent }
    }

    pub fn walk<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut NestedBuilder<'a>),
    {
        while self.cursor.goto_first_child() {
            f(self);

            while self.cursor.goto_next_sibling() {
                f(self);
            }

            break;
        }
    }

    pub fn on_field_id<T: AstNode + for<'b> TryFrom<TryFromParams<'b>, Error = AstError>, const FIELD_ID: u16>(&mut self, result: &mut Result<Option<Arc<T>>, AstError>)  {
        if let Some(field) = self.cursor.field_id()  {
            if field == std::num::NonZero::new(FIELD_ID).unwrap() {
                *result = self.builder.create(&self.cursor.node(), self.parent).map(Some);
            }
        }
    }

    pub fn on_vec_field_id<T: AstNode + for<'b> TryFrom<TryFromParams<'b>, Error = AstError>, const FIELD_ID: u16>(&mut self, result: &mut Vec<Arc<T>>)  {
        if let Some(field) = self.cursor.field_id()  {
            if field == std::num::NonZero::new(FIELD_ID).unwrap() {
                result.push(self.builder.create(&self.cursor.node(), self.parent).unwrap());
            }
        }
    }

    pub fn on_children_id<T: AstNode + for<'b> TryFrom<TryFromParams<'b>, Error = AstError>>(&mut self, result: &mut Result<Option<Arc<T>>, AstError>)  {
        if T::contains(&self.cursor.node()) {
            *result = self.builder.create(&self.cursor.node(), self.parent).map(Some);
        }
    }

    pub fn on_vec_children_id<T: AstNode + for<'b> TryFrom<TryFromParams<'b>, Error = AstError>>(&mut self, result: &mut Vec<Arc<T>>)  {
        if T::contains(&self.cursor.node()) {
            result.push(self.builder.create(&self.cursor.node(), self.parent).unwrap());
        }
    }
}
