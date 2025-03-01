use super::document::Document;
use super::root::Root;
use lsp_types::Url;
use std::collections::HashMap;

#[derive(Default)]
pub struct Workspace {
    pub roots: HashMap<Url, (Root, Document)>,
}
