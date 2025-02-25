use crate::{self as auto_lsp};
use crate::{choice, seq};

use crate::configure_parsers;

static CORE_QUERY: &str = "
(document) @document
(doctype) @doctype
(text) @text
(element
	. (start_tag)
) @full_tag

(self_closing_tag) @self_closing_tag

(script_element) @script_element
(style_element) @style_element
(erroneous_end_tag) @erroneous
(erroneous_end_tag_name) @erroneous_name

(start_tag) @start_tag
(tag_name) @tag_name
(end_tag) @end_tag
(attribute) @attribute
(attribute_name) @attribute_name
(attribute_value) @attribute_value
(entity) @entity
(quoted_attribute_value) @quoted_attribute_value
";

static COMMENT_QUERY: &str = "
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
    tags: Vec<Node>,
}

#[seq(query = "doctype")]
pub struct DocType {}

#[choice]
pub enum Node {
    DocType(DocType),
    Entity(Entity),
    Text(Text),
    Element(Element),
    Script(ScriptElement),
    Style(StyleElement),
    Erroneous(ErroneousEndTag),
}

#[choice]
pub enum Element {
    FullTag(FullTag),
    SelfClosingTag(SelfClosingTag),
}

#[seq(query = "full_tag")]
pub struct FullTag {
    start_tag: StartTag,
    elements: Vec<Node>,
    end_tag: Option<EndTag>
}

#[seq(query = "self_closing_tag")]
pub struct SelfClosingTag {
    tag_name: TagName,
    attributes: Vec<Attribute>,
}

#[seq(query = "script_element")]
pub struct ScriptElement {
    start_tag: StartTag,
    raw_text: Option<RawText>,
    end_tag: EndTag
}

#[seq(query = "style_element")]
pub struct StyleElement {
    start_tag: StartTag,
    raw_text: Option<RawText>,
    end_tag: EndTag
}

#[seq(query = "start_tag")]
pub struct StartTag {
    tag_name: TagName,
    attributes: Vec<Attribute>,
}

#[seq(query = "end_tag")]
pub struct EndTag {
    tag_name: TagName,
}

#[seq(query = "tag_name")]
pub struct TagName {}

#[seq(query = "raw_text")]
pub struct RawText {}

#[seq(query = "erroneous_end_tag")]
pub struct ErroneousEndTag {
    tag_name: ErroneousEndTagName,
}

#[seq(query = "erroneous_end_tag_name")]
pub struct ErroneousEndTagName {}

#[seq(query = "attribute")]
pub struct Attribute {
    name: AttributeName,
    value: Option<ValueOrQuotedValue>,
}

#[seq(query = "attribute_name")]
pub struct AttributeName {}

#[choice]
pub enum ValueOrQuotedValue {
    Value(AttributeValue),
    QuotedValue(QuotedAttributeValue),
}

#[seq(query = "attribute_value")]
pub struct AttributeValue {}

#[seq(query = "entity")]
pub struct Entity {}

#[seq(query = "quoted_attribute_value")]
pub struct QuotedAttributeValue {
    attribute_value: AttributeValue
}

#[seq(query = "text")]
pub struct Text {}
