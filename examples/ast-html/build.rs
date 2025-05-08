use auto_lsp_codegen::generate;
use std::{fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=../../crates/codegen/src");

    let output_path = PathBuf::from("./src/generated.rs");

    fs::write(
        output_path,
        generate(
            tree_sitter_html::NODE_TYPES,
            &tree_sitter_html::LANGUAGE.into(),
        )
        .to_string(),
    )
    .unwrap();
}
