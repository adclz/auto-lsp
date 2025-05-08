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
fn import_statements() -> Result {
    snap!(
        r#"
import a, b
import b.c as d
import a.b.c
"#
    )
}

#[test]
fn import_from_statements() -> Result {
    snap!(
        r#"
from a import b
from a import *
from a import (b, c)
from a.b import c
from . import b
from .. import b
from .a import b
from ..a import b
"#
    )
}

#[test]
fn future_import_statements() -> Result {
    snap!(
        r#"
from __future__ import print_statement
from __future__ import python4
from __future__ import (absolute_import, division, print_function,
                        unicode_literals)
"#
    )
}

#[test]
fn print_statements() -> Result {
    snap!(
        r#"
print a
print b, c
print 0 or 1, 1 or 0,
print 0 or 1
print not True
"#
    )
}

#[test]
fn print_statements_with_redirection() -> Result {
    snap!(
        r#"
print >> a
print >> a, "b", "c"
"#
    )
}

#[test]
fn assert_statements() -> Result {
    snap!(
        r#"
assert a
assert b, c
"#
    )
}

#[test]
fn expression_statements() -> Result {
    snap!(
        r#"
a
b + c
1, 2, 3
1, 2, 3,
"#
    )
}

#[test]
fn delete_statements() -> Result {
    snap!(
        r#"
del a[1], b[2]
"#
    )
}

#[test]
fn control_flow_statements() -> Result {
    snap!(
        r#"
while true:
  pass
  break
  continue
"#
    )
}

#[test]
fn return_statements() -> Result {
    snap!(
        r#"
return
return a + b, c
return not b
"#
    )
}

#[test]
fn if_statements() -> Result {
    snap!(
        r#"
if a:
  b
  c
"#
    )
}

#[test]
fn if_else_statements() -> Result {
    snap!(
        r#"
if a:
  b
elif c:
  d
else:
  f

if a:
  b
else:
  f

if a: b

if a: b; c
"#
    )
}

#[test]
fn nested_if_statements() -> Result {
    snap!(
        r#"
if a:
  if b:
    c
  else:
    if e:
      f
g
"#
    )
}

#[test]
fn while_statements() -> Result {
    snap!(
        r#"
while a:
  b

while c:
  d
else:
  e
  f
"#
    )
}

#[test]
fn for_statements() -> Result {
    snap!(
        r#"
for line, i in lines:
  print line
  for character, j in line:
    print character
else:
  print x

for x, in [(1,), (2,), (3,)]:
  x
"#
    )
}

#[test]
fn try_statements() -> Result {
    snap!(
        r#"
try:
  a
except b:
  c
except d as e:
  f
except g, h:
  i
except:
  j

try:
  a
except b:
  c
  d
else:
  e
finally:
  f

try:
  a
except* b:
  c
except* d as e:
  f
else:
  g
finally:
  h
"#
    )
}

#[test]
fn with_statements() -> Result {
    snap!(
        r#"
with a as b:
  c

with (open('d') as d,
      open('e') as e):
  f

with e as f, g as h,:
  i
"#
    )
}

#[test]
fn async_function_definitions() -> Result {
    snap!(
        r#"
async def a():
  b

async def c(d):
  e

async def g(g, h,):
  i

async def c(a: str):
  a

async def c(a: b.c):
  a

async def d(a: Sequence[T]) -> T:
  a

async def i(a, b=c, *c, **d):
  a

async def d(a: str) -> None:
  return None

async def d(a:str="default", b=c) -> None:
  return None
"#
    )
}

#[test]
fn function_definitions() -> Result {
    snap!(
        r#"
def e((a,b)):
  return (a,b)

def e(*list: str):
  pass

def e(**list: str):
  pass

def f():
  nonlocal a

def g(h, i, /, j, *, k=100, **kwarg):
  return h,i,j,k,kwarg

def h(*a):
  i((*a))
  j(((*a)))

def foo():
    pass \
\
\

"#
    )
}

#[test]
fn empty_blocks() -> Result {
    snap!(
        r#"
# These are not actually valid python; blocks
# must contain at least one statement. But we
# allow them because error recovery for empty
# blocks doesn't work very well otherwise.
def a(b, c):

if d:
  print e
  while f():
"#
    )
}

#[test]
fn class_definitions() -> Result {
    snap!(
        r#"
class A:
  def b(self):
    return c
class B():
  pass
class B(method1):
  def method1(self):
    return
class C(method1, Sequence[T]):
  pass
class D(Sequence[T, U]):
  pass
"#
    )
}
