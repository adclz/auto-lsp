use crate::core::ast::{AstSymbol, GetSymbolData};
use crate::core::build::Parse;
use crate::core::document::Document;
use crate::core::workspace::Workspace;
use lsp_types::Url;
use rstest::{fixture, rstest};

use super::html_workspace::*;
use super::python_workspace::*;

#[test]
fn function() -> miette::Result<()> {
    Function::parse(
        r#"
        def foo(param1, param2):
            pass 
        "#,
        &PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[fixture]
fn foo_bar() -> (Workspace, Document) {
    Workspace::from_utf8(
        &PYTHON_PARSERS.get("python").unwrap(),
        Url::parse("file:///test.py").unwrap(),
        r#"# foo comment
def foo(param1, param2: int, param3: int = 5):
    pass

def bar():
    pass  
"#
        .into(),
    )
    .unwrap()
}

#[rstest]
fn foo_bar_ast(foo_bar: (Workspace, Document)) {
    let ast = foo_bar.0.ast.as_ref().unwrap();
    let document = &foo_bar.1.texter;

    // Root node should be module

    assert!(ast.read().is::<Module>());
    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    // Both bar and foo should be found
    assert_eq!(module.functions.len(), 2);
    let function = module.functions[0].read();
    assert_eq!(
        function
            .name
            .read()
            .get_text(document.text.as_bytes())
            .unwrap(),
        "foo"
    );

    let function = module.functions[1].read();
    assert_eq!(
        function
            .name
            .read()
            .get_text(document.text.as_bytes())
            .unwrap(),
        "bar"
    );

    // Checks if bar's parent is module
    assert!(function.name.read().get_parent().is_some());
    let parent = function.name.read().get_parent().unwrap();
    assert!(parent.to_dyn().unwrap().read().is::<Function>());
}

#[rstest]
fn foo_bar_ast_parameters(foo_bar: (Workspace, Document)) {
    let ast = foo_bar.0.ast.as_ref().unwrap();
    let document = &foo_bar.1.texter;
    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    // Foo has 3 parameters
    let function = module.functions[0].read();
    assert_eq!(function.parameters.read().parameters.len(), 3);
    let parameters = &function.parameters.read().parameters;

    // param1 is untyped
    assert!(matches!(*parameters[0].read(), Parameter::Identifier(_)));

    // param2 is typed
    assert!(matches!(*parameters[1].read(), Parameter::Typed(_)));
    if let Parameter::Typed(typed) = &*parameters[1].read() {
        assert_eq!(
            typed
                .name
                .read()
                .get_text(document.text.as_bytes())
                .unwrap(),
            "param2"
        );

        assert_eq!(
            typed
                .parameter_type
                .read()
                .get_text(document.text.as_bytes())
                .unwrap(),
            "int"
        );
    } else {
        panic!("Expected Typed parameter");
    }

    // param3 is typed with default value
    if let Parameter::TypedDefault(typed_default) = &*parameters[2].read() {
        assert_eq!(
            typed_default
                .name
                .read()
                .get_text(document.text.as_bytes())
                .unwrap(),
            "param3"
        );

        assert_eq!(
            typed_default
                .parameter_type
                .read()
                .get_text(document.text.as_bytes())
                .unwrap(),
            "int"
        );

        assert!(typed_default.value.read().is_integer());

        assert_eq!(
            typed_default
                .value
                .read()
                .get_text(document.text.as_bytes())
                .unwrap(),
            "5"
        );
    } else {
        panic!("Expected TypedDefault parameter");
    }

    // param3 is typed with default value
    assert!(matches!(*parameters[2].read(), Parameter::TypedDefault(_)));
}

#[fixture]
fn sample_file() -> (Workspace, Document) {
    Workspace::from_utf8(
        &HTML_PARSERS.get("html").unwrap(),
        Url::parse("file:///sample_file.html").unwrap(),
        r#"<!DOCTYPE html>
<script></script>
<style></style>
<div>
	<span> </span>
    <br/>
</div>"#
            .into(),
    )
    .unwrap()
}

#[rstest]
fn html_ast(sample_file: (Workspace, Document)) {
    let workspace = sample_file.0;
    let document = sample_file.1;

    let ast = workspace.ast.as_ref().unwrap();
    let ast = ast.read();

    // Root node should be HtmlDocument

    assert!(ast.is::<HtmlDocument>());
    let html = ast.downcast_ref::<HtmlDocument>().unwrap();
    let tags = &html.tags;

    // Should contain Script, Style, and Element

    assert_eq!(tags.len(), 3);
    assert!(matches!(*tags[0].read(), Node::Script(_)));
    assert!(matches!(*tags[1].read(), Node::Style(_)));
    assert!(matches!(*tags[2].read(), Node::Element(_)));

    let tag_3 = tags[2].read();

    // Checks if Element node is a div

    if let Node::Element(ref element) = *tag_3 {
        let tag_name = element.tag_name.read();
        assert_eq!(
            tag_name.get_text(document.texter.text.as_bytes()).unwrap(),
            "div"
        );

        // Checks if Element node contains 2 children (span and self closing br)

        let elements = &element.elements;
        assert_eq!(elements.len(), 2);

        // Tag name should be span

        assert_eq!(
            elements[0]
                .read()
                .tag_name
                .read()
                .get_text(document.texter.text.as_bytes())
                .unwrap(),
            "span"
        );

        // Tag name should be br

        assert_eq!(
            elements[1]
                .read()
                .tag_name
                .read()
                .get_text(document.texter.text.as_bytes())
                .unwrap(),
            "br"
        );
    } else {
        panic!("Expected Element node");
    }
}
