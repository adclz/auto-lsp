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
#[case("import a, b")]
#[case("import b.c as d")]
#[case("import a.b.c")]
fn import_statements(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("from a import b")]
#[case("from a import *")]
#[case("from a import (b, c)")]
#[case("from a.b import c")]
#[case("from . import b")]
#[case("from .. import b")]
#[case("from .a import b")]
#[case("from ..a import b")]
fn import_from_statements(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("from __future__ import print_statement")]
#[case("from __future__ import python4")]
#[case("from __future__ import (absolute_import, division, print_function, unicode_literals)")]
fn import_future_statements(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("print a")]
#[case("print b, c")]
#[case("print 0 or 1, 1 or 0,")]
#[case("print 0 or 1")]
#[case("print not True")]
fn print_statements(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("print >> a")]
#[case(r#"print >> a, "b", "c""#)]
fn print_statements_with_redirection(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("assert a")]
#[case("assert b, c")]
fn assert_statements(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("a")]
#[case("b + c")]
#[case("1, 2, 3")]
#[case("1, 2, 3,")]
fn expression_statements(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("del a[1], b[2]")]
fn delete_statements(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("while true:\n  pass\n  break\n  continue")]
fn control_flow_statements(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("return a")]
#[case("return a + b, c")]
#[case("return not b")]
fn return_statements(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("if a:\n  b\n  c")]
fn if_statements(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("if a:\n  b\nelif c:\n  d\nelse:\n  f")]
#[case("if a:\n  b\nelse:\n  f")]
#[case("if a: b")]
#[case("if a: b; c")]
fn if_else_statements(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("if a:\n  if b:\n    c\n  else:\n    if e:\n      f")]
fn nested_if_statements(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("while a:\n  b\n  c")]
#[case("while c:\n  d\nelse:\n  e\n  f")]
fn while_statements(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("for line, i in lines:\n  print line\n  for character, j in line:\n    print character\nelse:\n  print x")]
#[case("for x, in [(1,), (2,), (3,)]:\n  x")]
fn for_statements(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("try:\n  a\nexcept b:\n  c\nexcept d as e:\n  f\nexcept g, h:\n  i\nexcept:\n  j")]
#[case("try:\n  a\nexcept b:\n  c\n  d\nelse:\n  e\nfinally:\n  f")]
#[case("try:\n  a\nexcept* b:\n  c\nexcept* d as e:\n  f\nelse:\n  g\nfinally:\n  h")]
fn try_statements(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("with a as b:\n  c")]
#[case("with (open('d') as d,\n      open('e') as e):\n  f")]
#[case("with e as f, g as h,:\n  i")]
fn with_statements(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("async def a():\n  b")]
#[case("async def c(d):\n  e")]
#[case("async def g(g, h,):\n  i")]
#[case("async def c(a: str):\n  a")]
#[case("async def c(a: b.c):\n  a")]
#[case("async def d(a: Sequence[T]) -> T:\n  a")]
#[case("async def i(a, b=c, *c, **d):\n  a")]
#[case("async def d(a: str) -> None:\n  return None")]
#[case("async def d(a:str=\"default\", b=c) -> None:\n  return None")]
fn async_functions(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("def e((a,b)):\n  return (a,b)")]
#[case("def e(*list: str):\n  pass")]
#[case("def e(**list: str):\n  pass")]
#[case("def f():\n  nonlocal a")]
#[case("def g(h, i, /, j, *, k=100, **kwarg):\n  return h,i,j,k,kwarg")]
#[case("def h(*a):\n  i((*a))\n  j(((*a)))")]
#[case("def foo():\n    pass \\\n\\\n\\")]
fn function_definitions(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("class A:\n  def b(self):\n    return c")]
#[case("class B():\n  pass")]
#[case("class B(method1):\n  def method1(self):\n    return")]
#[case("class C(method1, Sequence[T]):\n  pass")]
#[case("class D(Sequence[T, U]):\n  pass")]
fn class_definitions(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("class A(B, C):\n  def d():\n    e")]
fn class_definitions_with_superclasses(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("@a.b\nclass C:\n  @d(1)\n  @e[2].f.g\n  def f():\n    g")]
#[case(" @f()\n  async def f():\n    g")]
#[case("@buttons[0].clicked.connect\ndef spam():\n    ...")]
fn decorated_definitions(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("raise")]
#[case("raise RuntimeError('NO')")]
#[case("raise RunTimeError('NO') from e")]
fn raise_statements(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("global a")]
#[case("global a, b")]
fn global_statements(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("exec '1+1'")]
#[case("exec 'x+=1' in None")]
#[case("exec 'x+=1' in a, b")]
#[case("exec func in {}")]
fn exec_statements(#[case] input: &str) -> Result {
    snap!(input)
}
