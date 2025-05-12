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
fn integers() -> Result {
    snap!(
        r#"
-1
0xDEAD
0XDEAD
1j
-1j
0o123
0O123
0b001
0B001
1_1
0B1_1
0O1_1
0L
"#
    )
}

#[test]
fn floats() -> Result {
    snap!(
        r#"
-.6_6
+.1_1
123.4123
123.123J
1_1.3_1
1_1.
1e+3_4j
.3e1_4
"#
    )
}

#[test]
fn scientific_notation_floats() -> Result {
    snap!(
        r#"
1e322
1e-3
1e+3
1.8e10
1.e10
-1e10
"#
    )
}

#[test]
fn strings() -> Result {
    snap!(
        r#"
"I'm ok"
'"ok"'
UR'bye'
b'sup'
B"sup"
`1`
"\\"
"/"
"multiline \
string"
b"\x12\u12\U12\x13\N{WINKING FACE}"
"\xab\1\12\123\'\"\a\b\f\r\n\t\v\\"
"\xgh\o123\p\q\c\d\e\u12\U1234"
f'\N{GREEK CAPITAL LETTER DELTA}'
"#
    )
}

#[test]
fn raw_strings() -> Result {
    snap!(
        r#"
'ab\x00cd'
"\n"

# no escape sequences in these
r'ab\x00cd'
ur"\n"

# raw f-string
fr"\{0}"

r"\\"
r'"a\
de\
fg"'
"#
    )
}

#[test]
fn raw_strings_with_escaped_quotes() -> Result {
    snap!(
        r#"
re.compile(r"(\n|\A)#include\s*['\"]"
           r"(?P<name>[\w\d./\\]+[.]src)['\"]")
"#
    )
}

#[test]
fn format_strings() -> Result {
    snap!(
        r#"
# nested!
f"a {b(f'c {e} d')} e"
f"""a"{b}c"""
f"""a""{b}c"""
f"a {{}} e"
f"a {b}}}"
f"a {{{b}"
f"a {{b}}"
f"a {{{b}}}"
f"{c,}"
f"{yield d}"
f"{*a,}"

def function():
    return f"""
{"string1" if True else
 "string2"}"""

def test(self):
    self.assertEqual(f'''A complex trick: {
2  # two
}''', 'A complex trick: 2')
"#
    )
}

#[test]
fn format_strings_with_specifiers() -> Result {
    snap!(
        r#"
f"a {b:2} {c:34.5}"
f"{b:{c.d}.{d.e}}"
f"{a:#06x}"
f"{a=}"
f"{a=:.2f}"
f"{value:{width + padding!r}.{precision}}"
"#
    )
}

#[test]
fn unicode_escape_sequences() -> Result {
    snap!(
        r#"
"\x12 \123 \u1234"
"#
    )
}

#[test]
fn other_primitives() -> Result {
    snap!(
        r#"
True
False
None
"#
    )
}

#[test]
fn concatenated_strings() -> Result {
    snap!(
        r#"
"one" "two" "three"
"#
    )
}

#[test]
fn multi_line_strings() -> Result {
    snap!(
        r#"
"""
A double quote hello,
without double or single quotes.
"""

"""
A double quote "hello",
with double quotes.
"""

"""
A double quote 'hello',
with single quotes.
"""

'''
A single quote hello,
without double or single quotes.
'''

'''
A single quote 'hello',
with single quotes.
'''

'''
A single quote "hello",
with double quotes.
'''

"""
A double quote hello\n\
with an escaped newline\n\
and another escaped newline\n\
"""
"#
    )
}

#[test]
fn lists() -> Result {
    snap!(
        r#"
[a, b, [c, d]]
[*()]
[*[]]
[*a]
[*a.b]
[*a[b].c]
[*a()]
"#
    )
}

#[test]
fn list_comprehensions() -> Result {
    snap!(
        r#"
[a + b for (a, b) in items]
[a for b in c for a in b]
[(x,y) for x in [1,2,3] for y in [1,2,3] if True]
[a for a in lambda: True, lambda: False if a()]
"#
    )
}

#[test]
fn dictionaries() -> Result {
    snap!(
        r#"
{a: 1, b: 2}
{}
{**{}}
{**a}
{**a.b}
{**a[b].c}
{**a()}
"#
    )
}

#[test]
fn dictionary_comprehensions() -> Result {
    snap!(
        r#"
{a: b for a, b in items}
{a: b for c in d for e in items}
"#
    )
}

#[test]
fn sets() -> Result {
    snap!(
        r#"
{a, b, c,}
{*{}}
"#
    )
}

#[test]
fn set_comprehensions() -> Result {
    snap!(
        r#"
{a[b][c] for a, b, c in items}
{r for s in qs for n in ms}
"#
    )
}

#[test]
fn simple_tuples() -> Result {
    snap!(
        r#"
()
(a, b)
(a, b, c,)
(print, exec)
"#
    )
}

#[test]
fn generator_expressions() -> Result {
    snap!(
        r#"
(a[b][c] for a, b, c in items)
dict((a, b) for a, b in d)
(a for b in c for d in e,)
(x for x in range(1, 10))
"#
    )
}
