use crate::generate;

#[test]
fn gen_python() {
    let result = generate(&tree_sitter_python::NODE_TYPES, &tree_sitter_python::LANGUAGE.into());
}