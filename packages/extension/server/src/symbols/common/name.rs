use auto_lsp::traits::ast_item::AstItem;
use auto_lsp::traits::ast_item_builder::AstItemBuilder;
use auto_lsp_macros::{ast, ast_enum, ast_struct};
use std::{
    cell::RefCell,
    fmt::Debug,
    rc::Rc,
    sync::{Arc, RwLock},
};

use crate::symbols::pous::{
    function::Function,
    variables::{InputVariable, OutputVariable},
};

#[ast_struct(
    query_name = "name", 
    features(
        lsp_hover(call = self::get_name_hover),
    )
)]
pub struct Name {}

impl Name {
    fn get_name_hover(&self, doc: &lsp_textdocument::FullTextDocument) -> Option<String> {
        match self.parent {
            Some(ref parent) => {
                let parent = parent.read().unwrap();
                if parent.is::<Function>() {
                    return Some(format!(
                        r#"
```typescript
function {}
```"#,
                        self.get_text(doc.get_content(None).as_bytes())
                    ));
                } else if parent.is::<InputVariable>() {
                    return Some(format!(
                        r#"
```typescript
var {}
```"#,
                        self.get_text(doc.get_content(None).as_bytes())
                    ));
                } else if parent.is::<OutputVariable>() {
                    return Some(format!(
                        r#"
```typescript
var {}
```"#,
                        self.get_text(doc.get_content(None).as_bytes())
                    ));
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
