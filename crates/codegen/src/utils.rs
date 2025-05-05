pub(crate) fn sanitize_string(string: &str) -> String {
    if let Some(v) = PUNCTUATION.get(string) {
        v.to_string()
    } else if let Some(v) = RUST_KEYWORDS.get(string) {
        v.to_string()
    } else {
        let mut result = String::new();
        for c in string.chars() {
            if c == '_' {
                result.push(c);
            } else if let Some(v) = PUNCTUATION.get(&c.to_string()) {
                result.push_str(v);
            } else {
                result.push(c);
            }
        }
        result.replace(" ", "_")
    }
}

pub(crate) fn sanitize_string_to_pascal(string: &str) -> String {
    if let Some(v) = PUNCTUATION.get(string) {
        v.to_string()
    } else if let Some(v) = RUST_KEYWORDS.get(string) {
        v.to_string()
    } else {
        let mut result = String::new();
        for c in string.chars() {
            if c == '_' {
                result.push(c);
            } else if let Some(v) = PUNCTUATION.get(&c.to_string()) {
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

pub static PUNCTUATION: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "{" => "LeftCurly",
    "}" => "RightCurly",
    "(" => "LeftParen",
    ")" => "RightParen",
    "[" => "LeftBracket",
    "]" => "RightBracket",
    "," => "Comma",
    ":" => "Colon",
    ";" => "Semicolon",
    "." => "Dot",
    "@" => "At",
    "!" => "Bang",
    "#" => "Hash",
    "$" => "Dollar",
    "%" => "Percent",
    "^" => "Caret",
    "&" => "Ampersand",
    "*" => "Star",
    "-" => "Minus",
    "_" => "Underscore",
    "+" => "Plus",
    "=" => "Equal",
    ">" => "Greater",
    "<" => "Less",
    "|" => "Pipe",
    "~" => "Tilde",
    "/" => "Slash",
    "\\" => "Backslash",
    "//" => "SlashSlash",
    "//=>" => "SlashSlashEqual",
    "//=" => "SlashSlashEqual",
    "/=" => "SlashEqual",
    "/>" => "SlashGreater",
    "/?" => "SlashQuestion",
    "/??" => "SlashNullish",
    "/*" => "SlashStar",
    "*/" => "StarSlash",
    "+++" => "PlusPlusPlus",
    "!!" => "BangBang",
    "!!=" => "BangBangEqual",
    "!!?" => "BangBangQuestion",
    "!!??" => "BangBangNullish",
    "!!???" => "BangBangNullishQuestion",
    "?" => "Question",
    "->" => "Arrow",
    "=>" => "FatArrow",
    "++" => "PlusPlus",
    "--" => "MinusMinus",
    "&&" => "And",
    "||" => "Or",
    "==" => "EqualEqual",
    "!=" => "NotEqual",
    ">=" => "GreaterEqual",
    "<=" => "LessEqual",
    "===" => "TripleEqual",
    "!==" => "NotTripleEqual",
    "<<" => "ShiftLeft",
    ">>" => "ShiftRight",
    ">>>" => "ShiftRightUnsigned",
    "+=" => "PlusEqual",
    "-=" => "MinusEqual",
    "*=" => "StarEqual",
    "%=" => "PercentEqual",
    "&=" => "AmpersandEqual",
    "|=" => "PipeEqual",
    "^=" => "CaretEqual",
    "&&=" => "AndEqual",
    "||=" => "OrEqual",
    "??=" => "NullishEqual",
    "??" => "Nullish",
    "???" => "NullishQuestion",
    "**" => "StarStar",
    "**=" => "StarStarEqual",
    "<>" => "LessGreater",
    "<=>" => "LessGreaterEqual",
};

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
};
