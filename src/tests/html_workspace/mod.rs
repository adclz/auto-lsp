/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/

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
        core: CORE_QUERY,
        ast_root: HtmlDocument
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
    end_tag: Option<EndTag>,
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
    end_tag: EndTag,
}

#[seq(query = "style_element")]
pub struct StyleElement {
    start_tag: StartTag,
    raw_text: Option<RawText>,
    end_tag: EndTag,
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
    attribute_value: AttributeValue,
}

#[seq(query = "text")]
pub struct Text {}
