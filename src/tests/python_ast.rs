use auto_lsp_core::build::Parse;

use super::python_workspace::*;
use crate::python::ast::Function;

#[test]
fn function() -> miette::Result<()> {
    Function::miette_parse(
        r#"
       def foo():
           pass
       "#,
        &PYTHON_PARSERS.get("python").unwrap(),
    )
}
