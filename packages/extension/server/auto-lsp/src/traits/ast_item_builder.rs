use downcast_rs::{impl_downcast, Downcast};
use lsp_types::Diagnostic;
use std::cell::RefCell;
use std::fmt::Formatter;
use std::rc::Rc;
use tree_sitter::Query;

pub trait AstItemBuilder: Downcast {
    fn add(
        &mut self,
        query: &Query,
        node: Rc<RefCell<dyn AstItemBuilder>>,
        source_code: &[u8],
    ) -> Result<(), Diagnostic>;

    fn get_range(&self) -> tree_sitter::Range;

    fn get_query_index(&self) -> usize;

    fn get_start_position(&self) -> tree_sitter::Point {
        self.get_range().start_point
    }

    fn get_end_position(&self) -> tree_sitter::Point {
        self.get_range().end_point
    }

    fn get_lsp_range(&self) -> lsp_types::Range {
        let range = self.get_range();
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

    fn get_text<'a>(&self, source_code: &'a [u8]) -> &'a str {
        let range = self.get_range();
        std::str::from_utf8(&source_code[range.start_byte..range.end_byte]).unwrap()
    }
}

impl std::fmt::Debug for dyn AstItemBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.get_range())
    }
}

impl_downcast!(AstItemBuilder);
