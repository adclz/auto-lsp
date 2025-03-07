use auto_lsp_core::{document::Document, root::Root};
use lsp_types::Url;

use super::python_workspace::PYTHON_PARSERS;

pub fn create_python_workspace(source_code: &'static str) -> (Root, Document) {
    let mut root = Root::from_utf8(
        PYTHON_PARSERS.get("python").unwrap(),
        Url::parse("file:///test.py").unwrap(),
        source_code.into(),
    )
    .unwrap();

    root.0.resolve_references(&root.1);
    root.0.resolve_checks(&root.1);
    root.0.set_comments(&root.1);
    root
}
