use std::sync::RwLock;
use tree_sitter::{Language, Parser, Query, Tree};

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
        crate::session::parser_provider::ParserProvider {
            parser: RwLock::new(parser),
            language: lang.clone(),
            queries: crate::session::parser_provider::Queries {
                comments: tree_sitter::Query::new(&lang, COMMENTS_QUERY).unwrap(),
                fold: tree_sitter::Query::new(&lang, FOLD_QUERY).unwrap(),
                highlights: tree_sitter::Query::new(&lang, HIGHLIGHTS_QUERY).unwrap(),
                outline: tree_sitter::Query::new(&lang, OUTLINE_QUERY).unwrap(),
            },
        }
    }};
}
