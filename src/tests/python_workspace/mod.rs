#![allow(deprecated)]
use std::sync::LazyLock;

use crate::{self as auto_lsp};
use auto_lsp::core::ast::{
    AstSymbol, BuildCodeLens, BuildDocumentSymbols, BuildInlayHints, BuildSemanticTokens, Check,
    GetHover, GetSymbolData, Scope,
};
use auto_lsp::core::document::Document;
use auto_lsp::{choice, configure_parsers, define_semantic_token_types, seq};
use auto_lsp_core::ast::BuildCompletionItems;
use auto_lsp_core::document_symbols_builder::DocumentSymbolsBuilder;
use auto_lsp_core::semantic_tokens_builder::SemanticTokensBuilder;

static CORE_QUERY: &'static str = "
(module) @module
(identifier) @identifier

(integer) @integer
(float) @float
(string) @string
(true) @true
(false) @false

(function_definition
  body: (_) @body 
) @function

(parameters) @parameters
(typed_parameter) @typed_parameter
(typed_default_parameter) @typed_default_parameter

(pass_statement) @pass_statement
(assignment) @assignment
";

static COMMENT_QUERY: &'static str = "
(comment) @comment
";

configure_parsers!(
    PYTHON_PARSERS,
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

/// Globally available completion items
static GLOBAL_COMPLETION_ITEMS: LazyLock<Vec<lsp_types::CompletionItem>> = LazyLock::new(|| {
    vec![lsp_types::CompletionItem {
        label: "def ...".to_string(),
        kind: Some(lsp_types::CompletionItemKind::SNIPPET),
        insert_text_format: Some(lsp_types::InsertTextFormat::SNIPPET),
        insert_text: Some("def ${1:func_name}(${2:arg1}):$0".to_string()),
        ..Default::default()
    }]
});

#[seq(
    query = "module",
    code_lenses,
    document_symbols,
    completions,
    inlay_hints,
    semantic_tokens
)]
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
    fn build_inlay_hints(&self, doc: &Document, acc: &mut Vec<auto_lsp::lsp_types::InlayHint>) {
        for function in &self.functions {
            function.read().build_inlay_hints(doc, acc);
        }
    }
}

impl BuildSemanticTokens for Module {
    fn build_semantic_tokens(
        &self,
        doc: &Document,
        builder: &mut auto_lsp_core::semantic_tokens_builder::SemanticTokensBuilder,
    ) {
        for function in &self.functions {
            function.read().build_semantic_tokens(doc, builder);
        }
    }
}

impl BuildDocumentSymbols for Module {
    fn build_document_symbols(&self, doc: &Document, builder: &mut DocumentSymbolsBuilder) {
        self.functions.build_document_symbols(doc, builder);
    }
}

impl BuildCompletionItems for Module {
    fn build_completion_items(&self, _doc: &Document, acc: &mut Vec<lsp_types::CompletionItem>) {
        acc.extend(GLOBAL_COMPLETION_ITEMS.iter().cloned());
    }
}

#[seq(
    query = "function",
    document_symbols,
    code_lenses,
    inlay_hints,
    scope,
    comment,
    completions,
    semantic_tokens
)]
struct Function {
    name: Identifier,
    parameters: Parameters,
    body: Body,
}

impl Scope for Function {
    fn get_scope_range(&self) -> Vec<[usize; 2]> {
        vec![]
    }
}

impl BuildDocumentSymbols for Function {
    fn build_document_symbols(&self, doc: &Document, builder: &mut DocumentSymbolsBuilder) {
        let mut nested_builder = DocumentSymbolsBuilder::default();

        self.body
            .read()
            .build_document_symbols(doc, &mut nested_builder);

        builder.push_symbol(lsp_types::DocumentSymbol {
            name: self
                .name
                .read()
                .get_text(doc.texter.text.as_bytes())
                .unwrap()
                .to_string(),
            kind: lsp_types::SymbolKind::FUNCTION,
            range: self.name.read().get_lsp_range(doc),
            selection_range: self.name.read().get_lsp_range(doc),
            tags: None,
            detail: None,
            deprecated: None,
            children: Some(nested_builder.finalize()),
        });
    }
}

impl BuildInlayHints for Function {
    fn build_inlay_hints(&self, doc: &Document, acc: &mut Vec<auto_lsp::lsp_types::InlayHint>) {
        let range = self.get_range();
        let read = self.name.read();
        let name = format!(
            "[{} {}] - {}",
            range.start,
            range.end,
            self.name
                .read()
                .get_text(doc.texter.text.as_bytes())
                .unwrap()
        );
        acc.push(auto_lsp::lsp_types::InlayHint {
            kind: Some(auto_lsp::lsp_types::InlayHintKind::TYPE),
            label: auto_lsp::lsp_types::InlayHintLabel::String(name),
            position: read.get_start_position(doc),
            tooltip: None,
            text_edits: None,
            padding_left: None,
            padding_right: None,
            data: None,
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

impl BuildCompletionItems for Function {
    fn build_completion_items(&self, _doc: &Document, acc: &mut Vec<lsp_types::CompletionItem>) {
        acc.extend(GLOBAL_COMPLETION_ITEMS.iter().cloned());
    }
}

impl BuildSemanticTokens for Function {
    fn build_semantic_tokens(&self, doc: &Document, builder: &mut SemanticTokensBuilder) {
        builder.push(
            self.name.read().get_lsp_range(doc),
            TOKEN_TYPES.get_index("Function").unwrap() as u32,
            0,
        );
    }
}

#[seq(query = "parameters", scope)]
pub struct Parameters {
    parameters: Vec<Parameter>,
}

impl Scope for Parameters {
    fn get_scope_range(&self) -> Vec<[usize; 2]> {
        vec![]
    }
}

#[seq(query = "body", document_symbols, completions)]
pub struct Body {
    pub statements: Vec<Statement>,
}

impl BuildDocumentSymbols for Body {
    fn build_document_symbols(&self, doc: &Document, builder: &mut DocumentSymbolsBuilder) {
        self.statements.build_document_symbols(doc, builder);
    }
}

impl BuildCompletionItems for Body {
    fn build_completion_items(&self, _doc: &Document, acc: &mut Vec<lsp_types::CompletionItem>) {
        acc.extend(GLOBAL_COMPLETION_ITEMS.iter().cloned());
    }
}

#[choice]
enum Statement {
    SimpleStatement(SimpleStatement),
    CompoundStatement(CompoundStatement),
}

#[choice]
enum SimpleStatement {
    Expression(Expression),
    ExpressionStatement(ExpressionStatement),
    PassStatement(PassStatement),
}

#[choice]
enum CompoundStatement {
    Function(Function),
}

#[choice]
pub enum ExpressionStatement {
    Assignment(Assignment),
    AugmentedAssignent(AugmentedAssignment),
    Expression(Expression),
}

#[seq(query = "pass_statement", hover)]
pub struct PassStatement {}

impl GetHover for PassStatement {
    fn get_hover(&self, _doc: &Document) -> Option<lsp_types::Hover> {
        Some(lsp_types::Hover {
            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::Markdown,
                value: r#"This is a pass statement

[See python doc](https://docs.python.org/3/reference/simple_stmts.html#the-pass-statement)"#
                    .into(),
            }),
            range: None,
        })
    }
}

#[seq(query = "any")]
pub struct Any {}

#[seq(query = "augmented_assignment")]
pub struct AugmentedAssignment {
    left: Identifier,
    right: Any,
}

#[seq(query = "assignment")]
pub struct Assignment {
    left: Identifier,
    right: Any,
}

#[choice]
pub enum Expression {
    PrimaryExpression(PrimaryExpression),
}

impl Expression {
    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            Expression::PrimaryExpression(PrimaryExpression::Integer(_))
        )
    }

    pub fn is_float(&self) -> bool {
        matches!(
            self,
            Expression::PrimaryExpression(PrimaryExpression::Float(_))
        )
    }

    pub fn is_true(&self) -> bool {
        matches!(
            self,
            Expression::PrimaryExpression(PrimaryExpression::True(_))
        )
    }

    pub fn is_false(&self) -> bool {
        matches!(
            self,
            Expression::PrimaryExpression(PrimaryExpression::False(_))
        )
    }

    pub fn is_string(&self) -> bool {
        matches!(
            self,
            Expression::PrimaryExpression(PrimaryExpression::String(_))
        )
    }

    pub fn is_none(&self) -> bool {
        matches!(
            self,
            Expression::PrimaryExpression(PrimaryExpression::None(_))
        )
    }
}

#[choice]
pub enum PrimaryExpression {
    Identifier(Identifier),
    Integer(Integer),
    Float(Float),
    True(True),
    False(False),
    String(String),
    None(None),
}

#[seq(query = "identifier", hover, completions)]
struct Identifier {}

impl GetHover for Identifier {
    fn get_hover(&self, doc: &Document) -> Option<lsp_types::Hover> {
        let parent = self.get_parent().unwrap().to_dyn().unwrap();
        let comment = parent.read().get_comment(doc.texter.text.as_bytes());
        Some(lsp_types::Hover {
            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::PlainText,
                value: format!(
                    "{}hover {}",
                    if let Some(comment) = comment {
                        format!("{}\n", comment)
                    } else {
                        "".to_string()
                    },
                    self.get_text(doc.texter.text.as_bytes()).unwrap()
                )
                .into(),
            }),
            range: None,
        })
    }
}

impl BuildCompletionItems for Identifier {
    fn build_completion_items(&self, doc: &Document, acc: &mut Vec<lsp_types::CompletionItem>) {
        match self.get_parent_scope() {
            Some(scope) => {
                scope.build_completion_items(doc, acc);
            }
            None => {
                acc.extend(GLOBAL_COMPLETION_ITEMS.iter().cloned());
            }
        }
    }
}

#[choice]
enum Parameter {
    Identifier(Identifier),
    Typed(TypedParameter),
    TypedDefault(TypedDefaultParameter),
}

#[seq(query = "untyped_parameter")]
struct UntypedParameter {}

#[seq(query = "typed_parameter")]
struct TypedParameter {
    name: Identifier,
    parameter_type: Type,
}

#[seq(query = "typed_default_parameter", check)]
struct TypedDefaultParameter {
    name: Identifier,
    parameter_type: Type,
    value: Expression,
}

#[choice]
enum Type {
    Expression(Expression),
}

impl Check for TypedDefaultParameter {
    fn check(
        &self,
        doc: &Document,
        diagnostics: &mut Vec<lsp_types::Diagnostic>,
    ) -> Result<(), ()> {
        let source = doc.texter.text.as_bytes();

        match self.parameter_type.read().get_text(source).unwrap() {
            "int" => match self.value.read().is_integer() {
                true => Ok(()),
                false => {
                    diagnostics.push(self.type_error_message(doc));
                    return Err(());
                }
            },
            "float" => match self.value.read().is_float() {
                true => Ok(()),
                false => {
                    diagnostics.push(self.type_error_message(doc));
                    return Err(());
                }
            },
            "str" => match self.value.read().is_string() {
                true => Ok(()),
                false => {
                    diagnostics.push(self.type_error_message(doc));
                    return Err(());
                }
            },
            "bool" => match self.value.read().is_true() || self.value.read().is_false() {
                true => Ok(()),
                false => {
                    diagnostics.push(self.type_error_message(doc));
                    return Err(());
                }
            },
            _ => Err(()),
        }
    }
}

impl TypedDefaultParameter {
    fn type_error_message(&self, document: &Document) -> lsp_types::Diagnostic {
        let source_code = document.texter.text.as_bytes();
        lsp_types::Diagnostic {
            range: self.get_lsp_range(document),
            severity: Some(lsp_types::DiagnosticSeverity::ERROR),
            code: None,
            code_description: None,
            source: None,
            message: format!(
                "Invalid value {} for type {}",
                self.value.read().get_text(source_code).unwrap(),
                self.parameter_type.read().get_text(source_code).unwrap()
            ),
            related_information: None,
            tags: None,
            data: None,
        }
    }
}

#[seq(query = "integer")]
struct Integer {}

#[seq(query = "float")]
struct Float {}

#[seq(query = "string")]
struct String {}

#[seq(query = "true")]
struct True {}

#[seq(query = "false")]
struct False {}

#[seq(query = "none")]
struct None {}
