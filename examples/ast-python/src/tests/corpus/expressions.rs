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

use super::utils::Result;
use crate::snap;
use rstest::rstest;

#[rstest]
#[case("ψ1 = β_γ + Ψ_5")]
fn greek_letters(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case(
    r#"a[1]
b[2, 3]
c[4, 5,]"#
)]
#[case(
    r#"a[:]
b[5:]
b[5:6, ...]
c[::]"#
)]
fn subscript(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("a.b.c")]
fn attribute_references(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("__a__()")]
#[case("b(1)")]
#[case("c(e, f=g)")]
#[case("i(j, 5,)")]
fn call_expressions(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("print()")]
#[case("print(a)")]
#[case("print(a, b=c)")]
#[case("print(d, e)")]
#[case("print(d, *e)")]
#[case("print(*f, **g,)")]
#[case("a(print)")]
fn print_as_identifier(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case(
    r#"def a(print):
  b"#
)]
#[case(
    r#"def a(printer=print):
  c"#
)]
#[case(
    r#"def a(*print):
  b"#
)]
#[case(
    r#"def a(**print):
  b"#
)]
#[case(
    r#"def print():
  a"#
)]
fn print_as_parameter(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case(r#"exec("print \"'%s' has %i characters\" % (public_function(), len(public_function()))", {"__builtins__" : None}, safe_dict)"#)]
#[case(r#"exec("""exec _code_ in _globs_, _locs_""")"#)]
fn exec_as_identifier(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("async = 5")]
#[case("await = 5")]
#[case("print async, await")]
fn async_await_as_identifier(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("a(*())")]
#[case("a(**{})")]
#[case("a(*b)")]
#[case("c(d, *e, **g)")]
fn calls_with_splat(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("a + b * c ** d - e / 5")]
#[case("a // 2")]
#[case("-5")]
#[case("+x")]
#[case("~x")]
fn math_operators(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case(".1-.0")]
#[case(".1+.0")]
#[case(".1-0")]
#[case(".1+0")]
#[case("1-.0")]
#[case("1+.0")]
fn binary_addition_subtraction_with_floats(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("a << b | c >> d & e ^ f")]
fn bitwise_operator(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("a or b and c")]
#[case("not d")]
#[case("not a and b or c")]
#[case("a and not b and c")]
fn boolean_operators(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("a < b <= c == d >= e > f")]
#[case("not a == b or c == d")]
#[case("a not in b")]
#[case("a is not b")]
#[case("a is b and c != d")]
#[case("a <> b")]
fn comparison_operators(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("a = 1")]
#[case("a, b = 1, 2")]
#[case("a, *c = 1, 2, 3")]
#[case("a[b] = c = d")]
#[case("a, *b.c = d")]
fn assignments(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("tail_leaves: List[Leaf] = []")]
fn assignments_with_type_annotation(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("a += 1")]
#[case("b >>= 2")]
#[case("c //= 1")]
#[case("d @= e")]
#[case("f -= 2")]
#[case("g %= 2")]
#[case("h /= 5")]
#[case("i *= 2.2")]
#[case("j &= 1")]
#[case("k ^= 0")]
#[case("l <<= 2")]
#[case("m |= k")]
#[case("n **= 3")]
fn augmented_assignments(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("a := x")]
#[case("(y := f(x))")]
#[case("foo(x=(y := f(x)))")]
#[case(
    r#"def foo(answer=(p := 42)):
  return answer;"#
)]
#[case(
    r#"def foo(answer: (p := 42) = 5):
  return answer;"#
)]
#[case("foo(x := 3, cat='vector')")]
#[case("(z := (y := (x := 0)))")]
fn named_expressions(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("yield")]
#[case("yield 1")]
#[case("x = yield 2")]
#[case("yield from a")]
#[case(" yield from (yield from (x for x in range(1, 10)))")]
fn yield_expressions(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("lambda b, c: d(\"e\" % f)")]
#[case("lambda: True")]
#[case("lambda a, b = c, *d, **e: a")]
#[case("lambda (a, b): (a, b)")]
fn lambdas(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("(foo, *bar, *baz)")]
fn tuples_with_splat(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("(a, yield a, b, c)")]
fn tuples_with_yield(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case(
    r#"def comp_args((a, b)=(3, 4)):
    return a, b"#
)]
fn default_tuple_arguments(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("a = b if c else d")]
#[case("something() if a else d")]
#[case("slice(1,1,1) if a else d")]
fn conditional_if_expressions(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case(
    r#"async with a as b:
  async for c in d:
     [e async for f in g]"#
)]
fn async_context_managers_and_iterators(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("a,c = [1,2],3")]
#[case("w, x, y, z = 0, *a, c")]
fn splat_inside_of_expression_list(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("a: A[T] | B")]
#[case("y: type[int] = int")]
fn type_expressions(#[case] input: &str) -> Result {
    snap!(input)
}
