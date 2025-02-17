use auto_lsp_core::build::Parse;

use super::super::python_workspace::*;
use crate::python::ast::{Attribute, ExpressionStatement, Call, Subscript, Function, WithStatement};

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

#[test]
fn bitwise_operator() -> miette::Result<()> {
    ExpressionStatement::miette_parse(r#"a << b | c >> d & e ^ f"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn boolean_operators() -> miette::Result<()> {
    ExpressionStatement::miette_parse(r#"a or b and c"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"not d"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"not a and b or c"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"a and not b and c"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn comparison_operators() -> miette::Result<()> {
    ExpressionStatement::miette_parse(r#"a < b <= c == d >= e > f"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"not a == b or c == d"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"a not in b"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"a is not b"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"a is b and c != d"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"a <> b"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn assignments() -> miette::Result<()> {
    ExpressionStatement::miette_parse(r#"a = 1"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"a, b = 1, 2"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"a, *c = 1, 2, 3"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"a[b] = c = d"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"a, *b.c = d"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn assignments_with_type_annotation() -> miette::Result<()> {
    ExpressionStatement::miette_parse(r#"tail_leaves: List[Leaf] = []"#, &PYTHON_PARSERS.get("python").unwrap())
}


#[test]
fn augmented_assignments() -> miette::Result<()> {
    ExpressionStatement::miette_parse(r#"a += 1"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"b >>= 2"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"c //= 1"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"d @= e"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"f -= 2"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"g %= 2"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"h /= 5"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"i *= 2.2"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"j &= 1"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"k ^= 0"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"l <<= 2"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"m |= k"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"n **= 3"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn named_expressions() -> miette::Result<()> {
    ExpressionStatement::miette_parse(r#"a := x"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"(y := f(x))"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"foo(x=(y := f(x)))"#, &PYTHON_PARSERS.get("python").unwrap())?;
    Function::miette_parse(r#"def foo(answer=(p := 42)):
  return answer;"#, &PYTHON_PARSERS.get("python").unwrap())?;
    Function::miette_parse(r#"def foo(answer: (p := 42) = 5):
  return answer;"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"foo(x := 3, cat='vector')"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"(z := (y := (x := 0)))"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn yield_expressions() -> miette::Result<()> {
    ExpressionStatement::miette_parse(r#"yield"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"yield 1"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"x = yield 2"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"yield from a"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#" yield from (yield from (x for x in range(1, 10)))"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn lambdas() -> miette::Result<()> {
    ExpressionStatement::miette_parse(r#"lambda b, c: d("e" % f)"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"lambda: True"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"lambda a, b = c, *d, **e: a"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"lambda (a, b): (a, b)"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn tuples_with_splat() -> miette::Result<()> {
    ExpressionStatement::miette_parse(r#"(foo, *bar, *baz)"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn tuples_with_yield() -> miette::Result<()> {
    ExpressionStatement::miette_parse(r#"(a, yield a, b, c)"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn default_tuple_arguments() -> miette::Result<()> {
    Function::miette_parse(r#"def comp_args((a, b)=(3, 4)):
    return a, b"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn conditional_if_expressions() -> miette::Result<()> {
    ExpressionStatement::miette_parse(r#"a = b if c else d"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"something() if a else d"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"slice(1,1,1) if a else d"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn async_context_managers_and_iterators() -> miette::Result<()> {
    WithStatement::miette_parse(r#"async with a as b:
  async for c in d:
     [e async for f in g]"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn splat_inside_of_expression_list() -> miette::Result<()> {
    ExpressionStatement::miette_parse(r#"a,c = [1,2],3"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"w, x, y, z = 0, *a, c"#, &PYTHON_PARSERS.get("python").unwrap())
}


#[test]
fn type_expressions() -> miette::Result<()> {
    ExpressionStatement::miette_parse(r#"a: A[T] | B"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"y: type[int] = int"#, &PYTHON_PARSERS.get("python").unwrap())
}
