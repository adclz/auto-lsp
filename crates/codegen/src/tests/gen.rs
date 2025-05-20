#[test]
fn gen_python() {
    let _result = crate::generate(
        tree_sitter_python::NODE_TYPES,
        &tree_sitter_python::LANGUAGE.into(),
        None,
    );
}
