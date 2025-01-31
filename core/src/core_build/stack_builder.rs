use std::marker::PhantomData;
use std::sync::Arc;

use lsp_types::Diagnostic;
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

/// Trait for invoking the stack builder
///
/// This trait is implemented for all types that implement [`Buildable`] and [`Queryable`].
pub trait InvokeStackBuilder<
    T: Buildable + Queryable,
    Y: AstSymbol + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
>
{
    /// Creates a symbol.
    ///
    /// This method internally initializes a stack builder to build the AST and derive a symbol
    /// of type Y.
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

/// Function signature for invoking the stack builder.
///
/// This type alias is useful for mapping language IDs to specific parsers,
/// helping avoiding ambiguity.
pub type InvokeStackBuilderFn = fn(
    &mut Workspace,
    &Document,
    Option<std::ops::Range<usize>>,
) -> Result<DynSymbol, lsp_types::Diagnostic>;

/// Stack builder for constructing Abstract Syntax Trees (ASTs).
///
/// This struct is responsible for building ASTs using Tree-sitter queries.
/// Inspired by [vscode anycode](https://github.com/microsoft/vscode-anycode/blob/17190d5b94850095b7ecd7cc37170324dfe08e0e/anycode/server/src/common/features/documentSymbols.ts#L50).
pub struct StackBuilder<'a, T>
where
    T: Buildable + Queryable,
{
    _meta: PhantomData<T>,
    workspace: &'a mut Workspace,
    document: &'a Document,
    /// Symbols created while building the AST
    roots: Vec<PendingSymbol>,
    stack: Vec<PendingSymbol>,
    /// Indicates whether building has started
    build: bool,
}

impl<'a, T> StackBuilder<'a, T>
where
    T: Buildable + Queryable,
{
    /// Creates a new `StackBuilder` instance.
    pub fn new(workspace: &'a mut Workspace, document: &'a Document) -> Self {
        Self {
            _meta: PhantomData,
            workspace,
            document,
            roots: vec![],
            stack: vec![],
            build: false,
        }
    }

    /// Creates a symbol of type [`Y`] based on the specified range.
    ///
    /// This method builds the AST for the provided range (if any) and attempts to derive
    /// a symbol from the root node.
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
                format!("Internal error: Could not cast {:?}", T::QUERY_NAMES)
            ))?
            .try_into_builder(self.workspace, self.document)
    }

    /// Builds the AST based on Tree-sitter query captures.
    ///
    /// If a range is specified, only the portion of the document within that range
    /// is processed. Captures are iterated in the order they appear in the tree.
    fn build(&mut self, range: &Option<std::ops::Range<usize>>) -> &mut Self {
        let mut cursor = tree_sitter::QueryCursor::new();

        let mut captures = cursor.captures(
            &self.workspace.parsers.tree_sitter.queries.core,
            self.document.tree.root_node(),
            self.document.texter.text.as_bytes(),
        );

        // Limit the captures to the specified range.
        // Note that tree sitter will capture all nodes since the beginning until the end of the range,
        // which is why we use the delay_building flag to determine when to start building the AST.
        if let Some(range) = range {
            captures.set_byte_range(range.clone());
        }

        // Iterate over the captures.
        // Captures are sorted by their location in the tree, not their pattern.
        while let Some((m, capture_index)) = captures.next() {
            let capture = Arc::new(m.captures[*capture_index]);
            let capture_index = capture.index as usize;

            // To determine if we should start building the AST, we check if the current capture
            // is within the given range, we also check if T contains the query name .
            if !self.build {
                if let Some(range) = &range {
                    if ((capture.node.range().start_byte > range.start as usize)
                        || (capture.node.range().start_byte == range.start as usize))
                        && T::QUERY_NAMES.contains(
                            &self
                                .workspace
                                .parsers
                                .tree_sitter
                                .queries
                                .core
                                .capture_names()[capture.index as usize],
                        )
                    {
                        self.build = true;
                    } else {
                        continue;
                    }
                } else if T::QUERY_NAMES.contains(
                    &self
                        .workspace
                        .parsers
                        .tree_sitter
                        .queries
                        .core
                        .capture_names()[capture.index as usize],
                ) {
                    self.build = true;
                } else {
                    continue;
                }
            }

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
                        if intersecting_ranges(
                            &p.get_rc().borrow().get_range(),
                            &capture.node.range(),
                        ) {
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
        let mut node = T::new(
            self.workspace.url.clone(),
            &self.workspace.parsers.tree_sitter.queries.core,
            &capture,
        );

        #[cfg(feature = "log")]
        {
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
        }

        match node.take() {
            Some(builder) => {
                let node = PendingSymbol::new(builder);
                self.roots.push(node.clone());
                self.stack.push(node);
            }
            None => self.workspace.diagnostics.push(builder_warning!(
                tree_sitter_range_to_lsp_range(&capture.node.range()),
                format!(
                    "Syntax error: Unexpected {:?}",
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

    /// Creates a child node and tries to add it to the parent node.
    fn create_child_node(&mut self, parent: &PendingSymbol, capture: &QueryCapture) {
        let add = parent
            .get_rc()
            .borrow_mut()
            .add(&capture, self.workspace, &self.document);
        match add {
            Err(e) => {
                // Parent did not accept the child node and returned an error.
                self.workspace.diagnostics.push(e);
            }
            Ok(None) => {
                // Parent did not accept the child node.
                self.workspace.diagnostics.push(builder_warning!(
                    tree_sitter_range_to_lsp_range(&capture.node.range()),
                    format!(
                        "Syntax error: Unexpected {:?}",
                        self.workspace
                            .parsers
                            .tree_sitter
                            .queries
                            .core
                            .capture_names()[capture.index as usize],
                    )
                ));
                #[cfg(feature = "log")]
                {
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
            }
            Ok(Some(node)) => {
                self.stack.push(parent.clone());
                self.stack.push(node.clone());
                #[cfg(feature = "log")]
                {
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
            }
        };
    }

    /// Attempt to retrive root node, initially created with [`Self::create_root_node`].
    ///
    /// If no root node exists, an error is returned indicating the expected query names.
    fn get_root_node(
        &mut self,
        range: &Option<std::ops::Range<usize>>,
    ) -> Result<PendingSymbol, Diagnostic> {
        // Root node is the last node in the stack.
        match self.roots.pop() {
            Some(node) => Ok(node),
            None => match range {
                // Since there is no root node, we return an error indicating the expected query names.
                Some(range) => {
                    let node = self
                        .document
                        .tree
                        .root_node()
                        .named_descendant_for_byte_range(range.start, range.end)
                        .unwrap();

                    Err(builder_error!(
                        tree_sitter_range_to_lsp_range(&node.range()),
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
