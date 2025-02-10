use super::ast::{Expression, PrimaryExpression, TypedDefaultParameter};
use crate::{self as auto_lsp};
use auto_lsp::core::ast::{AstSymbol, Check};
use auto_lsp_core::document::Document;

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
