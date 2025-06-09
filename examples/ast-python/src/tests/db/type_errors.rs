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
use crate::db::create_python_db;
use crate::generated::{
    CompoundStatement, CompoundStatement_SimpleStatement, Expression, Module, Parameter,
    PrimaryExpression, TypedDefaultParameter,
};
use auto_lsp::core::ast::AstNode;
use auto_lsp::core::document::Document;
use auto_lsp::default::db::tracked::get_ast;
use auto_lsp::default::db::{BaseDatabase, File, FileManager};
use auto_lsp::lsp_types::Url;
use auto_lsp::salsa::Accumulator;
use auto_lsp::{lsp_types, salsa};
use rstest::{fixture, rstest};

#[salsa::accumulator]
pub struct CheckErrorAccumulator(pub lsp_types::Diagnostic);

#[salsa::tracked]
pub(crate) fn type_check_default_parameters<'db>(db: &'db dyn BaseDatabase, file: File) {
    let doc = file.document(db);
    let root = get_ast(db, file).get_root();

    let module = root.unwrap();
    let module = module.downcast_ref::<Module>().unwrap();

    for node in &module.children {
        if let CompoundStatement_SimpleStatement::CompoundStatement(
            CompoundStatement::FunctionDefinition(function),
        ) = node.as_ref()
        {
            function.parameters.children.iter().for_each(|param| {
                if let Parameter::TypedDefaultParameter(typed_param) = param.as_ref() {
                    typed_param.check(db, &doc);
                }
            });
        }
    }
}

impl TypedDefaultParameter {
    fn check(&self, db: &dyn BaseDatabase, doc: &Document) {
        let source = doc.as_bytes();

        match self.Type.get_text(source).unwrap() {
            "int" => match self.value.is_integer() {
                true => (),
                false => {
                    CheckErrorAccumulator::accumulate(self.type_error_message(doc), db);
                }
            },
            "float" => match self.value.is_float() {
                true => (),
                false => {
                    CheckErrorAccumulator::accumulate(self.type_error_message(doc), db);
                }
            },
            "str" => match self.value.is_string() {
                true => (),
                false => {
                    CheckErrorAccumulator::accumulate(self.type_error_message(doc), db);
                }
            },
            "bool" => match self.value.is_true() || self.value.is_false() {
                true => (),
                false => {
                    CheckErrorAccumulator::accumulate(self.type_error_message(doc), db);
                }
            },
            _ => {
                CheckErrorAccumulator::accumulate(self.type_error_message(doc), db);
            }
        }
    }
}

impl TypedDefaultParameter {
    fn type_error_message(&self, document: &Document) -> CheckErrorAccumulator {
        let source_code = document.as_bytes();
        CheckErrorAccumulator(lsp_types::Diagnostic {
            range: self.get_lsp_range(),
            severity: Some(lsp_types::DiagnosticSeverity::ERROR),
            code: None,
            code_description: None,
            source: None,
            message: format!(
                "Invalid value {} for type {}",
                self.value.get_text(source_code).unwrap(),
                self.Type.get_text(source_code).unwrap()
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

#[fixture]
fn foo_bar() -> impl BaseDatabase {
    create_python_db(&[r#"# foo comment
def foo(param1, param2: int, param3: int = 5):
    pass

def bar():
    pass
"#])
}

#[fixture]
fn foo_bar_with_type_error() -> impl BaseDatabase {
    create_python_db(&[r#"# foo comment
        def foo(param1, param2: int = "string"):
            pass

        def bar():
            pass
        "#])
}

#[rstest]
fn foo_has_type_error(foo_bar: impl BaseDatabase, foo_bar_with_type_error: impl BaseDatabase) {
    let file0_url = Url::parse("file:///test0.py").unwrap();
    let file = foo_bar.get_file(&file0_url).unwrap();

    let foo_bar_diagnostics =
        type_check_default_parameters::accumulated::<CheckErrorAccumulator>(&foo_bar, file);

    // foo_bar has no type errors
    assert!(foo_bar_diagnostics.is_empty());

    let file = foo_bar_with_type_error.get_file(&file0_url).unwrap();

    let foo_bar_diagnostics = type_check_default_parameters::accumulated::<CheckErrorAccumulator>(
        &foo_bar_with_type_error,
        file,
    );

    // foo_bar_with_type_error has one type error
    assert!(!foo_bar_diagnostics.is_empty());

    assert_eq!(
        foo_bar_diagnostics[0].0.message,
        "Invalid value \"string\" for type int"
    );
}

#[fixture]
fn foo_with_type_error() -> impl BaseDatabase {
    create_python_db(&[r#"def foo(p: int = "x"): pass "#])
}

#[rstest]
fn non_redundant_edited_type_error(mut foo_with_type_error: impl BaseDatabase) {
    let file0_url = Url::parse("file:///test0.py").unwrap();
    let file = foo_with_type_error.get_file(&file0_url).unwrap();

    let foo_with_type_error_diagnostics = type_check_default_parameters::accumulated::<
        CheckErrorAccumulator,
    >(&foo_with_type_error, file);

    // test to check if a same error is not reported twice between edits of the same error

    // foo_with_type_error has one type error
    assert!(!foo_with_type_error_diagnostics.is_empty());
    assert_eq!(
        foo_with_type_error_diagnostics[0].0.message,
        "Invalid value \"x\" for type int"
    );

    // Insert "xxxx"
    // "def foo(p: int = "x"): pass " -> "def foo(p: int = "xxxx"): pass "
    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 18,
            },
            end: lsp_types::Position {
                line: 0,
                character: 19,
            },
        }),
        range_length: Some(1),
        text: "xxxx".into(),
    };

    foo_with_type_error.update(&file0_url, &[change]).unwrap();

    let foo_with_type_error_diagnostics = type_check_default_parameters::accumulated::<
        CheckErrorAccumulator,
    >(&foo_with_type_error, file);

    // foo_with_type_error should have 1 error
    assert_eq!(foo_with_type_error_diagnostics.len(), 1);
    assert_eq!(
        foo_with_type_error_diagnostics[0].0.message,
        "Invalid value \"xxxx\" for type int"
    );
}

#[rstest]
fn fix_type_error(mut foo_with_type_error: impl BaseDatabase) {
    let file0_url = Url::parse("file:///test0.py").unwrap();
    let file = foo_with_type_error.get_file(&file0_url).unwrap();

    let foo_with_type_error_diagnostics = type_check_default_parameters::accumulated::<
        CheckErrorAccumulator,
    >(&foo_with_type_error, file);
    // Replaces "x" with 1 and therefore fixes the type error

    // foo_with_type_error has one type error
    assert!(!foo_with_type_error_diagnostics.is_empty());
    assert_eq!(
        foo_with_type_error_diagnostics[0].0.message,
        "Invalid value \"x\" for type int"
    );

    // Replace "x" with 1
    // "def foo(p: int = "x"): pass " -> "def foo(p: int = 1): pass "
    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 17,
            },
            end: lsp_types::Position {
                line: 0,
                character: 20,
            },
        }),
        range_length: Some(3),
        text: "1".into(),
    };

    foo_with_type_error.update(&file0_url, &[change]).unwrap();

    let foo_with_type_error_diagnostics = type_check_default_parameters::accumulated::<
        CheckErrorAccumulator,
    >(&foo_with_type_error, file);

    // foo_with_type_error should have no type errors
    assert_eq!(foo_with_type_error_diagnostics.len(), 0);
}
