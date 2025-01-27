use crate::{self as auto_lsp};
use auto_lsp::core::ast::{AstSymbol, BuildCodeLens, Check, GetHover, GetSymbolData, Scope, BuildDocumentSymbols, BuildInlayHints, BuildSemanticTokens, VecOrSymbol};
use auto_lsp::core::document::Document;
use auto_lsp::{configure_parsers, define_semantic_token_types, choice, seq};

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
    fn build_inlay_hints(&self, doc: &Document, acc: &mut Vec<auto_lsp::lsp_types::InlayHint>) {
        for function in &self.functions {
            function.read().build_inlay_hints(doc, acc);
        }
    }
}

impl BuildDocumentSymbols for Module {
    fn get_document_symbols(&self, doc: &Document) -> Option<VecOrSymbol> {
        self.functions.get_document_symbols(doc)
    }
}

impl BuildSemanticTokens for Module {
    fn build_semantic_tokens(&self, doc: &Document, builder: &mut auto_lsp_core::semantic_tokens_builder::SemanticTokensBuilder) {
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
    parameters: Parameters,
    body: Body,
}

#[seq(query_name = "parameters", kind(symbol(
    scope(user)
)))]
pub struct Parameters {
    parameters: Vec<Parameter>,
}

impl Scope for Parameters {
    fn get_scope_range(&self) -> Vec<[usize; 2]> {
        vec![]
    }
}

#[seq(query_name = "body", kind(symbol(
    lsp_inlay_hints(code_gen(query = true)),
)))]
pub struct Body {
    pub statements: Vec<Statement>,
}

#[choice]
enum Statement {
    SimpleStatement(SimpleStatement),
    CompoundStatement(CompoundStatement),
}

#[choice]
enum SimpleStatement {
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

#[seq(query_name = "pass_statement", kind(symbol(
    lsp_hover_info(user)
)))]
pub struct PassStatement {}

impl GetHover for PassStatement {
    fn get_hover(&self, _doc: &Document) -> Option<lsp_types::Hover> {
        Some(lsp_types::Hover {
            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::Markdown,
                value: r#"This is a pass statement

[See python doc](https://docs.python.org/3/reference/simple_stmts.html#the-pass-statement)"#.into(),
            }),
            range: None
        })
    }
}

#[seq(query_name = "any", kind(symbol()))]
pub struct Any {

}

#[seq(query_name = "augmented_assignment", kind(symbol()))]
pub struct AugmentedAssignment {
    left: Identifier,
    right: Any,
}

#[seq(query_name = "assignment", kind(symbol()))]
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
        matches!(self, Expression::PrimaryExpression(PrimaryExpression::Integer(_)))
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Expression::PrimaryExpression(PrimaryExpression::Float(_)))
    }

    pub fn is_true(&self) -> bool {
        matches!(self, Expression::PrimaryExpression(PrimaryExpression::True(_)))
    }

    pub fn is_false(&self) -> bool {
        matches!(self, Expression::PrimaryExpression(PrimaryExpression::False(_)))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Expression::PrimaryExpression(PrimaryExpression::String(_)))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Expression::PrimaryExpression(PrimaryExpression::None(_)))
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

impl Scope for Function {
    fn get_scope_range(&self) -> Vec<[usize; 2]> {
        vec![]
    }
}

impl BuildInlayHints for Function {
    fn build_inlay_hints(&self, doc: &Document, acc: &mut Vec<auto_lsp::lsp_types::InlayHint>) {
        let read = self.name.read();
        acc.push(auto_lsp::lsp_types::InlayHint {
            kind: Some(auto_lsp::lsp_types::InlayHintKind::TYPE),
            label: auto_lsp::lsp_types::InlayHintLabel::String(
                match read.get_text(doc.texter.text.as_bytes()) {
                    Some(text) => text.to_string(),
                    None => "".to_string(),
                }
            ),
            position: read.get_start_position(doc),
            tooltip: None,
            text_edits: None,
            padding_left: None,
            padding_right: None,
            data: None
        });
        self.body.read().build_inlay_hints(doc, acc);
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

#[seq(query_name = "identifier", kind(symbol(
    lsp_hover_info(user)
)))]
struct Identifier {}

impl GetHover for Identifier {
    fn get_hover(&self, doc: &Document) -> Option<lsp_types::Hover> {
        let parent = self.get_parent().unwrap().to_dyn().unwrap();
        let comment = parent.read().get_comment(doc.texter.text.as_bytes());
        Some(lsp_types::Hover {
            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::PlainText,
                value: format!("{}hover {}", 
                    if let Some(comment) = comment { format!("{}\n", comment) } else { "".to_string() },
                    self.get_text(doc.texter.text.as_bytes()).unwrap()).into(),
            }),
            range: None,
        })
    }
}

#[choice]
enum Parameter {
    Identifier(Identifier),
    Typed(TypedParameter),
    TypedDefault(TypedDefaultParameter),
}

#[seq(query_name = "untyped_parameter", kind(symbol()))]
struct UntypedParameter {}

#[seq(query_name = "typed_parameter", kind(symbol(
    
)))]
struct TypedParameter {
    name: Identifier,
    parameter_type: Type
}

#[seq(query_name = "typed_default_parameter", kind(symbol(
    check(user)
)))]
struct TypedDefaultParameter {
    name: Identifier,
    parameter_type: Type,
    value: Expression
}

#[choice]
enum Type {
    Expression(Expression)
}

impl Check for TypedDefaultParameter {
    fn check(&self, doc: &Document, diagnostics: &mut Vec<lsp_types::Diagnostic>) -> Result<(), ()> {
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
            _ => Err(())
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
            message:  format!("Invalid value {} for type {}", 
            self.value.read().get_text(source_code).unwrap(), self.parameter_type.read().get_text(source_code).unwrap()),
            related_information: None,
            tags: None,
            data: None,
        }
    }
}

#[seq(query_name = "integer", kind(symbol()))]
struct Integer {}

#[seq(query_name = "float", kind(symbol()))]
struct Float {}

#[seq(query_name = "string", kind(symbol()))]
struct String {}

#[seq(query_name = "true", kind(symbol()))]
struct True {}

#[seq(query_name = "false", kind(symbol()))]
struct False {}

#[seq(query_name = "none", kind(symbol()))]
struct None {}