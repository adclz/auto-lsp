use auto_lsp_core::build::Parse;

use super::python_workspace::*;
use crate::python::ast::{
    AssertStatement, DeleteStatement, ForStatement, Function, FutureImportStatement, IfStatement,
    ImportFromStatement, MatchStatement, NamedExpression, PrintStatement, RaiseStatement,
    ReturnStatement, TryStatement, WhileStatement,
};

/// Imports

#[test]
fn future_import_statement() -> miette::Result<()> {
    FutureImportStatement::miette_parse(
        r#"from __future__ import some_class"#,
        &PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn import_from_statement() -> miette::Result<()> {
    ImportFromStatement::miette_parse(
        r#"from module import some_class"#,
        &PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn relative_import_from_statement() -> miette::Result<()> {
    ImportFromStatement::miette_parse(
        r#"from ...module import some_class"#,
        &PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn aliased_import_from_statement() -> miette::Result<()> {
    ImportFromStatement::miette_parse(
        r#"from ...module import some_class as another_class"#,
        &PYTHON_PARSERS.get("python").unwrap(),
    )
}

// Statements

#[test]
fn print_statement() -> miette::Result<()> {
    PrintStatement::miette_parse(
        r#"print >> "Hello", "World""#,
        &PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn simple_try_statement() -> miette::Result<()> {
    TryStatement::miette_parse(
        r#"
        try :
            pass
        except:
            pass
        finally:
            pass
       "#,
        &PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn assert_statement() -> miette::Result<()> {
    AssertStatement::miette_parse(
        r#"assert 2 + 2 == 5, "Houston we've got a problem""#,
        &PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn return_statement() -> miette::Result<()> {
    ReturnStatement::miette_parse(
        r#"return something"#,
        &PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn delete_statement() -> miette::Result<()> {
    DeleteStatement::miette_parse(r#"del something"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn raise_statement() -> miette::Result<()> {
    RaiseStatement::miette_parse(r#"raise something"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn if_statement() -> miette::Result<()> {
    IfStatement::miette_parse(
        r#"
if b > a:
  print("b is greater than a")
elif a == b:
  print("a and b are equal") 
else:
  print("a is greater than b")"#,
        &PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn for_statement() -> miette::Result<()> {
    ForStatement::miette_parse(
        r#"
for x in ["apple", "banana", "cherry"]:
  if x == "banana":
    break
  print(x)"#,
        &PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn while_statement() -> miette::Result<()> {
    WhileStatement::miette_parse(
        r#"
while i < 10:
    i = i + 10"#,
        &PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn match_statement() -> miette::Result<()> {
    MatchStatement::miette_parse(
        r#"
match x:
    case 10:
      print("It's 10")
    case 20:
      print("It's 20")
    case _:
      print("It's neither 10 nor 20")"#,
        &PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn named_expression() -> miette::Result<()> {
    NamedExpression::miette_parse(
        r#"var := something"#,
        &PYTHON_PARSERS.get("python").unwrap(),
    )
}

/// Function

#[test]
fn function() -> miette::Result<()> {
    Function::miette_parse(
        r#"
       def foo():
           pass
       "#,
        &PYTHON_PARSERS.get("python").unwrap(),
    )
}
