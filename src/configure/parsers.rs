use tree_sitter::Language;

/// Generates a `pub static LazyLock<Parser>` for the given tree-sitter language and AST root.
///
/// The library has no opinion on how parsers are dispatched, callers pass the produced static
/// directly to file-creation/update functions. For multi-language servers, declare one static
/// per language and pick the right one in your handler closures.
///
/// # Example
/// ```rust, ignore
/// use auto_lsp::configure_parser;
///
/// configure_parser!(
///     PYTHON,
///     language: tree_sitter_python::LANGUAGE,
///     ast_root: Module,
/// );
///
/// // ...later, at a call site:
/// File::from_text_doc().session(session).doc(&doc).parsers(&PYTHON).call()?;
/// ```
#[macro_export]
macro_rules! configure_parser {
    ($name: ident,
     language: $language: path,
     ast_root: $root: ident $(,)?) => {
        pub static $name: std::sync::LazyLock<$crate::core::parsers::Parser> =
            std::sync::LazyLock::new(|| {
                let data = $crate::configure::parsers::create_parser($language);
                $crate::core::parsers::Parser {
                    parser: data.0,
                    language: data.1,
                    ast_parser:
                        |db: &dyn ::auto_lsp::salsa::Database,
                         document: &$crate::core::document::Document| {
                            let mut builder = $crate::core::ast::Builder::default();
                            let root = $root::try_from((
                                &document.tree.root_node(),
                                db,
                                &mut builder,
                                0,
                                None,
                            ))
                            .map_err(|e| $crate::core::errors::ParseError::from(e))?;
                            let mut nodes = builder.take_nodes();
                            nodes.push(Box::new(root));
                            Ok(nodes)
                        },
                }
            });
    };
}

#[doc(hidden)]
pub fn create_parser(
    language: tree_sitter_language::LanguageFn,
) -> (parking_lot::RwLock<tree_sitter::Parser>, Language) {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&language.into()).unwrap();
    let language = tree_sitter::Language::new(language);

    (parking_lot::RwLock::new(parser), language)
}
