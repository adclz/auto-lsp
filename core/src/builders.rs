use crate::builder_error;
use crate::convert::{TryFromBuilder, TryIntoBuilder};
use crate::pending_symbol::{AstBuilder, PendingSymbol};
use crate::queryable::Queryable;
use crate::symbol::{AstSymbol, DynSymbol, EditRange, ReferrersTrait, SymbolData, WeakSymbol};
use crate::workspace::WorkspaceContext;
use lsp_textdocument::FullTextDocument;
use lsp_types::{Diagnostic, TextDocumentContentChangeEvent, Url};
use std::marker::PhantomData;
use std::ops::ControlFlow;
use std::sync::Arc;
use std::vec;
use streaming_iterator::StreamingIterator;
use tree_sitter::QueryCapture;

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
    ($path: ident, $range: expr, $text: expr) => {
        $path::lsp_types::Diagnostic::new(
            $range,
            Some($path::lsp_types::DiagnosticSeverity::ERROR),
            None,
            None,
            $text.into(),
            None,
            None,
        )
    };
}

#[macro_export]
macro_rules! builder_warning {
    ($range: expr, $text: expr) => {
        lsp_types::Diagnostic::new(
            $range,
            Some(lsp_types::DiagnosticSeverity::WARNING),
            None,
            None,
            $text.into(),
            None,
            None,
        )
    };
    ($path: ident, $range: expr, $text: expr) => {
        $path::lsp_types::Diagnostic::new(
            $range,
            Some($path::lsp_types::DiagnosticSeverity::WARNING),
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
    pub unsolved_checks: &'a mut Vec<WeakSymbol>,
    pub unsolved_references: &'a mut Vec<WeakSymbol>,
}

impl<'a> BuilderParams<'a> {
    pub fn resolve_references(&mut self) -> &mut Self {
        self.unsolved_references.retain(|item| {
            let item = match item.to_dyn() {
                Some(read) => read,
                None => return false,
            };
            let read = item.read();
            match read.find(&self.doc) {
                Ok(Some(target)) => {
                    target.write().add_referrer(item.to_weak());
                    drop(read);
                    item.write().set_target(target.to_weak());
                    false
                }
                Ok(None) => true,
                Err(err) => {
                    self.diagnostics.push(err);
                    true
                }
            }
        });
        self
    }

    pub fn resolve_checks(&mut self) -> &mut Self {
        self.unsolved_checks.retain(|item| {
            let item = match item.to_dyn() {
                Some(read) => read,
                None => return false,
            };
            let read = item.read();
            match read.check(&self.doc, self.diagnostics) {
                Ok(()) => false,
                Err(()) => true,
            }
        });
        self
    }

    pub fn swap_ast(
        &'a mut self,
        root: &DynSymbol,
        edit_ranges: &Vec<(&TextDocumentContentChangeEvent, bool)>,
    ) -> &'a mut BuilderParams<'a> {
        let doc = self.doc;

        for (edit, is_ws) in edit_ranges.iter() {
            let edit_range = edit.range.unwrap();

            let range_offset = doc.offset_at(edit_range.start) as usize;
            let start_byte = range_offset;
            let old_end_byte = range_offset + edit.range_length.unwrap() as usize;
            let new_end_byte = range_offset + edit.text.len();

            let is_noop = old_end_byte == start_byte && new_end_byte == start_byte;
            if is_noop {
                continue;
            }

            root.edit_range(start_byte, (new_end_byte - old_end_byte) as isize);

            if !is_ws {
                if let ControlFlow::Break(Err(e)) =
                    root.write()
                        .dyn_swap(start_byte, (new_end_byte - old_end_byte) as isize, self)
                {
                    self.diagnostics.push(e);
                };
            }

            /*eprintln!(
                "Edit: Shift at {:?} of {:?}",
                start_byte,
                (new_end_byte - old_end_byte) as isize,
            );*/
        }
        self
    }
}

struct StackBuilder<'a, 'b, T>
where
    T: AstBuilder + Queryable,
{
    _meta: PhantomData<T>,
    params: &'a mut BuilderParams<'b>,
    roots: Vec<PendingSymbol>,
    stack: Vec<PendingSymbol>,
    start_building: bool,
}

impl<'a, 'b, T> StackBuilder<'a, 'b, T>
where
    T: AstBuilder + Queryable,
{
    fn create_root_node(&mut self, capture: &QueryCapture, capture_index: usize) {
        let mut node = T::new(self.params.url.clone(), &self.params.query, &capture);

        /*eprintln!(
            "Creating root node {:?}",
            self.params.query.capture_names()[capture.index as usize]
        );*/

        match node.take() {
            Some(builder) => {
                let node = PendingSymbol::new(builder);
                self.roots.push(node.clone());
                self.stack.push(node);
            }
            None => self.params.diagnostics.push(builder_warning!(
                tree_sitter_range_to_lsp_range(&capture.node.range()),
                format!(
                    "Unknown query {:?}",
                    self.params.query.capture_names()[capture_index as usize],
                )
            )),
        }
    }

    fn create_child_node(&mut self, parent: &PendingSymbol, capture: &QueryCapture) {
        match parent.get_rc().borrow_mut().add(&capture, self.params) {
            Err(e) => {
                self.params.diagnostics.push(e);
            }
            Ok(None) => self.params.diagnostics.push(builder_warning!(
                tree_sitter_range_to_lsp_range(&capture.node.range()),
                format!(
                    "Unknown query {:?}",
                    self.params.query.capture_names()[capture.index as usize],
                )
            )),
            Ok(Some(node)) => {
                self.stack.push(parent.clone());
                self.stack.push(node.clone());
            }
        };
    }

    pub fn new(builder_params: &'a mut BuilderParams<'b>) -> Self {
        Self {
            _meta: PhantomData,
            params: builder_params,
            roots: vec![],
            stack: vec![],
            start_building: false,
        }
    }

    pub fn build(&mut self, range: &Option<std::ops::Range<usize>>) -> &mut Self {
        let mut cursor = tree_sitter::QueryCursor::new();

        let mut captures = cursor.captures(
            &self.params.query,
            self.params.root_node,
            self.params.doc.get_content(None).as_bytes(),
        );

        if let Some(range) = range {
            captures.set_byte_range(range.clone());
        }

        while let Some((m, capture_index)) = captures.next() {
            let capture = m.captures[*capture_index];
            let capture_index = capture.index as usize;

            if !self.start_building {
                if let Some(range) = range {
                    if capture.node.range().start_byte.lt(&range.start) {
                        continue;
                    } else {
                        self.start_building = true;
                    }
                }
            }

            eprintln!("\x1b[2J\x1b[1;1H captures {:?}", capture);

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

    fn get_root_node(
        &mut self,
        range: &Option<std::ops::Range<usize>>,
    ) -> Result<PendingSymbol, Diagnostic> {
        match self.roots.pop() {
            Some(node) => Ok(node),
            None => match range {
                Some(range) => Err(builder_error!(
                    lsp_types::Range {
                        start: self.params.doc.position_at(range.start as u32),
                        end: self.params.doc.position_at(range.end as u32),
                    },
                    match T::QUERY_NAMES.len() {
                        1 => format!("Expected {}", T::QUERY_NAMES[0]),
                        _ => format!("Expected one of {:?}", T::QUERY_NAMES.join(", ")),
                    }
                )),
                None => Err(builder_error!(
                    tree_sitter_range_to_lsp_range(&self.params.root_node.range()),
                    match T::QUERY_NAMES.len() {
                        1 => format!("Expected {}", T::QUERY_NAMES[0]),
                        _ => format!("Expected one of {:?}", T::QUERY_NAMES.join(", ")),
                    }
                )),
            },
        }
    }

    pub fn to_static_symbol<Y>(
        &mut self,
        range: &Option<std::ops::Range<usize>>,
    ) -> Result<Y, Diagnostic>
    where
        Y: AstSymbol + for<'c> TryFromBuilder<&'c T, Error = lsp_types::Diagnostic>,
    {
        let result = self.get_root_node(range)?;
        let result = result.get_rc().borrow();
        result
            .downcast_ref::<T>()
            .ok_or(builder_error!(
                result.get_lsp_range(self.params.doc),
                format!("Invalid cast {:?}", T::QUERY_NAMES[0])
            ))?
            .try_into_builder(self.params)
    }
}

fn intersecting_ranges(range1: &std::ops::Range<usize>, range2: &tree_sitter::Range) -> bool {
    range1.start <= range2.start_byte && range1.end >= range2.end_byte
}

pub fn tree_sitter_range_to_lsp_range(range: &tree_sitter::Range) -> lsp_types::Range {
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

pub trait StaticBuilder<
    T: AstBuilder + Queryable,
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
    T: AstBuilder + Queryable,
    Y: AstSymbol + for<'b> TryFromBuilder<&'b T, Error = lsp_types::Diagnostic>,
{
    fn static_build<'a>(
        builder_params: &'a mut BuilderParams,
        range: Option<std::ops::Range<usize>>,
    ) -> Result<Y, Diagnostic> {
        StackBuilder::<T>::new(builder_params)
            .build(&range)
            .to_static_symbol(&range)
    }
}
