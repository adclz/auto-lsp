use auto_lsp_core::parsers::{Queries, TreeSitter};

/// Create the parsers with any given language and queries.
///
/// This generates a static map named  of parsers that can be used to parse source code in the root.
///
/// Every [`Root`] is linked to a parser, which is used to parse the source code and build both the CST and AST.
///
/// To determine which parser to use for a source code, the server will check the language ID against the keys in the map generated by this macro
///
/// # Example
/// ```rust
/// # use auto_lsp::seq;
/// # use auto_lsp::configure_parsers;
/// # use auto_lsp::core::ast::*;
/// static CORE_QUERY: &'static str = "
/// (module) @module
/// (function_definition
///    name: (identifier) @function.name) @function
/// ";
///
/// static COMMENT_QUERY: &'static str = "
/// (comment) @comment
/// ";
/// #[seq(query = "module")]
/// struct Module {}
///
/// configure_parsers!(
///     PARSER_LIST,
///     "python" => {
///         language: tree_sitter_python::LANGUAGE,
///         node_types: tree_sitter_python::NODE_TYPES,
///         ast_root: Module,
///         core: CORE_QUERY,
///         comment: Some(COMMENT_QUERY),
///         fold: None,
///         highlights: None
///     }
/// );
/// ```
#[macro_export]
macro_rules! configure_parsers {
    ($parser_list_name: ident,
        $($extension: expr => {
            language: $language: path,
            node_types: $node_types: path,
            ast_root: $root: ident,
            core: $core: path,
            comment: $comment: expr,
            fold: $fold: expr,
            highlights: $highlights: expr
        }),*) => {
        pub static $parser_list_name: std::sync::LazyLock<std::collections::HashMap<&str, $crate::core::parsers::Parsers>> =
            std::sync::LazyLock::new(|| {
                let mut map = std::collections::HashMap::new();
                map.insert(
                    $($extension, $crate::core::parsers::Parsers {
                        tree_sitter: $crate::configure::parsers::create_parser($language, $node_types, $core, $comment, $fold, $highlights),
                        ast_parser: |
                            db: &dyn $crate::core::salsa::db::BaseDatabase,
                            parsers: &'static $crate::core::parsers::Parsers,
                            url: &std::sync::Arc<lsp_types::Url>,
                            document: &$crate::core::document::Document,
                            range: Option<std::ops::Range<usize>>| {
                            use $crate::core::build::InvokeParser;

                            Ok::<$crate::core::ast::DynSymbol, $crate::lsp_types::Diagnostic>(
                                $crate::core::ast::Symbol::from($root::parse_symbol(db, parsers, url, document, range)?).into(),
                            )
                        },
                    }),*
                );
                map
            });
    };
}

#[doc(hidden)]
pub fn create_parser(
    language: tree_sitter_language::LanguageFn,
    node_types: &'static str,
    core: &'static str,
    comments: Option<&'static str>,
    fold: Option<&'static str>,
    highlights: Option<&'static str>,
) -> TreeSitter {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&language.into()).unwrap();

    let language = tree_sitter::Language::new(language);

    let core = tree_sitter::Query::new(&language, core).unwrap();
    let comments = comments.map(|path| tree_sitter::Query::new(&language, path).unwrap());
    let fold = fold.map(|path| tree_sitter::Query::new(&language, path).unwrap());
    let highlights = highlights.map(|path| tree_sitter::Query::new(&language, path).unwrap());
    TreeSitter {
        parser: parking_lot::RwLock::new(parser),
        node_types,
        language,
        queries: Queries {
            comments,
            fold,
            highlights,
            core,
        },
    }
}
