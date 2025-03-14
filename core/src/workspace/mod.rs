use super::document::Document;
use super::root::Root;
use lsp_types::Url;
use std::collections::HashMap;

/// Represents a workspace that maps [lsp_types::Url] to a tuple containing a [Root] and a [Document].
///
/// This is the top-level structure
#[derive(Default)]
pub struct Workspace {
    pub roots: HashMap<Url, (Root, Document)>,
}
