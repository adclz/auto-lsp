use auto_lsp::traits::ast_item::AstItem;
use lazy_static::lazy_static;
use lsp_textdocument::FullTextDocument;
use lsp_types::{Diagnostic, Uri};
use phf::{phf_map, Map};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tree_sitter::{Language, Parser, Query, Tree};

lazy_static! {
    pub static ref PARSERS: HashMap<String, ParserProvider> = {
        HashMap::from([(
            "iec-61131-2".into(),
            crate::create_parser!(tree_sitter_iec61131_3_2),
        )])
    };
}

pub struct Queries {
    pub comments: Query,
    pub fold: Query,
    pub highlights: Query,
    pub outline: Query,
}

pub struct ParserProvider {
    pub parser: RwLock<Parser>,
    pub language: Language,
    pub queries: Queries,
}

impl ParserProvider {
    pub fn try_parse(&self, text: &[u8], old_tree: Option<&Tree>) -> Option<Tree> {
        self.parser.write().unwrap().parse(text, old_tree)
    }
}

pub struct Workspace<'a> {
    pub provider: &'a ParserProvider,
    pub document: FullTextDocument,
    pub errors: Vec<Diagnostic>,
    pub cst: Tree,
    pub ast: Vec<Arc<RwLock<dyn AstItem>>>,
}

pub struct Session<'a> {
    pub extensions: HashMap<String, String>,
    pub workspaces: HashMap<String, Workspace<'a>>,
}

impl<'a> Default for Session<'a> {
    fn default() -> Self {
        Self {
            extensions: HashMap::new(),
            workspaces: HashMap::new(),
        }
    }
}

#[macro_export]
macro_rules! create_parser {
    ($parser: ident) => {{
        use std::sync::RwLock;
        use $parser::{COMMENTS_QUERY, FOLD_QUERY, HIGHLIGHTS_QUERY, LANGUAGE, OUTLINE_QUERY};
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&LANGUAGE.into())
            .expect(&format!("Error loading {} parser", stringify!($parser)));
        let lang = parser.language().unwrap();
        crate::globals::ParserProvider {
            parser: RwLock::new(parser),
            language: lang.clone(),
            queries: crate::globals::Queries {
                comments: tree_sitter::Query::new(&lang, COMMENTS_QUERY).unwrap(),
                fold: tree_sitter::Query::new(&lang, FOLD_QUERY).unwrap(),
                highlights: tree_sitter::Query::new(&lang, HIGHLIGHTS_QUERY).unwrap(),
                outline: tree_sitter::Query::new(&lang, OUTLINE_QUERY).unwrap(),
            },
        }
    }};
}
