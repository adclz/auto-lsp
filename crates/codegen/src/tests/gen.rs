#[test]
fn gen_python() {
    let _result = crate::generate(
        tree_sitter_python::NODE_TYPES,
        &tree_sitter_python::LANGUAGE.into(),
        None,
    );
}

#[test]
fn gen_html() {
    let _result = crate::generate(
        tree_sitter_html::NODE_TYPES,
        &tree_sitter_html::LANGUAGE.into(),
        None,
    );
}

#[test]
fn gen_javascript() {
    let _result = crate::generate(
        tree_sitter_javascript::NODE_TYPES,
        &tree_sitter_javascript::LANGUAGE.into(),
        Some(std::collections::HashMap::from([("`", "Backtick")])),
    );
}

#[test]
fn gen_c() {
    let _result = crate::generate(
        tree_sitter_c::NODE_TYPES,
        &tree_sitter_c::LANGUAGE.into(),
        Some(std::collections::HashMap::from([("\n", "Newline")])),
    );
}

#[test]
fn gen_c_sharp() {
    let _result = crate::generate(
        tree_sitter_c_sharp::NODE_TYPES,
        &tree_sitter_c_sharp::LANGUAGE.into(),
        None,
    );
}

#[test]
fn gen_haskell() {
    let _result = crate::generate(
        tree_sitter_haskell::NODE_TYPES,
        &tree_sitter_haskell::LANGUAGE.into(),
        Some(std::collections::HashMap::from([
            ("`", "Backtick"),
            ("←", "LeftArrow"),
            ("→", "RightArrow"),
            ("⇒", "DoubleRightArrow"),
            ("⊸", "RightTack"),
            ("∀", "Forall"),
            ("★", "Star"),
            ("∃", "Exists"),
            ("∷", "ColonColon"),
            ("⟦", "LeftDoubleBracket"),
            ("⟧", "RightDoubleBracket"),
        ])),
    );
}
