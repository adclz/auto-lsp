use crate::builder_error;
use crate::convert::TryFromBuilder;
use crate::pending_symbol::{AstBuilder, PendingSymbol, TryDownCast};
use crate::symbol::{AstSymbol, DynSymbol, EditRange, Locator, SymbolData};
use crate::workspace::WorkspaceContext;
use lsp_textdocument::FullTextDocument;
use lsp_types::{Diagnostic, TextDocumentContentChangeEvent, Url};
use std::ops::{ControlFlow, Range};
use std::sync::Arc;
use std::vec;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Query, QueryCapture};

#[macro_export]
macro_rules! builder_error {
    ($range: expr, $text: expr) => {
        lsp_types::Diagnostic::new(
            $range,
            Some(lsp_types::DiagnosticSeverity::ERROR),
            None,
            None,
            $text.into(),
            None,
            None,
        )
    };
}

pub struct BuilderParams<'a> {
    pub ctx: &'a dyn WorkspaceContext,
    pub query: &'a tree_sitter::Query,
    pub root_node: tree_sitter::Node<'a>,
    pub doc: &'a FullTextDocument,
    pub url: Arc<Url>,
    pub diagnostics: &'a mut Vec<Diagnostic>,
    pub checks: &'a mut Vec<DynSymbol>,
}

struct StackBuilder<'a, 'b> {
    params: &'a mut BuilderParams<'b>,
    roots: Vec<PendingSymbol>,
    stack: Vec<PendingSymbol>,
    start_building: bool,
}

impl<'a, 'b> StackBuilder<'a, 'b> {
    fn create_root_node<T: AstBuilder>(&mut self, capture: &QueryCapture, capture_index: usize) {
        let mut node = T::new(
            self.params.url.clone(),
            self.params.query,
            capture_index,
            capture.node.range(),
            capture.node.start_position(),
            capture.node.end_position(),
        );

        match node.take() {
            Some(builder) => {
                let node = PendingSymbol::new(builder);
                self.roots.push(node.clone());
                self.stack.push(node);
            }
            None => self.params.diagnostics.push(builder_error!(
                tree_sitter_range_to_lsp_range(&capture.node.range()),
                format!(
                    "Failed to create builder for query: {:?}, is the symbol declared in the AST ?",
                    self.params.query.capture_names()[capture_index as usize]
                )
            )),
        }
    }

    fn create_child_node(
        &mut self,
        parent: &PendingSymbol,
        url: Arc<Url>,
        query: &Query,
        capture: &QueryCapture,
        capture_index: usize,
    ) {
        let node = parent
            .get_rc()
            .borrow()
            .query_binder(url.clone(), &capture, &query);

        match node.as_ref() {
            Some(node) => {
                if let Err(e) = parent.get_rc().borrow_mut().add(
                    &query,
                    node.clone(),
                    self.params.doc.get_content(None).as_bytes(),
                    self.params,
                ) {
                    self.params.diagnostics.push(e);
                    return;
                };
                self.stack.push(parent.clone());
                self.stack.push(node.clone());
            }
            None => self.params.diagnostics.push(builder_error!(
                tree_sitter_range_to_lsp_range(&capture.node.range()),
                format!(
                    "Failed to create builder for query: {:?}, is the symbol declared in the AST ?",
                    query.capture_names()[capture_index as usize],
                )
            )),
        };
    }

    pub fn new(builder_params: &'a mut BuilderParams<'b>) -> Self {
        Self {
            params: builder_params,
            roots: vec![],
            stack: vec![],
            start_building: false,
        }
    }

    pub fn build<T: AstBuilder>(&mut self, range: Option<std::ops::Range<usize>>) {
        let mut cursor = tree_sitter::QueryCursor::new();

        let start_node = match &range {
            Some(range) => {
                let node = self
                    .params
                    .root_node
                    .descendant_for_byte_range(range.start, range.end)
                    .unwrap();
                cursor.set_byte_range(range.clone());
                Some(node)
            }
            None => None,
        };

        let mut captures = cursor.captures(
            &self.params.query,
            self.params.root_node,
            self.params.doc.get_content(None).as_bytes(),
        );

        while let Some((m, capture_index)) = captures.next() {
            let capture = m.captures[*capture_index];
            let capture_index = capture.index as usize;

            if !self.start_building {
                if let Some(node) = start_node {
                    if node.range() != capture.node.range() {
                        continue;
                    } else {
                        self.start_building = true;
                    }
                }
            }

            if range.is_some() {
                eprintln!(
                    "captures: {:?}",
                    self.params.query.capture_names()[capture_index]
                );
            }

            let mut parent = self.stack.pop();

            loop {
                match &parent {
                    None => {
                        self.create_root_node::<T>(&capture, capture_index);
                        break;
                    }
                    Some(p) => {
                        if intersecting_ranges(
                            &p.get_rc().borrow().get_range(),
                            &capture.node.range(),
                        ) {
                            self.create_child_node(
                                p,
                                self.params.url.clone(),
                                &self.params.query,
                                &capture,
                                capture_index,
                            );
                            break;
                        }
                    }
                }
                parent = self.stack.pop();
            }
        }
    }

    fn get_root_node(&mut self) -> Result<PendingSymbol, Diagnostic> {
        match self.roots.pop() {
            Some(node) => Ok(node),
            None => Err(builder_error!(
                lsp_types::Range {
                    start: lsp_types::Position {
                        line: 0,
                        character: 0,
                    },
                    end: lsp_types::Position {
                        line: self.params.doc.line_count() as u32,
                        character: 0,
                    },
                },
                "No root node found".to_string()
            )),
        }
    }

    pub fn to_dyn_symbol(&mut self) -> Result<DynSymbol, Diagnostic> {
        let result = self.get_root_node()?;
        let result = result.get_rc().borrow();

        let deferred: Vec<DynSymbol> = vec![];
        let item = result.try_to_dyn_symbol(self.params)?;
        item.write().inject_parent(item.to_weak());

        if item.read().is_accessor() {
            match item.read().find(&self.params.doc) {
                Ok(a) => {
                    if let Some(a) = a {
                        // todo!
                        a.write().get_mut_referrers().add_reference(item.to_weak());

                        item.write().set_target(a.to_weak());
                    };
                }
                Err(err) => self.params.diagnostics.push(err),
            }
        }

        if let Err(err) = item.write().find(&self.params.doc) {
            self.params.diagnostics.push(err);
        }

        let read = item.read();

        if read.must_check() {
            if let Ok(_) = read.check(self.params.doc, self.params.diagnostics) {
                //self.params.checks
            }
        }

        for item in deferred {
            let acc = if item.read().is_accessor() {
                match item.read().find(&self.params.doc) {
                    Ok(a) => a,
                    Err(err) => {
                        self.params.diagnostics.push(err);
                        None
                    }
                }
            } else {
                None
            };

            if let Some(a) = acc {
                // todo!
                a.write().get_mut_referrers().add_reference(item.to_weak());

                item.write().set_target(a.to_weak());
            }

            if item.read().must_check() {
                if let Err(_) = item.read().check(&self.params.doc, self.params.diagnostics) {}
            }
        }
        drop(read);
        Ok(item)
    }
}

pub type BuilderFn = for<'a> fn(
    params: &'a mut BuilderParams<'a>,
    range: Option<std::ops::Range<usize>>,
) -> Result<DynSymbol, Diagnostic>;

pub trait Builder {
    fn builder<'a>(
        params: &'a mut BuilderParams<'a>,
        range: Option<std::ops::Range<usize>>,
    ) -> Result<DynSymbol, Diagnostic>;
}

pub trait StaticBuilder<
    T: AstBuilder,
    Y: AstSymbol + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
>
{
    fn static_build<'a>(
        params: &'a mut BuilderParams,
        range: Option<std::ops::Range<usize>>,
    ) -> Result<Y, Diagnostic>;
}

impl<T, Y> StaticBuilder<T, Y> for Y
where
    T: AstBuilder,
    Y: AstSymbol + for<'b> TryFromBuilder<&'b T, Error = lsp_types::Diagnostic>,
{
    fn static_build<'a>(
        builder_params: &'a mut BuilderParams,
        range: Option<std::ops::Range<usize>>,
    ) -> Result<Y, Diagnostic> {
        let mut builder = StackBuilder::new(builder_params);
        builder.build::<T>(range);

        let root = &builder.roots[0];
        let ds = root.try_downcast(
            builder_params,
            "field_name",
            lsp_types::Range::default(),
            "input_name",
        );

        ds
    }
}

fn intersecting_ranges(range1: &std::ops::Range<usize>, range2: &tree_sitter::Range) -> bool {
    range1.start <= range2.start_byte && range1.end >= range2.end_byte
}

fn tree_sitter_range_to_lsp_range(range: &tree_sitter::Range) -> lsp_types::Range {
    let start = range.start_point;
    let end = range.end_point;
    lsp_types::Range {
        start: lsp_types::Position {
            line: start.row as u32,
            character: start.column as u32,
        },
        end: lsp_types::Position {
            line: end.row as u32,
            character: end.column as u32,
        },
    }
}

impl<T: AstBuilder> Builder for T {
    fn builder<'a>(
        params: &'a mut BuilderParams<'a>,
        range: Option<Range<usize>>,
    ) -> Result<DynSymbol, Diagnostic> {
        let mut builder = StackBuilder::new(params);
        builder.build::<T>(range);
        builder.to_dyn_symbol()
    }
}

pub fn swap_ast<'a>(
    old_ast: Option<&DynSymbol>,
    edit_ranges: &Vec<TextDocumentContentChangeEvent>,
    builder_params: &'a mut BuilderParams<'a>,
) {
    let root = match old_ast.as_ref() {
        Some(ast) => ast,
        None => return,
    };

    let doc = builder_params.doc;

    for edit in edit_ranges.iter() {
        let edit_range = edit.range.unwrap();

        let range_offset = doc.offset_at(edit_range.start) as usize;
        let start_byte = range_offset;
        let old_end_byte = range_offset + edit.range_length.unwrap() as usize;
        let new_end_byte = range_offset + edit.text.len();

        let is_noop = old_end_byte == start_byte && new_end_byte == start_byte;
        if is_noop {
            continue;
        }

        let start_edit = match root.write().dyn_swap(
            start_byte,
            (new_end_byte - old_end_byte) as isize,
            builder_params,
        ) {
            ControlFlow::Continue(()) => start_byte,
            ControlFlow::Break(Ok(start_edit)) => start_edit,
            ControlFlow::Break(Err(err)) => {
                builder_params.diagnostics.push(err);
                start_byte
            }
        };

        root.edit_range(start_edit, (new_end_byte - old_end_byte) as isize);

        eprintln!(
            "Edit: Shift at {:?} of {:?}",
            start_byte,
            (new_end_byte - old_end_byte) as isize,
        );
    }
}
