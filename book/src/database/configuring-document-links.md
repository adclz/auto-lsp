# Configuring Document Links

[Document links](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_documentLink) are declared outside the AST.

`auto-lsp` enables finding document links by running a regular expression on the comments.

## Example

```rust, ignore
// Create a document or use an existing one

let (root, document) = Root::from_utf8(
    &PARSER_LIST.get("HTML").unwrap(),
    Url::parse("file://index.html").unwrap(),
    r#"<!DOCTYPE html>
<!-- source:file1.txt:52 -->
<div>
    <!-- source:file2.txt:25 -->
</div>"#
        .into()
).unwrap();

let regex = Regex::new(r" source:(\w+\.\w+):(\d+)").unwrap();
let results = root.find_all_with_regex(&document, &regex);

assert_eq!(results.len(), 2);
```
