use auto_lsp::core::ast::{
    AstSymbol, BuildDocumentSymbols, GetSymbolData, IsComment, BuildSemanticTokens, StaticUpdate, Symbol,
    VecOrSymbol,
};
use auto_lsp::core::build::MainBuilder;
use auto_lsp::core::workspace::{Document, Workspace};
use auto_lsp::macros::seq;
use auto_lsp_core::ast::GetHoverInfo;
use criterion::{criterion_group, BatchSize, Criterion};
use lsp_types::Url;
use std::sync::{Arc, LazyLock};
use texter::core::text::Text;

use auto_lsp::session::Session;
use auto_lsp::{self as auto_lsp, define_semantic_token_types};

use auto_lsp::configure_parsers;

static CORE_QUERY: &'static str = "
(module) @module

(function_definition
  name: (identifier) @function.name) @function
";

static COMMENT_QUERY: &'static str = "
(comment) @comment
";

configure_parsers!(
    "python" => {
        language: tree_sitter_python::LANGUAGE,
        ast_root: Module,
        core: CORE_QUERY,
        comment: Some(COMMENT_QUERY),
        fold: None,
        highlights: None
    }
);

define_semantic_token_types!(standard {
    "Function" => FUNCTION,
});

#[seq(query_name = "module", kind(symbol(lsp_document_symbols(user), lsp_semantic_tokens(user))))]
struct Module {
    functions: Vec<Function>,
}

impl BuildDocumentSymbols for Module {
    fn get_document_symbols(&self, doc: &Document) -> Option<VecOrSymbol> {
        self.functions.get_document_symbols(doc)
    }
}

impl BuildSemanticTokens for Module {
    fn build_semantic_tokens(&self, doc: &Document, builder: &mut auto_lsp_core::semantic_tokens::SemanticTokensBuilder) {
        for function in &self.functions {
            function.read().build_semantic_tokens(doc, builder);
        }
    }
}

#[seq(query_name = "function", kind(symbol(
    lsp_document_symbols( 
        code_gen(
            name = self::name,
            kind = auto_lsp::lsp_types::SymbolKind::FUNCTION,
        )
    ),
    lsp_semantic_tokens(
        code_gen(
            range = self::name,
            token_types = TOKEN_TYPES,
            token_type_index = "Function"
        )
    ),
    comment(user)
)))]
struct Function {
    name: FunctionName,
}
#[seq(query_name = "function.name", kind(symbol(
    lsp_hover_info(user)
)))]
struct FunctionName {}

impl GetHoverInfo for FunctionName {
    fn get_hover(&self, doc: &Document) -> Option<lsp_types::Hover> {
        Some(lsp_types::Hover {
            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::PlainText,
                value: format!("hover {}", self.get_text(doc.document.text.as_bytes()).unwrap()).into(),
            }),
            range: None,
        })
    }
}

fn create_python_workspace(uri: Url, source_code: String) -> Workspace {
    let parse = PARSERS.get("python").unwrap();

    let tree = parse
        .cst_parser
        .parser
        .write()
        .parse(source_code.as_bytes(), None)
        .unwrap();

    let document = Document {
        document: Text::new(source_code.into()),
        cst: tree,
    };

    let mut diagnostics = vec![];
    let mut unsolved_checks = vec![];
    let mut unsolved_references = vec![];

    let mut params = MainBuilder {
        query: &parse.cst_parser.queries.core,
        document: &document,
        url: Arc::new(uri),
        diagnostics: &mut diagnostics,
        unsolved_checks: &mut unsolved_checks,
        unsolved_references: &mut unsolved_references,
    };

    let ast_parser = parse.ast_parser;
    let ast = ast_parser(&mut params, None).unwrap();

    let workspace = Workspace {
        parsers: parse,
        document,
        errors: diagnostics,
        ast: Some(ast),
        unsolved_checks,
        unsolved_references,
    };

    Session::add_comments(&workspace).unwrap();

    workspace
}

fn parse_django(c: &mut Criterion) {
    let text = include_str!("django.py").to_string();
    c.bench_function("parse_django", move |b| {
        b.iter(|| {
            let uri = Url::parse("file:///test.py").unwrap();
            let workspace = create_python_workspace(uri, text.clone());
            workspace
        });

    });
}

criterion_group!(benches, parse_django);