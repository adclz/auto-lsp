use auto_lsp_codegen::generate;
use std::{fs, path::PathBuf};

fn main() {
    if std::env::var("AST_GEN").unwrap_or("0".to_string()) == "0" {
        //return;
    }

    let output_path = PathBuf::from("./src/generated.rs");

    fs::write(
        output_path,
        generate(
            tree_sitter_python::NODE_TYPES,
            &tree_sitter_python::LANGUAGE.into(),
            None,
        )
        .to_string(),
    )
    .unwrap();
}
