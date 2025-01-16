use auto_lsp_core::ast::{AstSymbol, StaticUpdate, Symbol};
use auto_lsp_core::build::MainBuilder;
use auto_lsp_core::workspace::{Document, Workspace};
use auto_lsp_macros::{choice, seq};
use lsp_types::Url;
use std::sync::{Arc, LazyLock};
use texter::core::text::Text;

use crate as auto_lsp;

use crate::configure_parsers;

static CORE_QUERY: &'static str = "
(document) @document

(element
    (start_tag
    	(tag_name) @tag_name
	)
    (end_tag)
) @element

(element
	(self_closing_tag
		(tag_name) @tag_name
	)
) @element

(script_element
    (start_tag
    	(tag_name) @tag_name
	)
    (end_tag)
) @script_tag

(style_element
    (start_tag
    	(tag_name) @tag_name
	)
    (end_tag)
) @style_tag
";

configure_parsers!(
    "html" => {
        language: tree_sitter_html::LANGUAGE,
        ast_root: HtmlDocument,
        core: CORE_QUERY,
        comment: None,
        fold: None,
        highlights: None
    }
);

#[seq(query_name = "document", kind(symbol()))]
pub struct HtmlDocument {
    tags: Vec<Node>,
}

#[choice]
pub enum Node {
    Element(Element),
    Script(Script),
    Style(Style),
}

#[seq(query_name = "element", kind(symbol()))]
pub struct Element {
    tag_name: TagName,
    elements: Vec<Element>,
}

#[seq(query_name = "tag_name", kind(symbol()))]
pub struct TagName {}

#[seq(query_name = "script_tag", kind(symbol()))]
pub struct Script {}

#[seq(query_name = "style_tag", kind(symbol()))]
pub struct Style {}

fn create_html_workspace(uri: Url, source_code: String) -> Workspace {
    let parse = PARSERS.get("html").unwrap();

    let tree = parse
        .cst_parser
        .parser
        .write()
        .parse(source_code.as_bytes(), None)
        .unwrap();

    let document = Document {
        document: Text::new(source_code.into()),
        cst: tree,
    };

    let mut diagnostics = vec![];
    let mut unsolved_checks = vec![];
    let mut unsolved_references = vec![];

    let mut params = MainBuilder {
        query: &parse.cst_parser.queries.core,
        document: &document,
        url: Arc::new(uri),
        diagnostics: &mut diagnostics,
        unsolved_checks: &mut unsolved_checks,
        unsolved_references: &mut unsolved_references,
    };

    let ast_parser = parse.ast_parser;
    let ast = ast_parser(&mut params, None).unwrap();

    let workspace = Workspace {
        parsers: parse,
        document,
        errors: diagnostics,
        ast: Some(ast),
        unsolved_checks,
        unsolved_references,
    };

    workspace
}

static TEST_FILE: LazyLock<Workspace> = LazyLock::new(|| {
    create_html_workspace(
        Url::parse("file:///test.html").unwrap(),
        r#" 
<script></script>
<style></style>
<div>
	<span> </span>
    <br/>
</div>
"#
        .into(),
    )
});

#[test]
fn check_ast() {
    let workspace = &TEST_FILE;
    let ast = workspace.ast.as_ref().unwrap();
    let ast = ast.read();

    // Root node should be HtmlDocument

    assert!(ast.is::<HtmlDocument>());
    let html = ast.downcast_ref::<HtmlDocument>().unwrap();
    let tags = &html.tags;

    // Should contains Script, Style, and Element

    assert_eq!(tags.len(), 3);
    assert!(matches!(*tags[0].read(), Node::Script(_)));
    assert!(matches!(*tags[1].read(), Node::Style(_)));
    assert!(matches!(*tags[2].read(), Node::Element(_)));

    let tag_3 = tags[2].read();

    // Checks if Element node is a div

    if let Node::Element(ref element) = *tag_3 {
        let tag_name = element.tag_name.read();
        assert_eq!(
            tag_name
                .get_text(workspace.document.document.text.as_bytes())
                .unwrap(),
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
                .get_text(workspace.document.document.text.as_bytes())
                .unwrap(),
            "span"
        );

        // Tag name should be br

        assert_eq!(
            elements[1]
                .read()
                .tag_name
                .read()
                .get_text(workspace.document.document.text.as_bytes())
                .unwrap(),
            "br"
        );
    } else {
        panic!("Expected Element node");
    }
}
