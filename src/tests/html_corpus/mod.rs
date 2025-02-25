use auto_lsp_core::build::TryParse;

use super::html_workspace::*;
use crate::html::HtmlDocument;

#[test]
fn tags() -> Result<(), ()> {
    HtmlDocument::try_parse(r#"<span>Hello</span>"#, HTML_PARSERS.get("html").unwrap())
}

#[test]
fn tags_with_attributes() -> Result<(), ()> {
    HtmlDocument::try_parse(
        r#"<input value=yes class="a" data-💩></input>"#,
        HTML_PARSERS.get("html").unwrap(),
    )
}

#[test]
fn nested_tags() -> Result<(), ()> {
    HtmlDocument::try_parse(
        r#"<div>
  <span>a</span>
  b
  <b>c</b>
  Multi-line
  text
</div>"#,
        HTML_PARSERS.get("html").unwrap(),
    )
}

#[test]
fn void_tags() -> Result<(), ()> {
    HtmlDocument::try_parse(
        r#"<form><img src="something.png"><br><input type=submit value=Ok /></form>"#,
        HTML_PARSERS.get("html").unwrap(),
    )
}

#[test]
fn void_tags_at_eof() -> Result<(), ()> {
    HtmlDocument::try_parse(
        r#"<img src="something.png">"#,
        HTML_PARSERS.get("html").unwrap(),
    )
}

#[test]
fn custom_tags() -> Result<(), ()> {
    HtmlDocument::try_parse(
        r#"<something:different>
  <atom-text-editor mini>
    Hello
  </atom-text-editor>
</something:different>"#,
        HTML_PARSERS.get("html").unwrap(),
    )
}

#[test]
fn raw_text_elements() -> Result<(), ()> {
    HtmlDocument::try_parse(
        r#"<script>
  </s
  </sc
  </scr
  </scri
  </scrip
</script>

<style>
  </ </s </st </sty </styl
</style>

<script>
</SCRIPT>"#,
        HTML_PARSERS.get("html").unwrap(),
    )
}

#[test]
fn all_caps_doctype() -> Result<(), ()> {
    HtmlDocument::try_parse(
        r#"<!DOCTYPE html PUBLIC
  "-//W3C//DTD XHTML 1.0 Transitional//EN"
  "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">"#,
        HTML_PARSERS.get("html").unwrap(),
    )
}

#[test]
fn li_elements_without_close_tags() -> Result<(), ()> {
    HtmlDocument::try_parse(
        r#"<ul>
  <li>One
  <li>Two
</ul>"#,
        HTML_PARSERS.get("html").unwrap(),
    )
}

#[test]
fn dt_and_dl_elements_without_close_tags() -> Result<(), ()> {
    HtmlDocument::try_parse(
        r#"<dl>
  <dt>Coffee
  <dt>Café
  <dd>Black hot drink
  <dt>Milk
  <dd>White cold drink
</dl>"#,
        HTML_PARSERS.get("html").unwrap(),
    )
}

#[test]
fn p_elements_without_close_tags() -> Result<(), ()> {
    HtmlDocument::try_parse(
        r#"<p>One
<div>Two</div>
<p>Three
<p>Four
<h1>Five</h1>"#,
        HTML_PARSERS.get("html").unwrap(),
    )
}

#[test]
fn ruby_annotation_elements_without_close_tags() -> Result<(), ()> {
    HtmlDocument::try_parse(
        r#"<ruby>東<rb>京<rt>とう<rt>きょう</ruby>"#,
        HTML_PARSERS.get("html").unwrap(),
    )
}

#[test]
fn col_group_elements_without_end_tags() -> Result<(), ()> {
    HtmlDocument::try_parse(
        r#"<table>
  <colgroup>
    <col style="background-color: #0f0">
    <col span="2">
  <tr>
    <th>Lime</th>
    <th>Lemon</th>
    <th>Orange</th>
  </tr>
</table>"#,
        HTML_PARSERS.get("html").unwrap(),
    )
}

#[test]
fn tr_td_th_elements_without_end_tags() -> Result<(), ()> {
    HtmlDocument::try_parse(
        r#"<table>
  <tr>
    <th>One
    <th>Two
  <tr>
    <td>Three
    <td>Four
</table>"#,
        HTML_PARSERS.get("html").unwrap(),
    )
}

#[test]
fn named_entities_in_tag_content() -> Result<(), ()> {
    HtmlDocument::try_parse(
        r#"<p>Lorem ipsum &nbsp; dolor sit &copy; amet.</p>"#,
        HTML_PARSERS.get("html").unwrap(),
    )
}

#[test]
fn numeric_entities_in_tag_content() -> Result<(), ()> {
    HtmlDocument::try_parse(
        r#"<p>Lorem ipsum &#160; dolor sit &#8212; amet.</p>"#,
        HTML_PARSERS.get("html").unwrap(),
    )
}

#[test]
fn multiple_entities_in_tag_content() -> Result<(), ()> {
    HtmlDocument::try_parse(
        r#"<p>Lorem ipsum &#xA0; dolor &#xa0; sit &nbsp; amet.</p>"#,
        HTML_PARSERS.get("html").unwrap(),
    )
}

#[test]
fn omitted_end_tags() -> Result<(), ()> {
    HtmlDocument::try_parse(
        r#"<!doctype html><html><head>"#,
        HTML_PARSERS.get("html").unwrap(),
    )
}
