use crate::{self as auto_lsp};
use crate::{choice, seq};

use crate::configure_parsers;

static CORE_QUERY: &'static str = "
(document) @document
(document (doctype) @doctype)
    
(element (start_tag (tag_name) @tag_name) (end_tag)) @element
(element (self_closing_tag (tag_name) @tag_name)) @element
(script_element (start_tag (tag_name) @tag_name) (end_tag)) @script_tag
(style_element (start_tag (tag_name) @tag_name) (end_tag)) @style_tag
";

configure_parsers!(
    "html" => {
        language: tree_sitter_html::LANGUAGE,
        node_types: tree_sitter_html::NODE_TYPES,
        ast_root: HtmlDocument,
        core: CORE_QUERY,
        comment: None,
        fold: None,
        highlights: None
    }
);

#[seq(query_name = "document", kind(symbol()))]
pub struct HtmlDocument {
    doctype: Option<DocType>,
    tags: Vec<Node>,
}

#[seq(query_name = "doctype", kind(symbol()))]
pub struct DocType {}

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
