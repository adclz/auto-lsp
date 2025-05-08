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

#[test]
fn identifiers_with_greek_letters() -> Result {
    snap!(
        r#"
ψ1 = β_γ + Ψ_5
"#
    )
}

#[test]
fn subscript_expressions() -> Result {
    snap!(
        r#"
a[1]
b[2, 3]
c[4, 5,]
"#
    )
}

#[test]
fn subscript_slice_expressions() -> Result {
    snap!(
        r#"
a[:]
b[5:]
b[5:6, ...]
c[::]
"#
    )
}

#[test]
fn attribute_references() -> Result {
    snap!(
        r#"
a.b.c
"#
    )
}

#[test]
fn await_expressions() -> Result {
    snap!(
        r#"
await i(j, 5)
return await i(j, 5)
async def region_exists(region: str) -> bool:
    return region in await all_regions()

assert await a(b) == c
"#
    )
}

#[test]
fn call_expressions() -> Result {
    snap!(
        r#"
__a__()
b(1)
c(e, f=g)
i(j, 5,)
"#
    )
}

#[test]
fn print_used_as_an_identifier() -> Result {
    snap!(
        r#"
print()
print(a)
print(a, b=c)
print(d, e)
print(d, *e)
print(*f, **g,)
a(print)
"#
    )
}

#[test]
fn print_used_as_a_parameter() -> Result {
    snap!(
        r#"
def a(print):
  b
def a(printer=print):
  c
def a(*print):
  b
def a(**print):
  b
def print():
  a
"#
    )
}

#[test]
fn exec_used_as_an_identifier() -> Result {
    snap!(
        r#"
exec("print \"'%s' has %i characters\" % (public_function(), len(public_function()))", {"__builtins__" : None}, safe_dict)
exec("""exec _code_ in _globs_, _locs_""")
"#
    )
}

#[test]
fn async_await_used_as_identifiers() -> Result {
    snap!(
        r#"
async = 4
await = 5
print async, await
"#
    )
}

#[test]
fn calls_with_splats() -> Result {
    snap!(
        r#"
a(*())
a(**{})
a(*b)
c(d, *e, **g)
"#
    )
}

#[test]
fn math_operators() -> Result {
    snap!(
        r#"
a + b * c ** d - e / 5
a // 2
-5
+x
~x
"#
    )
}

#[test]
fn binary_addition_subtraction_with_floats() -> Result {
    snap!(
        r#"
.1-.0
.1+.0
.1-0
.1+0

1-.0
1+.0
"#
    )
}

#[test]
fn power_operator_precedence() -> Result {
    snap!(
        r#"
2**2**3
-2**2
"#
    )
}

#[test]
fn operator_precedence() -> Result {
    snap!(
        r#"
a() + b[c] * c.d.e
"#
    )
}

#[test]
fn bitwise_operators() -> Result {
    snap!(
        r#"
a << b | c >> d & e ^ f
"#
    )
}

#[test]
fn boolean_operators() -> Result {
    snap!(
        r#"
a or b and c
not d
not a and b or c
a and not b and c
"#
    )
}

#[test]
fn comparison_operators() -> Result {
    snap!(
        r#"
a < b <= c == d >= e > f
not a == b or c == d
a not in b
a is not b
a is b and c != d
a <> b
"#
    )
}

#[test]
fn assignments() -> Result {
    snap!(
        r#"
a = 1
a, b = 1, 2
a, *c = 1, 2, 3
a, = 1, 2
a[b] = c = d
a, *b.c = d
"#
    )
}

#[test]
fn assignments_with_type_annotations() -> Result {
    snap!(
        r#"
tail_leaves: List[Leaf] = []
"#
    )
}

#[test]
fn augmented_assignments() -> Result {
    snap!(
        r#"
a += 1
b >>= 2
c //= 1
d @= e
f -= 2
g %= 2
h /= 5
i *= 2.2
j &= 1
k ^= 0
l <<= 2
m |= k
n **= 3
"#
    )
}

#[test]
fn named_expressions() -> Result {
    snap!(
        r#"
a := x
(y := f(x))
foo(x=(y := f(x)))
y0 = (y1 := f(x))
def foo(answer=(p := 42)):
  return answer;
def foo(answer: (p := 42) = 5):
  return answer;
foo(x := 3, cat='vector')
(z := (y := (x := 0)))
"#
    )
}

#[test]
fn yield_expressions() -> Result {
    snap!(
        r#"
def example():
  yield
  yield 1
  x = yield 2
  yield from a
  yield from (yield from (x for x in range(1, 10)))
"#
    )
}

#[test]
fn lambdas() -> Result {
    snap!(
        r#"
lambda b, c: d("e" % f)
lambda: True
lambda a, b = c, *d, **e: a
lambda (a, b): (a, b)
"#
    )
}

#[test]
fn tuples_with_splats() -> Result {
    snap!(
        r#"
(foo, *bar, *baz)
"#
    )
}
