use std::fs;

use crate::generate;

#[test]
fn generate_python() {
    let output = generate(
        tree_sitter_python::NODE_TYPES,
        &tree_sitter_python::LANGUAGE.into(),
    );

    let test_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/tests/python");

    fs::create_dir_all(&test_dir).expect("Failed to create output directory");
    fs::write(test_dir.join("generated_python.rs"), output.to_string())
        .expect("Failed to write output file");
}
