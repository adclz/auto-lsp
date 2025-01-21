use crate::core::build::MainBuilder;
use crate::core::ast::{AstSymbol, BuildDocumentSymbols, BuildInlayHints, BuildSemanticTokens, Symbol, VecOrSymbol};
use crate::core::workspace::{Document, Workspace};
use crate::macros::seq;
use auto_lsp_core::ast::{BuildCodeLens, GetHover, GetSymbolData, Scope};
use auto_lsp_macros::choice;
use lsp_types::Url;
use std::sync::Arc;
use texter::core::text::Text;

use crate::server::Session;
use crate::{self as auto_lsp, define_semantic_token_types};

use crate::configure_parsers;

static CORE_QUERY: &'static str = "
(module) @module

(function_definition
  name: (identifier) @identifier) @function

(parameter
  ((identifier) @untyped_parameter [ \",\" \")\"])
)

(typed_parameter
	. (_) @any
    type: (_) @any2
) @typed_parameter

(typed_default_parameter
	name: (identifier) @identifier
    type: (_) @any
    value: (_) @any2
) @typed_default_parameter
";

static COMMENT_QUERY: &'static str = "
(comment) @comment
";

configure_parsers!(
    "python" => {
        language: tree_sitter_python::LANGUAGE,
        node_types: tree_sitter_python::NODE_TYPES,
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

#[seq(query_name = "module", kind(symbol(
    lsp_document_symbols(user), 
    lsp_semantic_tokens(user),
    lsp_inlay_hints(user),
    lsp_code_lens(user)
)))]
struct Module {
    functions: Vec<Function>,
}

impl BuildCodeLens for Module {
    fn build_code_lens(&self, doc: &Document, acc: &mut Vec<lsp_types::CodeLens>) {
        for function in &self.functions {
            function.read().build_code_lens(doc, acc);
        }
    }
}

impl BuildInlayHints for Module {
    fn build_inlay_hint(&self, doc: &Document, acc: &mut Vec<auto_lsp::lsp_types::InlayHint>) {
        for function in &self.functions {
            function.read().build_inlay_hint(doc, acc);
        }
    }
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
    lsp_inlay_hints(user),
    lsp_code_lens(user),
    comment(user),
    scope(user)
)))]
struct Function {
    name: Identifier,
    parameters: Vec<Parameter>
}

impl Scope for Function {
    fn get_scope_range(&self) -> Vec<[usize; 2]> {
        vec![]
    }
}

impl BuildInlayHints for Function {
    fn build_inlay_hint(&self, doc: &Document, acc: &mut Vec<auto_lsp::lsp_types::InlayHint>) {
        let read = self.name.read();
        acc.push(auto_lsp::lsp_types::InlayHint {
            kind: Some(auto_lsp::lsp_types::InlayHintKind::TYPE),
            label: auto_lsp::lsp_types::InlayHintLabel::String(
                read.get_text(doc.document.text.as_bytes()).unwrap().into()
            ),
            position: read.get_start_position(doc),
            tooltip: None,
            text_edits: None,
            padding_left: None,
            padding_right: None,
            data: None
        });
    }
}

impl BuildCodeLens for Function {
    fn build_code_lens(&self, doc: &Document, acc: &mut Vec<lsp_types::CodeLens>) {
        let read = self.name.read();
        acc.push(lsp_types::CodeLens {
            range: read.get_lsp_range(&doc),
            command: None,
            data: None,
        })
    }
}

#[seq(query_name = "any", kind(symbol(
)))]
struct Any {}

#[seq(query_name = "any2", kind(symbol(
)))]
struct Any2 {}

#[seq(query_name = "identifier", kind(symbol(
    lsp_hover_info(user)
)))]
struct Identifier {}

impl GetHover for Identifier {
    fn get_hover(&self, doc: &Document) -> Option<lsp_types::Hover> {
        let parent = self.get_parent().unwrap().to_dyn().unwrap();
        let comment = parent.read().get_comment(doc.document.text.as_bytes());
        Some(lsp_types::Hover {
            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::PlainText,
                value: format!("{}hover {}", 
                    if let Some(comment) = comment { format!("{}\n", comment) } else { "".to_string() },
                    self.get_text(doc.document.text.as_bytes()).unwrap()).into(),
            }),
            range: None,
        })
    }
}

#[choice]
enum Parameter {
    Untyped(UntypedParameter),
    Typed(TypedParameter),
    TypedDefault(TypedDefaultParameter),
}

#[seq(query_name = "untyped_parameter", kind(symbol()))]
struct UntypedParameter {}

#[seq(query_name = "typed_parameter", kind(symbol(
    
)))]
struct TypedParameter {
    name: Any,
    _type: Any2
}

#[seq(query_name = "typed_default_parameter", kind(symbol()))]
struct TypedDefaultParameter {
    name: Identifier,
    _type: Any,
    default: Any2
}

pub fn create_python_workspace(uri: Url, source_code: String) -> Workspace {
    let parse = PARSERS.get("python").unwrap();

    let tree = parse
        .tree_sitter
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
        query: &parse.tree_sitter.queries.core,
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