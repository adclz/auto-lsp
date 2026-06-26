use crate::snap;
use utils::Result;
mod utils;

#[test]
fn tags() -> Result {
    snap!("<span>Hello</span>", tags)
}

#[test]
fn tags_with_attributes() -> Result {
    snap!(
        r#"<input value=yes class="a" data-💩></input>"#,
        tags_with_attributes
    )
}

#[test]
fn nested_tags() -> Result {
    snap!(
        r#"<div>
  <span>a</span>
  b
  <b>c</b>
  Multi-line
  text
</div>"#,
        nested_tags
    )
}

#[test]
fn void_tags() -> Result {
    snap!(
        r#"<form><img src="something.png"><br><input type=submit value=Ok /></form>"#,
        void_tags
    )
}

#[test]
fn void_tags_at_eof() -> Result {
    snap!(r#"<img src="something.png">"#, void_tags_at_eof)
}

#[test]
fn custom_tags() -> Result {
    snap!(
        r#"<something:different>
  <atom-text-editor mini>
    Hello
  </atom-text-editor>
</something:different>"#,
        custom_tags
    )
}

#[test]
fn raw_text_elements() -> Result {
    snap!(
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
        raw_text_elements
    )
}

#[test]
fn all_caps_doctype() -> Result {
    snap!(
        r#"<!DOCTYPE html PUBLIC
  "-//W3C//DTD XHTML 1.0 Transitional//EN"
  "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">"#,
        all_caps_doctype
    )
}

#[test]
fn li_elements_without_close_tags() -> Result {
    snap!(
        r#"<ul>
  <li>One
  <li>Two
</ul>"#,
        li_elements_without_close_tags
    )
}

#[test]
fn dt_and_dl_elements_without_close_tags() -> Result {
    snap!(
        r#"<dl>
  <dt>Coffee
  <dt>Café
  <dd>Black hot drink
  <dt>Milk
  <dd>White cold drink
</dl>"#,
        dt_and_dl_elements_without_close_tags
    )
}

#[test]
fn p_elements_without_close_tags() -> Result {
    snap!(
        r#"<p>One
<div>Two</div>
<p>Three
<p>Four
<h1>Five</h1>"#,
        p_elements_without_close_tags
    )
}

#[test]
fn ruby_annotation_elements_without_close_tags() -> Result {
    snap!(
        r#"<ruby>東<rb>京<rt>とう<rt>きょう</ruby>"#,
        ruby_annotation_elements_without_close_tags
    )
}

#[test]
fn col_group_elements_without_end_tags() -> Result {
    snap!(
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
        col_group_elements_without_end_tags
    )
}

#[test]
fn tr_td_th_elements_without_end_tags() -> Result {
    snap!(
        r#"<table>
  <tr>
    <th>One
    <th>Two
  <tr>
    <td>Three
    <td>Four
</table>"#,
        tr_td_th_elements_without_end_tags
    )
}

#[test]
fn named_entities_in_tag_content() -> Result {
    snap!(
        r#"<p>Lorem ipsum &nbsp; dolor sit &copy; amet.</p>"#,
        named_entities_in_tag_content
    )
}

#[test]
fn numeric_entities_in_tag_content() -> Result {
    snap!(
        r#"<p>Lorem ipsum &#160; dolor sit &#8212; amet.</p>"#,
        numeric_entities_in_tag_content
    )
}

#[test]
fn multiple_entities_in_tag_content() -> Result {
    snap!(
        r#"<p>Lorem ipsum &#xA0; dolor &#xa0; sit &nbsp; amet.</p>"#,
        multiple_entities_in_tag_content
    )
}

#[test]
fn omitted_end_tags() -> Result {
    snap!(r#"<!doctype html><html><head>"#, omitted_end_tags)
}
