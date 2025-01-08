use super::super::common::name::{Name, NameBuilder};
use auto_lsp::auto_lsp_core::symbol::*;
use auto_lsp::auto_lsp_core::pending_symbol::*;
use auto_lsp::auto_lsp_core::find::Finder;
use auto_lsp::auto_lsp_macros::seq;
use auto_lsp::lsp_types::Diagnostic;

#[seq(query_name = "function", kind(symbol(
    lsp_document_symbols( 
        code_gen(
            name = self::name,
            kind = auto_lsp::lsp_types::SymbolKind::FUNCTION,
            childrens(self::input_variables, self::output_variables)
        )
    ),
    lsp_inlay_hints(code_gen(query = true)),
    scope(user),
    comment(user)
)))]
pub struct Function {
    name: Name,
    input_variables: Vec<InputVariable>,
    output_variables: Vec<OutputVariable>,
    body: Option<Assignment>,
}

impl Scope for Function {
    fn get_scope_range(&self) -> Vec<[usize; 2]> {
        let range = self.get_range();
        vec!([range.start, range.end])
    }
}

#[seq(query_name = "variable.input", kind(symbol(
    lsp_document_symbols(
        code_gen(
            name = self::name,
            kind = auto_lsp::lsp_types::SymbolKind::VARIABLE,
        )
    ),
    lsp_inlay_hints(code_gen(query = true)),
)))]
pub struct InputVariable {
    name: Name,
}

#[seq(query_name = "variable.output", kind(symbol(
    lsp_document_symbols(
        code_gen(
            name = self::name,
            kind = auto_lsp::lsp_types::SymbolKind::VARIABLE,
        )
    ),
    lsp_inlay_hints(code_gen(query = true)),
)))]
pub struct OutputVariable {
    name: Name,
}

#[seq(
    query_name = "stmt.assign",
    kind(symbol(lsp_inlay_hints(code_gen(query = true))))
)]
pub struct Assignment {
    control_variable: VariableAccess,
}

#[seq(query_name = "variable.symbolic", kind(accessor(
    lsp_go_to_definition(reference),
    lsp_go_to_declaration(reference),
    lsp_hover_info(reference),
)))]
pub struct VariableAccess {}

impl Accessor for VariableAccess {
    fn find(
        &self,
        doc: &auto_lsp::lsp_textdocument::FullTextDocument,
    ) -> Result<Option<DynSymbol>, Diagnostic> {
        if let Some(node) = self.find_in_file(doc) {
            return Ok(Some(node))
        } else {
            Err(Diagnostic::new(
                self.get_lsp_range(doc),
                None,
                None,
                None,
                format!(
                    "Could not find variable {:?}",
                    self.get_text(doc.get_content(None).as_bytes())
                ),
                None,
                None,
            ))
        }
    }
}