/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/

use std::ops::Deref;

use super::ast::{
    CompoundStatement, Expression, PrimaryExpression, Statement, TypedDefaultParameter,
};
use crate::{
    self as auto_lsp,
    python::ast::{Module, Parameter},
};
use auto_lsp::core::ast::AstSymbol;
use auto_lsp_core::{
    document::Document,
    salsa::{
        db::{BaseDatabase, File},
        tracked::get_ast,
    },
};
use salsa::Accumulator;

#[salsa::accumulator]
pub struct CheckErrorAccumulator(pub lsp_types::Diagnostic);

#[salsa::tracked]
pub(crate) fn type_check_default_parameters<'db>(db: &'db dyn BaseDatabase, file: File) {
    let doc = file.document(db).read();
    let root = get_ast(db, file).to_symbol();

    let module = root.as_ref().unwrap();
    let module = module.read();
    let module = module.downcast_ref::<Module>().unwrap();

    for node in &module.statements {
        if let Statement::Compound(CompoundStatement::Function(function)) = node.read().deref() {
            function
                .parameters
                .read()
                .parameters
                .iter()
                .for_each(|param| {
                    if let Parameter::TypedDefault(typed_param) = param.read().deref() {
                        typed_param.check(db, &doc);
                    }
                });
        }
    }
}

impl TypedDefaultParameter {
    fn check(&self, db: &dyn BaseDatabase, doc: &Document) {
        let source = doc.texter.text.as_bytes();

        match self.parameter_type.read().get_text(source).unwrap() {
            "int" => match self.value.read().is_integer() {
                true => (),
                false => {
                    CheckErrorAccumulator::accumulate(self.type_error_message(doc), db);
                }
            },
            "float" => match self.value.read().is_float() {
                true => (),
                false => {
                    CheckErrorAccumulator::accumulate(self.type_error_message(doc), db);
                }
            },
            "str" => match self.value.read().is_string() {
                true => (),
                false => {
                    CheckErrorAccumulator::accumulate(self.type_error_message(doc), db);
                }
            },
            "bool" => match self.value.read().is_true() || self.value.read().is_false() {
                true => (),
                false => {
                    CheckErrorAccumulator::accumulate(self.type_error_message(doc), db);
                }
            },
            _ => {
                CheckErrorAccumulator::accumulate(self.type_error_message(doc).into(), db);
            }
        }
    }
}

impl TypedDefaultParameter {
    fn type_error_message(&self, document: &Document) -> CheckErrorAccumulator {
        let source_code = document.texter.text.as_bytes();
        CheckErrorAccumulator(lsp_types::Diagnostic {
            range: self.get_lsp_range(document).unwrap(),
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
        })
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
