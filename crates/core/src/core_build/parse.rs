use std::sync::Arc;

use crate::ast::DynSymbol;
use crate::document::Document;
use crate::parsers::Parsers;
use crate::salsa::db::BaseDatabase;
use crate::salsa::db::BaseDb;
use crate::salsa::tracked::get_ast;
use crate::salsa::tracked::DiagnosticAccumulator;
use crate::{
    ast::AstSymbol,
    build::{Buildable, Queryable, TryFromBuilder},
};
use ariadne::Fmt;
use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
use lsp_types::Url;
use texter::core::text::Text;

use super::stack_builder::StackBuilder;

/// Trait for invoking the stack builder
///
/// This trait is implemented for all types that implement [`Buildable`] and [`Queryable`].
pub trait InvokeParser<
    T: Buildable + Queryable,
    Y: AstSymbol + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
>
{
    /// Creates a symbol.
    ///
    /// This method internally initializes a stack builder to build the AST and derive a symbol
    /// of type Y.
    fn parse_symbol(
        db: &dyn BaseDatabase,
        parsers: &'static Parsers,
        url: &Arc<Url>,
        document: &Document,
    ) -> Result<Y, lsp_types::Diagnostic>;
}

impl<T, Y> InvokeParser<T, Y> for Y
where
    T: Buildable + Queryable,
    Y: AstSymbol + for<'b> TryFromBuilder<&'b T, Error = lsp_types::Diagnostic>,
{
    fn parse_symbol(
        db: &dyn BaseDatabase,
        parsers: &'static Parsers,
        url: &Arc<Url>,
        document: &Document,
    ) -> Result<Y, lsp_types::Diagnostic> {
        StackBuilder::<T>::new(db, parsers, url, document).create_symbol()
    }
}

/// Function signature for invoking the stack builder.
///
/// This type alias is useful for mapping language IDs to specific parsers,
/// avoiding ambiguity.
pub type InvokeParserFn = fn(
    &dyn BaseDatabase,
    &'static Parsers,
    &Arc<Url>,
    &Document,
) -> Result<DynSymbol, lsp_types::Diagnostic>;

pub type TestParseResult<E = AriadneReport> = anyhow::Result<(), Box<E>>;

pub struct AriadneReport {
    pub result: Report<'static>,
    pub cache: Source<&'static str>,
}

impl std::fmt::Debug for AriadneReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for AriadneReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = Vec::<u8>::new();
        self.result.write(self.cache.clone(), &mut output).unwrap();
        write!(f, "{}", String::from_utf8_lossy(&output).into_owned())
    }
}

pub trait TryParse<
    T: Buildable + Queryable,
    Y: AstSymbol + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
>
{
    /// Parses the provided test code and validates the AST symbol construction.
    ///
    /// # Arguments
    /// - `test_code`: The code to be parsed and analyzed.
    /// - `parsers`: A reference to the parsers for syntax tree generation.
    ///
    /// # Returns [`TestParseResult`]
    fn test_parse(test_code: &'static str, parsers: &'static Parsers) -> TestParseResult;
}

impl<T, Y> TryParse<T, Y> for Y
where
    T: Buildable + Queryable,
    Y: AstSymbol + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
{
    fn test_parse(test_code: &'static str, parsers: &'static Parsers) -> TestParseResult {
        let mut db = BaseDb::default();
        let url = Url::parse("file://test.txt").unwrap();
        let text = Text::new(test_code.to_string());
        let source = Source::from(test_code);

        match db.add_file_from_texter(parsers, &url, text) {
            Ok(_) => {}
            Err(e) => {
                return Err(Box::new(AriadneReport {
                    result: Report::build(ReportKind::Error, 0..test_code.len())
                        .with_message(format!("Failed to create file: {}", e))
                        .finish(),
                    cache: source,
                }));
            }
        }

        let file = db.get_file(&url).unwrap();
        let ast = get_ast(&db, file);
        let diagnostics = get_ast::accumulated::<DiagnosticAccumulator>(&db, file);

        match diagnostics.is_empty() {
            false => {
                let mut colors = ColorGenerator::new();
                let mut report = Report::build(ReportKind::Error, 0..test_code.len())
                    .with_message(format!("Parsing failed: {} error(s)", diagnostics.len()));

                for diagnostic in &diagnostics {
                    let diagnostic = diagnostic.0.clone();
                    let range = diagnostic.range;
                    let start_line = source.line(range.start.line as usize).unwrap().offset();
                    let end_line = source.line(range.end.line as usize).unwrap().offset();
                    let start = start_line + range.start.character as usize;
                    let end = end_line + range.end.character as usize;

                    let curr_color = colors.next();

                    report.add_label(
                        Label::new(start..end)
                            .with_message(format!("{}", diagnostic.message.clone().fg(curr_color)))
                            .with_color(curr_color),
                    );
                }

                if let Some(ast) = ast.to_symbol() {
                    report.add_note(format!("{}", ast.read()));
                }

                Err(Box::new(AriadneReport {
                    result: report.finish(),
                    cache: source,
                }))
            }
            true => Ok(()),
        }
    }
}
