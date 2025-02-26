use auto_lsp_core::build::{TryParse, TestParseResult};

use super::super::python_workspace::*;
use crate::python::ast::{
    AssertStatement, Class, DecoratedDefinition, DeleteStatement, ExecStatement,
    ExpressionStatement, ForStatement, Function, FutureImportStatement, GlobalStatement,
    IfStatement, ImportFromStatement, ImportStatement, PrintStatement, RaiseStatement,
    ReturnStatement, TryStatement, WhileStatement, WithStatement,
};

#[test]
fn import_statements() -> TestParseResult {
    ImportStatement::test_parse(r#"import a, b"#, PYTHON_PARSERS.get("python").unwrap())?;
    ImportStatement::test_parse(r#"import b.c as d"#, PYTHON_PARSERS.get("python").unwrap())?;
    ImportStatement::test_parse(r#"import a.b.c"#, PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn import_from_statements() -> TestParseResult {
    ImportFromStatement::test_parse(r#"from a import b"#, PYTHON_PARSERS.get("python").unwrap())?;
    ImportFromStatement::test_parse(r#"from a import *"#, PYTHON_PARSERS.get("python").unwrap())?;
    ImportFromStatement::test_parse(
        r#"from a import (b, c)"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;
    ImportFromStatement::test_parse(
        r#"from a.b import c"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;
    ImportFromStatement::test_parse(r#"from . import b"#, PYTHON_PARSERS.get("python").unwrap())?;
    ImportFromStatement::test_parse(r#"from .. import b"#, PYTHON_PARSERS.get("python").unwrap())?;
    ImportFromStatement::test_parse(r#"from .a import b"#, PYTHON_PARSERS.get("python").unwrap())?;
    ImportFromStatement::test_parse(
        r#"from ..a import b"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn import_future_statements() -> TestParseResult {
    FutureImportStatement::test_parse(
        r#"from __future__ import print_statement"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;
    FutureImportStatement::test_parse(
        r#"from __future__ import python4"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;
    FutureImportStatement::test_parse(
        r#"from __future__ import (absolute_import, division, print_function, unicode_literals)"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn print_statements() -> TestParseResult {
    PrintStatement::test_parse(r#"print a"#, PYTHON_PARSERS.get("python").unwrap())?;
    PrintStatement::test_parse(r#"print b, c"#, PYTHON_PARSERS.get("python").unwrap())?;
    PrintStatement::test_parse(
        r#"print 0 or 1, 1 or 0,"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;
    PrintStatement::test_parse(r#"print 0 or 1"#, PYTHON_PARSERS.get("python").unwrap())?;
    PrintStatement::test_parse(r#"print not True"#, PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn print_statements_with_redirection() -> TestParseResult {
    PrintStatement::test_parse(r#"print >> a"#, PYTHON_PARSERS.get("python").unwrap())?;
    PrintStatement::test_parse(
        r#"print >> a, "b", "c""#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn assert_statements() -> TestParseResult {
    AssertStatement::test_parse(r#"assert a"#, PYTHON_PARSERS.get("python").unwrap())?;
    AssertStatement::test_parse(r#"assert b, c"#, PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn expression_statements() -> TestParseResult {
    ExpressionStatement::test_parse(r#"a"#, PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::test_parse(r#"b + c"#, PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::test_parse(r#"1, 2, 3"#, PYTHON_PARSERS.get("python").unwrap())?;
    ExpressionStatement::test_parse(r#"1, 2, 3,"#, PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn delete_statements() -> TestParseResult {
    DeleteStatement::test_parse(r#"del a[1], b[2]"#, PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn control_flow_statements() -> TestParseResult {
    WhileStatement::test_parse(
        r#"while true:
  pass
  break
  continue"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn return_statements() -> TestParseResult {
    ReturnStatement::test_parse(r#"return a"#, PYTHON_PARSERS.get("python").unwrap())?;
    ReturnStatement::test_parse(r#"return a + b, c"#, PYTHON_PARSERS.get("python").unwrap())?;
    ReturnStatement::test_parse(r#"return not b"#, PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn if_statements() -> TestParseResult {
    IfStatement::test_parse(
        r#"if a:
  b
  c"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn if_else_statements() -> TestParseResult {
    IfStatement::test_parse(
        r#"if a:
  b
elif c:
  d
else:
  f"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;
    IfStatement::test_parse(
        r#"if a:
  b
else:
  f"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;
    IfStatement::test_parse(r#"if a: b"#, PYTHON_PARSERS.get("python").unwrap())?;
    IfStatement::test_parse(r#"if a: b; c"#, PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn nested_if_statements() -> TestParseResult {
    IfStatement::test_parse(
        r#"if a:
  if b:
    c
  else:
    if e:
      f"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn while_statements() -> TestParseResult {
    WhileStatement::test_parse(
        r#"while a:
  b
  c"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    WhileStatement::test_parse(
        r#"while c:
  d
else:
  e
  f"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn for_statements() -> TestParseResult {
    ForStatement::test_parse(
        r#"for line, i in lines:
  print line
  for character, j in line:
    print character
else:
  print x"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    ForStatement::test_parse(
        r#"for x, in [(1,), (2,), (3,)]:
  x"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn try_statements() -> TestParseResult {
    TryStatement::test_parse(
        r#"try:
  a
except b:
  c
except d as e:
  f
except g, h:
  i
except:
  j"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    TryStatement::test_parse(
        r#"try:
  a
except b:
  c
  d
else:
  e
finally:
  f"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    TryStatement::test_parse(
        r#"try:
  a
except* b:
  c
except* d as e:
  f
else:
  g
finally:
  h"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn with_statements() -> TestParseResult {
    WithStatement::test_parse(
        r#"with a as b:
  c"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    WithStatement::test_parse(
        r#"with (open('d') as d,
      open('e') as e):
  f"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    WithStatement::test_parse(
        r#"with e as f, g as h,:
  i"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn async_functions() -> TestParseResult {
    Function::test_parse(
        r#"async def a():
  b"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    Function::test_parse(
        r#"async def c(d):
  e"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    Function::test_parse(
        r#"async def g(g, h,):
  i"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    Function::test_parse(
        r#"async def c(a: str):
  a"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    Function::test_parse(
        r#"async def c(a: b.c):
  a"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    Function::test_parse(
        r#"async def d(a: Sequence[T]) -> T:
  a"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    Function::test_parse(
        r#"async def i(a, b=c, *c, **d):
  a"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    Function::test_parse(
        r#"async def d(a: str) -> None:
  return None"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    Function::test_parse(
        r#"async def d(a:str="default", b=c) -> None:
  return None"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn function_definitions() -> TestParseResult {
    Function::test_parse(
        r#"def e((a,b)):
  return (a,b)"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    Function::test_parse(
        r#"def e(*list: str):
  pass"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    Function::test_parse(
        r#"def e(**list: str):
  pass"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    Function::test_parse(
        r#"def f():
  nonlocal a"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    Function::test_parse(
        r#"def g(h, i, /, j, *, k=100, **kwarg):
  return h,i,j,k,kwarg"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    Function::test_parse(
        r#"def h(*a):
  i((*a))
  j(((*a)))"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    Function::test_parse(
        r#"def foo():
    pass \
\
\"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn class_definitions() -> TestParseResult {
    Class::test_parse(
        r#"class A:
  def b(self):
    return c"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    Class::test_parse(
        r#"class B():
  pass"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    Class::test_parse(
        r#"class B(method1):
  def method1(self):
    return"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    Class::test_parse(
        r#"class C(method1, Sequence[T]):
  pass"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    Class::test_parse(
        r#"class D(Sequence[T, U]):
  pass"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn class_definitions_with_superclasses() -> TestParseResult {
    Class::test_parse(
        r#"class A(B, C):
  def d():
    e"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn decorated_definitions() -> TestParseResult {
    DecoratedDefinition::test_parse(
        r#"@a.b
class C:
  @d(1)
  @e[2].f.g
  def f():
    g"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    DecoratedDefinition::test_parse(
        r#" @f()
  async def f():
    g"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;

    DecoratedDefinition::test_parse(
        r#"@buttons[0].clicked.connect
def spam():
    ..."#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn raise_statements() -> TestParseResult {
    RaiseStatement::test_parse(r#"raise"#, PYTHON_PARSERS.get("python").unwrap())?;
    RaiseStatement::test_parse(
        r#"raise RuntimeError('NO')"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;
    RaiseStatement::test_parse(
        r#"raise RunTimeError('NO') from e"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn global_statements() -> TestParseResult {
    GlobalStatement::test_parse(r#"global a"#, PYTHON_PARSERS.get("python").unwrap())?;
    GlobalStatement::test_parse(r#"global a, b"#, PYTHON_PARSERS.get("python").unwrap())
}

#[test]
fn exec_statements() -> TestParseResult {
    ExecStatement::test_parse(r#"exec '1+1'"#, PYTHON_PARSERS.get("python").unwrap())?;
    ExecStatement::test_parse(
        r#"exec 'x+=1' in None"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;
    ExecStatement::test_parse(
        r#"exec 'x+=1' in a, b"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )?;
    ExecStatement::test_parse(r#"exec func in {}"#, PYTHON_PARSERS.get("python").unwrap())
}
