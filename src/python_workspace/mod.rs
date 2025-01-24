use crate::{self as auto_lsp};
use auto_lsp::core::ast::{AstSymbol, BuildCodeLens, Check, GetHover, GetSymbolData, Scope, BuildDocumentSymbols, BuildInlayHints, BuildSemanticTokens, VecOrSymbol};
use auto_lsp::core::document::Document;
use auto_lsp::{configure_parsers, define_semantic_token_types, choice, seq};

static CORE_QUERY: &'static str = "
(module) @module

(function_definition
  name: (identifier) @identifier
  parameters: (_) @parameters 
  body: (_) @body
) @function

(parameter
  ((identifier) @untyped_parameter [ \",\" \")\"])
)

(typed_parameter
	. (identifier) @identifier
    type: [
    		((_) @bool (#eq? @bool \"bool\"))
          	((_) @complex (#eq? @complex \"complex\"))
    	  	((_) @int (#eq? @int \"int\"))
          	((_) @float (#eq? @float \"float\"))
            ((_) @str (#eq? @str \"str\"))
          ]
) @typed_parameter

(typed_default_parameter
	name: (identifier) @identifier
    type: [
    		((_) @bool (#eq? @bool \"bool\"))
          	((_) @complex (#eq? @complex \"complex\"))
    	  	((_) @int (#eq? @int \"int\"))
          	((_) @float (#eq? @float \"float\"))
            ((_) @str (#eq? @str \"str\"))
          ]
    value: (_) @any
) @typed_default_parameter

(assignment
	(_) @identifier
	\"=\"
    (_) @any
) @assignment
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

#[seq(query_name = "parameters", kind(symbol()))]
pub struct Parameters {
    parameters: Vec<Parameter>,
}

#[seq(query_name = "body", kind(symbol()))]
pub struct Body {
    pub statements: Vec<Statement>,
}

#[choice]
enum Statement {
    Assignment(Assignment),
}

#[seq(query_name = "assignment", kind(symbol()))]
pub struct Assignment {
    left: Identifier,
    right: Any,
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
                read.get_text(doc.texter.text.as_bytes()).unwrap().into()
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
    Untyped(UntypedParameter),
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
    default: Any
}

impl Check for TypedDefaultParameter {
    fn check(&self, doc: &Document, diagnostics: &mut Vec<lsp_types::Diagnostic>) -> Result<(), ()> {
        let source = doc.texter.text.as_bytes();
        match self.parameter_type.read().check_value(self.default.read().get_text(source).unwrap()) {
            true => Ok(()),
            false => {
                diagnostics.push(lsp_types::Diagnostic {
                    range: self.get_lsp_range(doc),
                    severity: Some(lsp_types::DiagnosticSeverity::ERROR),
                    code: None,
                    code_description: None,
                    source: None,
                    message: format!("Invalid value {} for type {}", 
                        self.default.read().get_text(source).unwrap(),
                        self.parameter_type.read().get_text(source).unwrap()),
                    related_information: None,
                    tags: None,
                    data: None,
                });
                Err(())
            }
        }
    }
}

#[choice]
enum Type {
    Bool(Bool),
    Complex(Complex),
    Int(Int),
    Float(Float),
    Str(Str),
}

pub trait CheckPrimitive {
    fn check_value(&self, value: &str) -> bool;
}

impl CheckPrimitive for Type {
    fn check_value(&self, value: &str) -> bool {
        match self {
            Type::Bool(t) => t.check_value(value),
            Type::Complex(t) => t.check_value(value),
            Type::Int(t) => t.check_value(value),
            Type::Float(t) => t.check_value(value),
            Type::Str(t) => t.check_value(value),
        }
    }
}

#[seq(query_name = "bool", kind(symbol()))]
struct Bool {}

impl CheckPrimitive for Bool {
    fn check_value(&self, value: &str) -> bool {
        value.parse::<bool>().is_ok()
    }
}

#[seq(query_name = "complex", kind(symbol()))]
struct Complex {}

impl CheckPrimitive for Complex {
    fn check_value(&self, value: &str) -> bool {
        value.parse::<f64>().is_ok()
    }
}

#[seq(query_name = "int", kind(symbol()))]
struct Int {}

impl CheckPrimitive for Int {
    fn check_value(&self, value: &str) -> bool {
        value.parse::<i64>().is_ok()
    }
}

#[seq(query_name = "float", kind(symbol()))]
struct Float {}

impl CheckPrimitive for Float {
    fn check_value(&self, value: &str) -> bool {
        value.parse::<f64>().is_ok()
    }
}

#[seq(query_name = "str", kind(symbol()))]
struct Str {}

impl CheckPrimitive for Str {
    fn check_value(&self, value: &str) -> bool {
        value.starts_with("\"") && value.ends_with("\"")
    }
}