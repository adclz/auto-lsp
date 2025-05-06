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
#[case("-1")]
#[case("0xDEAD")]
#[case("0XDEAD")]
#[case("1j")]
#[case("-1j")]
#[case("0o123")]
#[case("0O123")]
#[case("0b001")]
#[case("0B001")]
#[case("1_1")]
#[case("0B1_1")]
#[case("0O1_1")]
#[case("0L")]
fn integers(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("-.6_6")]
#[case("+.1_1")]
#[case("123.4123")]
#[case("123.123J")]
#[case("1_1.3_1")]
#[case("1_1.")]
#[case("1e+3_4j")]
#[case(".3e1_4")]
fn floats(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("1e322")]
#[case("1e-3")]
#[case("1e+3")]
#[case("1.8e10")]
#[case("1.e10")]
#[case("-1e10")]
fn scientific_notation_floats(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case(r#""I'm ok""#)]
#[case(r#"'"ok"'"#)]
#[case(r#"UR'bye'"#)]
#[case(r#"b'sup'"#)]
#[case(r#"B"sup""#)]
#[case(r#"`1`"#)]
#[case(r#""\\"#)]
#[case(r#""/""#)]
#[case(
    r#""multiline \
string""#
)]
#[case(r#"b"\x12\u12\U12\x13\N{WINKING FACE}""#)]
#[case(r#""\xab\1\12\123\'\"\a\b\f\r\n\t\v\\""#)]
#[case(r#""\xgh\o123\p\q\c\d\e\u12\U1234""#)]
#[case(r#"f'\N{GREEK CAPITAL LETTER DELTA}'"#)]
fn strings(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case(r#"'ab\x00cd'"#)]
#[case(r#""\n""#)]
#[case(r#"r'ab\x00cd'"#)]
#[case(r#"ur"\n""#)]
#[case(r#"fr"\{0}""#)]
#[case(r#"r"\\""#)]
#[case(
    r#"r'"a\
de\
fg"'"#
)]
fn raw_strings(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case(
    r#"re.compile(r"(\n|\A)#include\s*['\"]"
           r"(?P<name>[\w\d./\\]+[.]src)['\"]")"#
)]
fn raw_strings_with_escaped_quotes(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case(r#"f"a {b(f'c {e} d')} e""#)]
#[case(r#"f"""a"{b}c""""#)]
#[case(r#"f"""a""{b}c""""#)]
#[case(r#"f"a {{}} e""#)]
#[case(r#"f"a {b}}}""#)]
#[case(r#"f"a {{{b}""#)]
#[case(r#"f"a {{b}}""#)]
#[case(r#"f"a {{{b}}}""#)]
#[case(r#"f"{c,}""#)]
#[case(r#"f"{yield d}""#)]
#[case(r#"f"{*a,}""#)]
#[case(
    r#"def function():
    return f"""
{"string1" if True else
 "string2"}""""#
)]
#[case(
    r#"def test(self):
    self.assertEqual(f'''A complex trick: {
2  # two
}''', 'A complex trick: 2')"#
)]
fn format_strings(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case(r#"f"a {b:2} {c:34.5}""#)]
#[case(r#"f"{b:{c.d}.{d.e}}""#)]
#[case(r#"f"{a:#06x}""#)]
#[case(r#"f"{a=}""#)]
#[case(r#"f"{a=:.2f}""#)]
#[case(r#"f"{value:{width + padding!r}.{precision}}""#)]
fn format_strings_with_format_specifiers(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case(r#""\x12 \123 \u1234""#)]
fn unicode_escape_sequences(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("True")]
#[case("False")]
#[case("None")]
fn other_primitives(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case(r#""one" "two" "three""#)]
fn concatenated_strings(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case(
    r#""""
A double quote hello,
without double or single quotes.
""""#
)]
#[case(
    r#""""
A double quote "hello",
with double quotes.
""""#
)]
#[case(
    r#""""
A double quote 'hello',
with single quotes.
""""#
)]
#[case(
    r#"'''
A single quote hello,
without double or single quotes.
'''"#
)]
#[case(
    r#"'''
A single quote 'hello',
with single quotes.
'''"#
)]
#[case(
    r#"'''
A single quote "hello",
with double quotes.
'''"#
)]
#[case(
    r#""""
A double quote hello\n\
with an escaped newline\n\
and another escaped newline\n\
""""#
)]
fn multi_line_strings(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("[a, b, [c, d]]")]
#[case("[*()]")]
#[case("[*[]]")]
#[case("[*a]")]
#[case("[*a.b]")]
#[case("[*a[b].c]")]
#[case("[*a()]")]
fn lists(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("[a + b for (a, b) in items]")]
#[case("[a for b in c for a in b]")]
#[case("[(x,y) for x in [1,2,3] for y in [1,2,3] if True]")]
#[case("[a for a in lambda: True, lambda: False if a()]")]
fn list_comprehensions(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("{a: 1, b: 2}")]
#[case("{}")]
#[case("{**{}}")]
#[case("{**a}")]
#[case("{**a.b}")]
#[case("{**a[b].c}")]
#[case("{**a()}")]
fn dictionaries(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("{a: b for a, b in items}")]
#[case("{a: b for c in d for e in items}")]
fn dictionary_comprehensions(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("{a, b, c,}")]
#[case("{*{}}")]
fn sets(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("{a[b][c] for a, b, c in items}")]
#[case("{r for s in qs for n in ms}")]
fn set_comprehensions(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("()")]
#[case("(a, b)")]
#[case("(a, b, c,)")]
#[case("(print, exec)")]
fn simple_tuples(#[case] input: &str) -> Result {
    snap!(input)
}

#[rstest]
#[case("(a[b][c] for a, b, c in items)")]
#[case("dict((a, b) for a, b in d)")]
#[case("(a for b in c for d in e,)")]
#[case("(x for x in range(1, 10))")]
fn generator_expressions(#[case] input: &str) -> Result {
    snap!(input)
}
