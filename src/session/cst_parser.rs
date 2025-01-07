use std::sync::RwLock;
use tree_sitter::{Language, Parser, Query, Tree};

pub struct Queries {
    pub comments: Query,
    pub fold: Query,
    pub highlights: Query,
    pub outline: Query,
}

pub struct CstParser {
    pub parser: RwLock<Parser>,
    pub language: Language,
    pub queries: Queries,
}

impl CstParser {
    pub fn try_parse(&self, text: &[u8], old_tree: Option<&Tree>) -> Option<Tree> {
        self.parser.write().unwrap().parse(text, old_tree)
    }
}
