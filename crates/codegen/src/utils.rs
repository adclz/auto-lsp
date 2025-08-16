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

use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
};

pub(crate) fn sanitize_string(string: &str) -> String {
    if let Some(v) = TOKENS.read().unwrap().get(string) {
        v.to_string()
    } else if let Some(v) = RUST_KEYWORDS.get(string) {
        v.to_string()
    } else {
        let mut result = String::new();
        for c in string.chars() {
            if c == '_' {
                result.push(c);
            } else if let Some(v) = TOKENS.read().unwrap().get(c.to_string().as_str()) {
                result.push_str(v);
            } else {
                result.push(c);
            }
        }
        result.replace(" ", "_")
    }
}

pub(crate) fn sanitize_string_to_pascal(string: &str) -> String {
    if let Some(v) = TOKENS.read().unwrap().get(string) {
        v.to_string()
    } else if let Some(v) = RUST_KEYWORDS.get(string) {
        v.to_string()
    } else {
        let mut result = String::new();
        for c in string.chars() {
            if c == '_' {
                result.push(c);
            } else if let Some(v) = TOKENS.read().unwrap().get(c.to_string().as_str()) {
                result.push_str(v);
            } else {
                result.push(c);
            }
        }
        result
            .replace(" ", "_")
            .split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => format!(
                        "{}{}",
                        first.to_ascii_uppercase(),
                        chars.collect::<String>()
                    ),
                    None => String::new(),
                }
            })
            .collect()
    }
}

pub(crate) static TOKENS: LazyLock<RwLock<HashMap<&'static str, &'static str>>> =
    LazyLock::new(|| {
        RwLock::new(HashMap::from([
            ("{", "LeftCurly"),
            ("}", "RightCurly"),
            ("(", "LeftParen"),
            (")", "RightParen"),
            ("[", "LeftBracket"),
            ("]", "RightBracket"),
            (",", "Comma"),
            (":", "Colon"),
            (";", "Semicolon"),
            (".", "Dot"),
            ("'", "Quote"),
            ("\"", "DoubleQuote"),
            ("@", "At"),
            ("!", "Bang"),
            ("#", "Hash"),
            ("$", "Dollar"),
            ("%", "Percent"),
            ("^", "Caret"),
            ("&", "Ampersand"),
            ("*", "Star"),
            ("-", "Minus"),
            ("_", "Underscore"),
            ("+", "Plus"),
            ("=", "Equal"),
            (">", "Greater"),
            ("<", "Less"),
            ("|", "Pipe"),
            ("~", "Tilde"),
            ("/", "Slash"),
            ("\\", "Backslash"),
            ("//", "SlashSlash"),
            ("//=>", "SlashSlashEqual"),
            ("//=", "SlashSlashEqual"),
            ("/=", "SlashEqual"),
            ("/>", "SlashGreater"),
            ("/?", "SlashQuestion"),
            ("/??", "SlashNullish"),
            ("/*", "SlashStar"),
            ("*/", "StarSlash"),
            ("+++", "PlusPlusPlus"),
            ("!!", "BangBang"),
            ("!!=", "BangBangEqual"),
            ("!!?", "BangBangQuestion"),
            ("!!??", "BangBangNullish"),
            ("!!???", "BangBangNullishQuestion"),
            ("?", "Question"),
            ("->", "Arrow"),
            ("=>", "FatArrow"),
            ("++", "PlusPlus"),
            ("--", "MinusMinus"),
            ("&&", "And"),
            ("||", "Or"),
            ("==", "EqualEqual"),
            ("!=", "NotEqual"),
            (">=", "GreaterEqual"),
            ("<=", "LessEqual"),
            ("===", "TripleEqual"),
            ("!==", "NotTripleEqual"),
            ("<<", "ShiftLeft"),
            (">>", "ShiftRight"),
            (">>>", "ShiftRightUnsigned"),
            ("+=", "PlusEqual"),
            ("-=", "MinusEqual"),
            ("*=", "StarEqual"),
            ("%=", "PercentEqual"),
            ("&=", "AmpersandEqual"),
            ("|=", "PipeEqual"),
            ("^=", "CaretEqual"),
            ("&&=", "AndEqual"),
            ("||=", "OrEqual"),
            ("??=", "NullishEqual"),
            ("??", "Nullish"),
            ("???", "NullishQuestion"),
            ("**", "StarStar"),
            ("**=", "StarStarEqual"),
            ("<>", "LessGreater"),
            ("<=>", "LessGreaterEqual"),
            ("<!", "LessBang"),
            ("</", "LessSlash"),
            ("0", "Zero"),
            ("1", "One"),
            ("2", "Two"),
            ("3", "Three"),
            ("4", "Four"),
            ("5", "Five"),
            ("6", "Six"),
            ("7", "Seven"),
            ("8", "Eight"),
            ("9", "Nine"),
        ]))
    });

pub static RUST_KEYWORDS: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "abstract" => "Abstract",
    "as" => "As",
    "async" => "Async",
    "await" => "Await",
    "break" => "Break",
    "const" => "Const",
    "continue" => "Continue",
    "crate" => "Crate",
    "dyn" => "Dyn",
    "else" => "Else",
    "enum" => "Enum",
    "extern" => "Extern",
    "false" => "False",
    "final" => "Final",
    "fn" => "Fn",
    "for" => "For",
    "if" => "If",
    "impl" => "Impl",
    "in" => "In",
    "let" => "Let",
    "loop" => "Loop",
    "match" => "Match",
    "mod" => "Mod",
    "move" => "Move",
    "mut" => "Mut",
    "pub" => "Pub",
    "ref" => "Ref",
    "return" => "Return",
    "self" => "Self",
    "static" => "Static",
    "struct" => "Struct",
    "super" => "Super",
    "trait" => "Trait",
    "true" => "True",
    "type" => "Type",
    "unsafe" => "Unsafe",
    "use" => "Use",
    "where" => "Where",
    "while" => "While",
    "with" => "With",
    "yield" => "Yield",
    "None" => "_None",
    "Some" => "_Some",
    "Ok" => "_Ok",
    "Err" => "_Err",
    "Result" => "_Result",
    "Option" => "_Option",
    "Vec" => "_Vec",
    "0" => "Zero",
    "1" => "One",
    "2" => "Two",
    "3" => "Three",
    "4" => "Four",
    "5" => "Five",
    "6" => "Six",
    "7" => "Seven",
    "8" => "Eight",
    "9" => "Nine",
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize() {
        assert_eq!(sanitize_string("fn"), "Fn");
        assert_eq!(
            sanitize_string("namespace_0_async_$"),
            "namespace_Zero_async_Dollar"
        );
        assert_eq!(sanitize_string("let"), "Let");
        assert_eq!(sanitize_string("match"), "Match");
        // Test special tokens
        assert_eq!(sanitize_string("=>"), "FatArrow");
        assert_eq!(sanitize_string("??"), "Nullish");
        // Test mixed content
        assert_eq!(
            sanitize_string("if_=>_??"),
            "if_EqualGreater_QuestionQuestion"
        );
        // Test empty string
        assert_eq!(sanitize_string(""), "");
        // Test numbers
        assert_eq!(sanitize_string("123"), "OneTwoThree");
        // Test special characters
        assert_eq!(sanitize_string("!@#"), "BangAtHash");
    }

    #[test]
    fn sanitize_to_pascal() {
        assert_eq!(sanitize_string("fn"), "Fn");
        assert_eq!(
            sanitize_string_to_pascal("namespace_0_async_$"),
            "NamespaceZeroAsyncDollar"
        );
        // Test Rust keywords
        assert_eq!(sanitize_string_to_pascal("let"), "Let");
        assert_eq!(sanitize_string_to_pascal("match"), "Match");
        // Test special tokens
        assert_eq!(sanitize_string_to_pascal("=>"), "FatArrow");
        assert_eq!(sanitize_string_to_pascal("??"), "Nullish");
        // Test mixed content with underscores
        assert_eq!(
            sanitize_string_to_pascal("if_=>_??"),
            "IfEqualGreaterQuestionQuestion"
        );
        // Test empty string
        assert_eq!(sanitize_string_to_pascal(""), "");
        // Test with spaces
        assert_eq!(sanitize_string_to_pascal("hello world"), "HelloWorld");
        // Test with mixed case
        assert_eq!(sanitize_string_to_pascal("hello_World"), "HelloWorld");
        // Test with multiple underscores
        assert_eq!(sanitize_string_to_pascal("hello__world"), "HelloWorld");

        // Test numbers
        assert_eq!(sanitize_string("0"), "Zero");
        assert_eq!(sanitize_string("123"), "OneTwoThree");
    }
}
