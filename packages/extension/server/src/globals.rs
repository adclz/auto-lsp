use auto_lsp::traits::ast_item::AstItem;
use lsp_textdocument::FullTextDocument;
use lsp_types::{Diagnostic, Uri};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tree_sitter::{Language, Parser, Query, Tree};
use tree_sitter_iec61131_3_2::{
    COMMENTS_QUERY, FOLD_QUERY, HIGHLIGHTS_QUERY, LANGUAGE, OUTLINE_QUERY,
};

use crate::symbols::symbols::Symbol;

pub struct Queries {
    pub comments: Query,
    pub fold: Query,
    pub highlights: Query,
    pub outline: Query,
}

pub struct Workspace {
    pub document: FullTextDocument,
    pub errors: Vec<Diagnostic>,
    pub cst: Tree,
    pub ast: Vec<Arc<RwLock<dyn AstItem>>>,
}

pub struct Session {
    pub workspaces: HashMap<String, Workspace>,
    pub language: Language,
    pub parser: Parser,
    pub queries: Queries,
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

impl Session {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&LANGUAGE.into())
            .expect("Error loading IEC-61131 parser");
        let lang = parser.language().unwrap();

        Self {
            workspaces: HashMap::new(),
            language: lang.clone(),
            parser,
            queries: Queries {
                comments: Query::new(&lang, COMMENTS_QUERY).unwrap(),
                fold: Query::new(&lang, FOLD_QUERY).unwrap(),
                highlights: Query::new(&lang, HIGHLIGHTS_QUERY).unwrap(),
                outline: Query::new(&lang, OUTLINE_QUERY).unwrap(),
            },
        }
    }
}
