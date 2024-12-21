use crate::builder_error;
use crate::convert::TryFromBuilder;
use crate::pending_symbol::{AstBuilder, PendingSymbol, TryDownCast};
use crate::symbol::{AstSymbol, DynSymbol, Locator, Symbol, SymbolData};
use crate::workspace::WorkspaceContext;
use lsp_textdocument::FullTextDocument;
use lsp_types::{Diagnostic, TextDocumentContentChangeEvent, Url};
use std::ops::Range;
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
}

struct StackBuilder<'a> {
    builder_params: &'a mut BuilderParams<'a>,
    roots: Vec<PendingSymbol>,
    stack: Vec<PendingSymbol>,
    start_building: bool,
}

impl<'a> StackBuilder<'a> {
    fn create_root_node<T: AstBuilder>(&mut self, capture: &QueryCapture, capture_index: usize) {
        let mut node = T::new(
            self.builder_params.url.clone(),
            &self.builder_params.query,
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
            None => self.builder_params.diagnostics.push(builder_error!(
                tree_sitter_range_to_lsp_range(&capture.node.range()),
                format!(
                    "Failed to create builder for query: {:?}, is the symbol declared in the AST ?",
                    self.builder_params.query.capture_names()[capture_index as usize]
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
                    &self.builder_params.doc.get_content(None).as_bytes(),
                ) {
                    self.builder_params.diagnostics.push(e);
                    return;
                };
                self.stack.push(parent.clone());
                self.stack.push(node.clone());
            }
            None => self.builder_params.diagnostics.push(builder_error!(
                tree_sitter_range_to_lsp_range(&capture.node.range()),
                format!(
                    "Failed to create builder for query: {:?}, is the symbol declared in the AST ?",
                    query.capture_names()[capture_index as usize],
                )
            )),
        };
    }

    pub fn build<T: AstBuilder>(
        range: Option<std::ops::Range<usize>>,
        builder_params: &'a mut BuilderParams<'a>,
    ) -> StackBuilder<'a> {
        let mut builder = Self {
            builder_params,
            roots: vec![],
            stack: vec![],
            start_building: false,
        };

        let mut cursor = tree_sitter::QueryCursor::new();

        let start_node = match &range {
            Some(range) => {
                let node = builder
                    .builder_params
                    .root_node
                    .descendant_for_byte_range(range.start, range.end)
                    .unwrap();
                cursor.set_byte_range(range.clone());
                Some(node)
            }
            None => None,
        };

        let mut captures = cursor.captures(
            &builder.builder_params.query,
            builder.builder_params.root_node,
            builder.builder_params.doc.get_content(None).as_bytes(),
        );

        while let Some((m, capture_index)) = captures.next() {
            let capture = m.captures[*capture_index];
            let capture_index = capture.index as usize;

            if !builder.start_building {
                if let Some(node) = start_node {
                    if node.range() != capture.node.range() {
                        continue;
                    } else {
                        builder.start_building = true;
                    }
                }
            }

            /*if range.is_some() {
                eprintln!(
                    "captures: {:?}",
                    builder.builder_params.query.capture_names()[capture_index]
                );
            }*/

            let mut parent = builder.stack.pop();

            loop {
                match &parent {
                    None => {
                        builder.create_root_node::<T>(&capture, capture_index);
                        break;
                    }
                    Some(p) => {
                        if intersecting_ranges(
                            &p.get_rc().borrow().get_range(),
                            &capture.node.range(),
                        ) {
                            builder.create_child_node(
                                p,
                                builder.builder_params.url.clone(),
                                &builder.builder_params.query,
                                &capture,
                                capture_index,
                            );
                            break;
                        }
                    }
                }
                parent = builder.stack.pop();
            }
        }
        builder
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
                        line: self.builder_params.doc.line_count() as u32,
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

        let mut deferred = vec![];
        let item = result.try_to_dyn_symbol(&mut deferred)?;
        item.write().inject_parent(item.to_weak());

        if item.read().is_accessor() {
            match item
                .read()
                .find(&self.builder_params.doc, self.builder_params.ctx)
            {
                Ok(a) => {
                    if let Some(a) = a {
                        // todo!
                        a.write().get_mut_referrers().add_reference(item.to_weak());

                        item.write().set_target(a.to_weak());
                    };
                }
                Err(err) => self.builder_params.diagnostics.push(err),
            }
        }

        if let Err(err) = item
            .write()
            .find(&self.builder_params.doc, self.builder_params.ctx)
        {
            self.builder_params.diagnostics.push(err);
        }

        let read = item.read();

        if read.must_check() {
            read.check(self.builder_params.doc, self.builder_params.diagnostics);
        }

        for item in deferred {
            let acc = if item.read().is_accessor() {
                match item
                    .read()
                    .find(&self.builder_params.doc, self.builder_params.ctx)
                {
                    Ok(a) => a,
                    Err(err) => {
                        self.builder_params.diagnostics.push(err);
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
                item.read()
                    .check(&self.builder_params.doc, self.builder_params.diagnostics);
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
        params: &'a mut BuilderParams<'a>,
        range: Option<std::ops::Range<usize>>,
    ) -> Result<Symbol<Y>, Diagnostic>;
}

impl<T, Y> StaticBuilder<T, Y> for Y
where
    T: AstBuilder,
    Y: AstSymbol + for<'b> TryFromBuilder<&'b T, Error = lsp_types::Diagnostic>,
{
    fn static_build<'a>(
        builder_params: &'a mut BuilderParams<'a>,
        range: Option<std::ops::Range<usize>>,
    ) -> Result<Symbol<Y>, Diagnostic> {
        let builder = StackBuilder::build::<T>(range, builder_params);

        let root = &builder.roots[0];
        let ds = root.try_downcast(
            &mut vec![],
            "field_name",
            lsp_types::Range::default(),
            "input_name",
        );

        ds
    }
}

fn intersecting_ranges(range1: &tree_sitter::Range, range2: &tree_sitter::Range) -> bool {
    range1.start_byte <= range2.start_byte && range1.end_byte >= range2.end_byte
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
        StackBuilder::build::<T>(range, params).to_dyn_symbol()
    }
}

pub struct EditRange {
    pub start_byte: usize,
    pub steps: isize,
}

pub fn swap_ast(
    old_ast: Option<&DynSymbol>,
    edit_ranges: &Vec<TextDocumentContentChangeEvent>,
    builder_params: &mut BuilderParams,
) -> Vec<EditRange> {
    let mut results = vec![];

    let root = match old_ast.as_ref() {
        Some(ast) => ast,
        None => return results,
    };

    let doc = builder_params.doc;

    for edit in edit_ranges.iter() {
        let edit_range = edit.range.unwrap();

        let range_offset = doc.offset_at(edit_range.start) as usize;
        let start_byte = range_offset;
        let old_end_byte = range_offset + edit.range_length.unwrap() as usize;
        let new_end_byte = range_offset + edit.text.len();

        let start_position = doc.position_at(start_byte as u32);
        let old_end_position = doc.position_at(old_end_byte as u32);

        let is_noop = old_end_byte == start_byte && new_end_byte == start_byte;
        if is_noop {
            continue;
        }

        let is_insertion = old_end_byte == start_byte;

        let is_ws = match is_insertion {
            true => edit.text.trim().is_empty(),
            false => doc
                .get_content(Some(lsp_types::Range {
                    start: start_position,
                    end: old_end_position,
                }))
                .trim()
                .is_empty(),
        };

        if !is_ws {
            match root.find_at_offset(range_offset) {
                // todo! implement to_swap
                Some(node) => {
                    eprintln!(
                        "Edit: Shift at {:?} of {:?}",
                        start_byte,
                        (new_end_byte - old_end_byte) as isize,
                    );
                    eprintln!("Edited: Found node at offset: {:?}", range_offset);
                    results.push(EditRange {
                        start_byte,
                        steps: (new_end_byte - old_end_byte) as isize,
                    });
                }
                None => {
                    eprintln!(
                        "Edit: Shift at {:?} of {:?}",
                        start_byte,
                        (new_end_byte - old_end_byte) as isize,
                    );
                    eprintln!("No node found at offset: {:?}", range_offset);
                    results.push(EditRange {
                        start_byte,
                        steps: (new_end_byte - old_end_byte) as isize,
                    });
                }
            }
        }
    }

    results
}
