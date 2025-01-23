use std::marker::PhantomData;
use std::sync::Arc;

use lsp_types::{Diagnostic, Position};
use streaming_iterator::StreamingIterator;
use tree_sitter::QueryCapture;

use super::buildable::*;
use super::downcast::*;
use super::symbol::*;
use super::utils::{intersecting_ranges, tree_sitter_range_to_lsp_range};
use crate::ast::DynSymbol;
use crate::document::Document;
use crate::workspace::Workspace;
use crate::{builder_error, builder_warning, core_ast::core::AstSymbol};

pub trait InvokeStackBuilder<
    T: Buildable + Queryable,
    Y: AstSymbol + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
>
{
    fn create_symbol(
        workspace: &mut Workspace,
        document: &Document,
        range: Option<std::ops::Range<usize>>,
    ) -> Result<Y, Diagnostic>;
}

impl<T, Y> InvokeStackBuilder<T, Y> for Y
where
    T: Buildable + Queryable,
    Y: AstSymbol + for<'b> TryFromBuilder<&'b T, Error = lsp_types::Diagnostic>,
{
    fn create_symbol(
        workspace: &mut Workspace,
        document: &Document,
        range: Option<std::ops::Range<usize>>,
    ) -> Result<Y, Diagnostic> {
        StackBuilder::<T>::new(workspace, document).create_symbol(&range)
    }
}
pub type InvokeStackBuilderFn = fn(
    &mut Workspace,
    &Document,
    Option<std::ops::Range<usize>>,
) -> Result<DynSymbol, lsp_types::Diagnostic>;

pub struct StackBuilder<'a, T>
where
    T: Buildable + Queryable,
{
    _meta: PhantomData<T>,
    workspace: &'a mut Workspace,
    document: &'a Document,
    roots: Vec<PendingSymbol>,
    stack: Vec<PendingSymbol>,
    start_building: bool,
}

impl<'a, T> StackBuilder<'a, T>
where
    T: Buildable + Queryable,
{
    pub fn new(workspace: &'a mut Workspace, document: &'a Document) -> Self {
        Self {
            _meta: PhantomData,
            workspace,
            document,
            roots: vec![],
            stack: vec![],
            start_building: false,
        }
    }

    pub fn create_symbol<Y>(
        &mut self,
        range: &Option<std::ops::Range<usize>>,
    ) -> Result<Y, Diagnostic>
    where
        Y: AstSymbol + for<'c> TryFromBuilder<&'c T, Error = lsp_types::Diagnostic>,
    {
        self.build(range);
        let result = self.get_root_node(range)?;
        let result = result.get_rc().borrow();
        result
            .downcast_ref::<T>()
            .ok_or(builder_error!(
                result.get_lsp_range(&self.document),
                format!("Invalid cast {:?}", T::QUERY_NAMES[0])
            ))?
            .try_into_builder(self.workspace, self.document)
    }

    fn build(&mut self, range: &Option<std::ops::Range<usize>>) -> &mut Self {
        let mut cursor = tree_sitter::QueryCursor::new();

        let mut captures = cursor.captures(
            &self.workspace.parsers.tree_sitter.queries.core,
            self.document.tree.root_node(),
            self.document.texter.text.as_bytes(),
        );

        if let Some(range) = range {
            captures.set_byte_range(range.clone());
        }

        while let Some((m, capture_index)) = captures.next() {
            let capture = Arc::new(m.captures[*capture_index]);
            let capture_index = capture.index as usize;

            if !self.start_building {
                if let Some(range) = range {
                    if capture.node.range().start_byte <= range.start
                        && self
                            .workspace
                            .parsers
                            .tree_sitter
                            .queries
                            .core
                            .capture_names()[capture_index as usize]
                            != T::QUERY_NAMES[0]
                    {
                        continue;
                    } else {
                        self.start_building = true;
                    }
                }
            }

            let mut parent = self.stack.pop();

            loop {
                match &parent {
                    None => {
                        self.create_root_node(&capture, capture_index);
                        break;
                    }
                    Some(p) => {
                        if intersecting_ranges(
                            &p.get_rc().borrow().get_range(),
                            &capture.node.range(),
                        ) {
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

    fn create_root_node(&mut self, capture: &QueryCapture, capture_index: usize) {
        let mut node = T::new(
            self.workspace.url.clone(),
            &self.workspace.parsers.tree_sitter.queries.core,
            &capture,
        );

        let node_char_start = tree_sitter_range_to_lsp_range(&capture.node.range())
            .start
            .character as usize;

        log::debug!(
            "{}├──{:?} [root]",
            " ".repeat(node_char_start as usize),
            self.workspace
                .parsers
                .tree_sitter
                .queries
                .core
                .capture_names()[capture.index as usize]
        );

        match node.take() {
            Some(builder) => {
                let node = PendingSymbol::new(builder);
                self.roots.push(node.clone());
                self.stack.push(node);
            }
            None => self.workspace.diagnostics.push(builder_warning!(
                tree_sitter_range_to_lsp_range(&capture.node.range()),
                format!(
                    "Unknown query {:?}",
                    self.workspace
                        .parsers
                        .tree_sitter
                        .queries
                        .core
                        .capture_names()[capture_index as usize],
                )
            )),
        }
    }

    fn create_child_node(&mut self, parent: &PendingSymbol, capture: &QueryCapture) {
        let add = parent
            .get_rc()
            .borrow_mut()
            .add(&capture, self.workspace, &self.document);
        match add {
            Err(e) => {
                self.workspace.diagnostics.push(e);
            }
            Ok(None) => {
                self.workspace.diagnostics.push(builder_warning!(
                    tree_sitter_range_to_lsp_range(&capture.node.range()),
                    format!(
                        "Unknown query {:?}",
                        self.workspace
                            .parsers
                            .tree_sitter
                            .queries
                            .core
                            .capture_names()[capture.index as usize],
                    )
                ));
                let parent_char_start = parent
                    .get_rc()
                    .borrow()
                    .get_lsp_range(&self.document)
                    .start
                    .character as usize;

                let node_char_start = capture.node.start_position().column;

                log::warn!(
                    " {}└──{}{:?} [unknown]",
                    " ".repeat(parent_char_start as usize),
                    "─".repeat(
                        (node_char_start)
                            .checked_sub(parent_char_start + 3)
                            .or(Some(0))
                            .unwrap(),
                    ),
                    self.workspace
                        .parsers
                        .tree_sitter
                        .queries
                        .core
                        .capture_names()[capture.index as usize],
                );
            }
            Ok(Some(node)) => {
                self.stack.push(parent.clone());
                self.stack.push(node.clone());
                let parent_char_start = parent
                    .get_rc()
                    .borrow()
                    .get_lsp_range(&self.document)
                    .start
                    .character as usize;

                let node_char_start = capture.node.start_position().column;

                log::debug!(
                    "{}└──{}{:?}",
                    " ".repeat(parent_char_start as usize),
                    "─".repeat(
                        (node_char_start)
                            .checked_sub(parent_char_start + 3)
                            .or(Some(0))
                            .unwrap(),
                    ),
                    self.workspace
                        .parsers
                        .tree_sitter
                        .queries
                        .core
                        .capture_names()[capture.index as usize],
                );
            }
        };
    }

    fn get_root_node(
        &mut self,
        range: &Option<std::ops::Range<usize>>,
    ) -> Result<PendingSymbol, Diagnostic> {
        match self.roots.pop() {
            Some(node) => Ok(node),
            None => match range {
                Some(range) => {
                    let node = self
                        .document
                        .tree
                        .root_node()
                        .descendant_for_byte_range(range.start, range.end)
                        .unwrap();

                    Err(builder_error!(
                        lsp_types::Range {
                            start: Position {
                                line: node.start_position().row as u32,
                                character: node.start_position().column as u32,
                            },
                            end: Position {
                                line: node.end_position().row as u32,
                                character: node.end_position().column as u32,
                            },
                        },
                        match T::QUERY_NAMES.len() {
                            1 => format!("Expected {}", T::QUERY_NAMES[0]),
                            _ => format!("Expected one of {:?}", T::QUERY_NAMES.join(", ")),
                        }
                    ))
                }
                None => Err(builder_error!(
                    tree_sitter_range_to_lsp_range(&self.document.tree.root_node().range()),
                    match T::QUERY_NAMES.len() {
                        1 => format!("Expected {}", T::QUERY_NAMES[0]),
                        _ => format!("Expected one of {:?}", T::QUERY_NAMES.join(", ")),
                    }
                )),
            },
        }
    }
}
