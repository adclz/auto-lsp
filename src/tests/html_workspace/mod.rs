use crate::{self as auto_lsp};
use crate::{choice, seq};

use crate::configure_parsers;

static CORE_QUERY: &'static str = "
(document) @document
(document (doctype) @doctype)
(comment) @comment

(element (start_tag (tag_name) @tag_name) (end_tag)) @element
(element (self_closing_tag (tag_name) @tag_name)) @element
(script_element (start_tag (tag_name) @tag_name) (end_tag)) @script_tag
(style_element (start_tag (tag_name) @tag_name) (end_tag)) @style_tag
";

static COMMENT_QUERY: &'static str = "
(comment) @comment
";

configure_parsers!(
    HTML_PARSERS,
    "html" => {
        language: tree_sitter_html::LANGUAGE,
        node_types: tree_sitter_html::NODE_TYPES,
        ast_root: HtmlDocument,
        core: CORE_QUERY,
        comment: Some(COMMENT_QUERY),
        fold: None,
        highlights: None
    }
);

#[seq(query = "document")]
pub struct HtmlDocument {
    doctype: Option<DocType>,
    tags: Vec<Node>,
}

#[seq(query = "doctype")]
pub struct DocType {}

#[choice]
pub enum Node {
    Element(Element),
    Script(Script),
    Style(Style),
}

#[seq(query = "element")]
pub struct Element {
    tag_name: TagName,
    elements: Vec<Element>,
}

#[seq(query = "tag_name")]
pub struct TagName {}

#[seq(query = "script_tag")]
pub struct Script {}

#[seq(query = "style_tag")]
pub struct Style {}
