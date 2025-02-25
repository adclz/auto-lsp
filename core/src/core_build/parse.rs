use crate::ast::DynSymbol;
use crate::document::Document;
use crate::workspace::Parsers;
use crate::workspace::Workspace;
use crate::{
    ast::AstSymbol,
    build::{Buildable, Queryable, TryFromBuilder},
};
use ariadne::Fmt;
use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
use lsp_types::Url;

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
        workspace: &mut Workspace,
        document: &Document,
        range: Option<std::ops::Range<usize>>,
    ) -> Result<Y, lsp_types::Diagnostic>;

    fn parse_symbols(
        workspace: &mut Workspace,
        document: &Document,
        range: Option<std::ops::Range<usize>>,
    ) -> Result<Vec<Y>, lsp_types::Diagnostic>;
}

impl<T, Y> InvokeParser<T, Y> for Y
where
    T: Buildable + Queryable,
    Y: AstSymbol + for<'b> TryFromBuilder<&'b T, Error = lsp_types::Diagnostic>,
{
    fn parse_symbol(
        workspace: &mut Workspace,
        document: &Document,
        range: Option<std::ops::Range<usize>>,
    ) -> Result<Y, lsp_types::Diagnostic> {
        StackBuilder::<T>::new(workspace, document).create_symbol(&range)
    }
    fn parse_symbols(
        workspace: &mut Workspace,
        document: &Document,
        range: Option<std::ops::Range<usize>>,
    ) -> Result<Vec<Y>, lsp_types::Diagnostic> {
        StackBuilder::<T>::new(workspace, document).create_symbols(&range)
    }
}

/// Function signature for invoking the stack builder.
///
/// This type alias is useful for mapping language IDs to specific parsers,
/// avoiding ambiguity.
pub type InvokeParserFn = fn(
    &mut Workspace,
    &Document,
    Option<std::ops::Range<usize>>,
) -> Result<DynSymbol, lsp_types::Diagnostic>;

pub type TryParseResult<E = AriadneReport> = Result<(), E>;

#[derive(Debug)]
pub struct AriadneReport {
    pub result: Report<'static>,
    pub cache: Source<&'static str>,
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
    /// # Returns
    /// - `Ok(())` if the code was successfully parsed and validated.
    /// - `Err(Result<(), ()>)` if any parsing or validation errors occurred.
    fn try_parse(test_code: &'static str, parsers: &'static Parsers) -> Result<(), ()>;
}

impl<T, Y> TryParse<T, Y> for Y
where
    T: Buildable + Queryable,
    Y: AstSymbol + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
{
    fn try_parse(test_code: &'static str, parsers: &'static Parsers) -> Result<(), ()> {
        let source = Source::from(test_code);

        let (mut workspace, document) = match Workspace::from_utf8(
            parsers,
            Url::parse("file://test.txt").unwrap(),
            test_code.into(),
        ) {
            Ok(workspace) => workspace,
            Err(err) => {
                Report::build(ReportKind::Error, 0..source.len())
                    .with_message(err.to_string())
                    .finish()
                    .print(source)
                    .unwrap();
                return Err(());
            }
        };

        let result: Result<Y, lsp_types::Diagnostic> =
            Y::parse_symbol(&mut workspace, &document, None);

        match &workspace.diagnostics.is_empty() {
            false => {
                let mut colors = ColorGenerator::new();
                let mut report = Report::build(ReportKind::Error, 0..source.len()).with_message(
                    format!("Parsing failed: {} error(s)", workspace.diagnostics.len()),
                );

                for diagnostic in &workspace.diagnostics {
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

                if let Ok(ast) = result {
                    report.add_note(format!("\n\n {}", ast.to_string()));
                }

                report.finish().print(source).unwrap();
                Err(())
            }
            true => Ok(()),
        }
    }
}
