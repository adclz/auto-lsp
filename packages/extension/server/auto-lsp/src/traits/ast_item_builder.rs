use downcast_rs::{impl_downcast, Downcast};
use std::cell::RefCell;
use std::fmt::Formatter;
use std::rc::Rc;
use tree_sitter::Query;

pub trait AstItemBuilder: Downcast {
    fn add(&mut self, query: &Query, node: Rc<RefCell<dyn AstItemBuilder>>) -> bool;

    fn get_range(&self) -> tree_sitter::Range;

    fn get_query_index(&self) -> usize;

    fn get_start_position(&self) -> tree_sitter::Point {
        self.get_range().start_point
    }

    fn get_end_position(&self) -> tree_sitter::Point {
        self.get_range().end_point
    }
}

impl std::fmt::Debug for dyn AstItemBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AstItemBuilder")
    }
}

impl_downcast!(AstItemBuilder);
