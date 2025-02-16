use auto_lsp_core::build::Parse;

use super::super::python_workspace::*;
use crate::python::ast::{Attribute, ExpressionStatement, Call, Subscript};

#[test]
fn greek_letters() -> miette::Result<()> {
    ExpressionStatement::miette_parse(r#"ψ1 = β_γ + Ψ_5"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn subscript() -> miette::Result<()> {
    Subscript::miette_parse(r#"a[1]
b[2, 3]
c[4, 5,]"#, &PYTHON_PARSERS.get("python").unwrap())?;

    // slice
    Subscript::miette_parse(r#"a[:]
b[5:]
b[5:6, ...]
c[::]"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn attribute_references() -> miette::Result<()> {
    Attribute::miette_parse(r#"a.b.c"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn call_expressions() -> miette::Result<()> {
    Call::miette_parse(r#"__a__()"#, &PYTHON_PARSERS.get("python").unwrap())?;
    Call::miette_parse(r#"b(1)"#, &PYTHON_PARSERS.get("python").unwrap())?;
    Call::miette_parse(r#"c(e, f=g)"#, &PYTHON_PARSERS.get("python").unwrap())?;
    Call::miette_parse(r#"i(j, 5,)"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn print_as_identifier() -> miette::Result<()> {
    ExpressionStatement::miette_parse(r#"print()"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"print(a)"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"print(a, b=c)"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"print(d, e)"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"print(d, *e)"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"print(*f, **g,)"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"a(print)"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn print_as_parameter() -> miette::Result<()> {
    ExpressionStatement::miette_parse(r#"def a(print):
  b"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"def a(printer=print):
  c"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"def a(*print):
  b"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"def a(**print):
  b"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"def print():
  a"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn exec_as_identifier() -> miette::Result<()> {
    ExpressionStatement::miette_parse(
        r#"exec("print \"'%s' has %i characters\" % (public_function(), len(public_function()))", {"__builtins__" : None}, safe_dict)
"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"exec("""exec _code_ in _globs_, _locs_""")"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn async_await_as_identifier() -> miette::Result<()> {
    ExpressionStatement::miette_parse(r#"async = 5"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"await = 5"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"print async, await"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn calls_with_splat() -> miette::Result<()> {
    Call::miette_parse(r#"a(*())"#, &PYTHON_PARSERS.get("python").unwrap())?;
    Call::miette_parse(r#"a(**{})"#, &PYTHON_PARSERS.get("python").unwrap())?;
    Call::miette_parse(r#"a(*b)"#, &PYTHON_PARSERS.get("python").unwrap())?;
    Call::miette_parse(r#"c(d, *e, **g)"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn math_operators() -> miette::Result<()> {
    ExpressionStatement::miette_parse(r#"a + b * c ** d - e / 5"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"a // 2"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"-5"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"+x"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"~x"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn binary_addition_subtraction_with_floats() -> miette::Result<()> {
    ExpressionStatement::miette_parse(r#".1-.0"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#".1+.0"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#".1-0"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#".1+0"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"1-.0"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"1+.0"#, &PYTHON_PARSERS.get("python").unwrap())
}

