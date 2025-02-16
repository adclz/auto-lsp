use auto_lsp_core::build::Parse;

use super::super::python_workspace::*;
use crate::python::ast::{ImportStatement, ImportFromStatement, FutureImportStatement, PrintStatement, AssertStatement, IfStatement, ExpressionStatement, ReturnStatement, DeleteStatement, ForStatement, WhileStatement, TryStatement, WithStatement, Function, Class, DecoratedDefinition, RaiseStatement, GlobalStatement, ExecStatement, MatchStatement};

#[test]
fn import_statements() -> miette::Result<()> {
    ImportStatement::miette_parse(r#"import a, b"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ImportStatement::miette_parse(r#"import b.c as d"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ImportStatement::miette_parse(r#"import a.b.c"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn import_from_statements() -> miette::Result<()> {
    ImportFromStatement::miette_parse(r#"from a import b"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ImportFromStatement::miette_parse(r#"from a import *"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ImportFromStatement::miette_parse(r#"from a import (b, c)"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ImportFromStatement::miette_parse(r#"from a.b import c"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ImportFromStatement::miette_parse(r#"from . import b"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ImportFromStatement::miette_parse(r#"from .. import b"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ImportFromStatement::miette_parse(r#"from .a import b"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ImportFromStatement::miette_parse(r#"from ..a import b"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn import_future_statements() -> miette::Result<()> {
    FutureImportStatement::miette_parse(r#"from __future__ import print_statement"#, &PYTHON_PARSERS.get("python").unwrap())?;
    FutureImportStatement::miette_parse(r#"from __future__ import python4"#, &PYTHON_PARSERS.get("python").unwrap())?;
    FutureImportStatement::miette_parse(r#"from __future__ import (absolute_import, division, print_function, unicode_literals)"#, &PYTHON_PARSERS.get("python").unwrap())
}


#[test]
fn print_statements() -> miette::Result<()> {
    PrintStatement::miette_parse(r#"print a"#, &PYTHON_PARSERS.get("python").unwrap())?;
    PrintStatement::miette_parse(r#"print b, c"#, &PYTHON_PARSERS.get("python").unwrap())?;
    PrintStatement::miette_parse(r#"print 0 or 1, 1 or 0,"#, &PYTHON_PARSERS.get("python").unwrap())?;
    PrintStatement::miette_parse(r#"print 0 or 1"#, &PYTHON_PARSERS.get("python").unwrap())?;
    PrintStatement::miette_parse(r#"print not True"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn print_statements_with_redirection() -> miette::Result<()> {
    PrintStatement::miette_parse(r#"print >> a"#, &PYTHON_PARSERS.get("python").unwrap())?;
    PrintStatement::miette_parse(r#"print >> a, "b", "c""#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn assert_statements() -> miette::Result<()> {
    AssertStatement::miette_parse(r#"assert a"#, &PYTHON_PARSERS.get("python").unwrap())?;
    AssertStatement::miette_parse(r#"assert b, c"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn expression_statements() -> miette::Result<()> {
    ExpressionStatement::miette_parse(r#"a"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"b + c"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"1, 2, 3"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::miette_parse(r#"1, 2, 3,"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn delete_statements() -> miette::Result<()> {
    DeleteStatement::miette_parse(r#"del a[1], b[2]"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn control_flow_statements() -> miette::Result<()> {
    WhileStatement::miette_parse(r#"while true:
  pass
  break
  continue"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn return_statements() -> miette::Result<()> {
    ReturnStatement::miette_parse(r#"return a"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ReturnStatement::miette_parse(r#"return a + b, c"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ReturnStatement::miette_parse(r#"return not b"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn if_statements() -> miette::Result<()> {
    IfStatement::miette_parse(r#"if a:
  b
  c"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn if_else_statements() -> miette::Result<()> {
    IfStatement::miette_parse(r#"if a:
  b
elif c:
  d
else:
  f"#, &PYTHON_PARSERS.get("python").unwrap())?;
    IfStatement::miette_parse(r#"if a:
  b
else:
  f"#, &PYTHON_PARSERS.get("python").unwrap())?;
    IfStatement::miette_parse(r#"if a: b"#, &PYTHON_PARSERS.get("python").unwrap())?;
    IfStatement::miette_parse(r#"if a: b; c"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn nested_if_statements() -> miette::Result<()> {
    IfStatement::miette_parse(r#"if a:
  if b:
    c
  else:
    if e:
      f"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn while_statements() -> miette::Result<()> {
    WhileStatement::miette_parse(r#"while a:
  b
  c"#, &PYTHON_PARSERS.get("python").unwrap())?;

    WhileStatement::miette_parse(r#"while c:
  d
else:
  e
  f"#, &PYTHON_PARSERS.get("python").unwrap())

}


#[test]
fn for_statements() -> miette::Result<()> {
    ForStatement::miette_parse(r#"for line, i in lines:
  print line
  for character, j in line:
    print character
else:
  print x"#, &PYTHON_PARSERS.get("python").unwrap())?;

    ForStatement::miette_parse(r#"for x, in [(1,), (2,), (3,)]:
  x"#, &PYTHON_PARSERS.get("python").unwrap())
}


#[test]
fn try_statements() -> miette::Result<()> {
    TryStatement::miette_parse(r#"try:
  a
except b:
  c
except d as e:
  f
except g, h:
  i
except:
  j"#, &PYTHON_PARSERS.get("python").unwrap())?;

    TryStatement::miette_parse(r#"try:
  a
except b:
  c
  d
else:
  e
finally:
  f"#, &PYTHON_PARSERS.get("python").unwrap())?;

    TryStatement::miette_parse(r#"try:
  a
except* b:
  c
except* d as e:
  f
else:
  g
finally:
  h"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn with_statements() -> miette::Result<()> {
    WithStatement::miette_parse(r#"with a as b:
  c"#, &PYTHON_PARSERS.get("python").unwrap())?;

    WithStatement::miette_parse(r#"with (open('d') as d,
      open('e') as e):
  f"#, &PYTHON_PARSERS.get("python").unwrap())?;

    WithStatement::miette_parse(r#"with e as f, g as h,:
  i"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn async_functions() -> miette::Result<()> {
    Function::miette_parse(r#"async def a():
  b"#, &PYTHON_PARSERS.get("python").unwrap())?;

    Function::miette_parse(r#"async def c(d):
  e"#, &PYTHON_PARSERS.get("python").unwrap())?;

    Function::miette_parse(r#"async def g(g, h,):
  i"#, &PYTHON_PARSERS.get("python").unwrap())?;

    Function::miette_parse(r#"async def c(a: str):
  a"#, &PYTHON_PARSERS.get("python").unwrap())?;

    Function::miette_parse(r#"async def c(a: b.c):
  a"#, &PYTHON_PARSERS.get("python").unwrap())?;

    Function::miette_parse(r#"async def d(a: Sequence[T]) -> T:
  a"#, &PYTHON_PARSERS.get("python").unwrap())?;

    Function::miette_parse(r#"async def i(a, b=c, *c, **d):
  a"#, &PYTHON_PARSERS.get("python").unwrap())?;

    Function::miette_parse(r#"async def d(a: str) -> None:
  return None"#, &PYTHON_PARSERS.get("python").unwrap())?;

    Function::miette_parse(r#"async def d(a:str="default", b=c) -> None:
  return None"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn function_definitions() -> miette::Result<()> {
    Function::miette_parse(r#"def e((a,b)):
  return (a,b)"#, &PYTHON_PARSERS.get("python").unwrap())?;

    Function::miette_parse(r#"def e(*list: str):
  pass"#, &PYTHON_PARSERS.get("python").unwrap())?;

    Function::miette_parse(r#"def e(**list: str):
  pass"#, &PYTHON_PARSERS.get("python").unwrap())?;

    Function::miette_parse(r#"def f():
  nonlocal a"#, &PYTHON_PARSERS.get("python").unwrap())?;

    Function::miette_parse(r#"def g(h, i, /, j, *, k=100, **kwarg):
  return h,i,j,k,kwarg"#, &PYTHON_PARSERS.get("python").unwrap())?;

    Function::miette_parse(r#"def h(*a):
  i((*a))
  j(((*a)))"#, &PYTHON_PARSERS.get("python").unwrap())?;

    Function::miette_parse(r#"def foo():
    pass \
\
\"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn class_definitions() -> miette::Result<()> {
    Class::miette_parse(r#"class A:
  def b(self):
    return c"#, &PYTHON_PARSERS.get("python").unwrap())?;

    Class::miette_parse(r#"class B():
  pass"#, &PYTHON_PARSERS.get("python").unwrap())?;

    Class::miette_parse(r#"class B(method1):
  def method1(self):
    return"#, &PYTHON_PARSERS.get("python").unwrap())?;

    Class::miette_parse(r#"class C(method1, Sequence[T]):
  pass"#, &PYTHON_PARSERS.get("python").unwrap())?;

    Class::miette_parse(r#"class D(Sequence[T, U]):
  pass"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn class_definitions_with_superclasses() -> miette::Result<()> {
    Class::miette_parse(r#"class A(B, C):
  def d():
    e"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn decorated_definitions() -> miette::Result<()> {
    DecoratedDefinition::miette_parse(r#"@a.b
class C:
  @d(1)
  @e[2].f.g
  def f():
    g"#, &PYTHON_PARSERS.get("python").unwrap())?;

    DecoratedDefinition::miette_parse(r#" @f()
  async def f():
    g"#, &PYTHON_PARSERS.get("python").unwrap())?;

    DecoratedDefinition::miette_parse(r#"@buttons[0].clicked.connect
def spam():
    ..."#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn raise_statements() -> miette::Result<()> {
    RaiseStatement::miette_parse(r#"raise"#, &PYTHON_PARSERS.get("python").unwrap())?;
    RaiseStatement::miette_parse(r#"raise RuntimeError('NO')"#, &PYTHON_PARSERS.get("python").unwrap())?;
    RaiseStatement::miette_parse(r#"raise RunTimeError('NO') from e"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn global_statements() -> miette::Result<()> {
    GlobalStatement::miette_parse(r#"global a"#, &PYTHON_PARSERS.get("python").unwrap())?;
    GlobalStatement::miette_parse(r#"global a, b"#, &PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn exec_statements() -> miette::Result<()> {
    ExecStatement::miette_parse(r#"exec '1+1'"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExecStatement::miette_parse(r#"exec 'x+=1' in None"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExecStatement::miette_parse(r#"exec 'x+=1' in a, b"#, &PYTHON_PARSERS.get("python").unwrap())?;
    ExecStatement::miette_parse(r#"exec func in {}"#, &PYTHON_PARSERS.get("python").unwrap())
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

