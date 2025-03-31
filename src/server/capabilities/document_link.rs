use crate::server::{session::Session, RegexToDocumentLink};
use auto_lsp_core::salsa::db::WorkspaceDatabase;
use lsp_types::{DocumentLink, DocumentLinkParams};
use std::ops::Deref;

/// Get document links for a document.
///
/// To find a document link, we need the comment [`tree_sitter::Query`] to find all comments,
/// then we use the regex from the [`crate::server::RegexToDocumentLink`] to find the links,
/// and finally we pass matches to the **to_document_link** function.
pub fn get_document_links<Db: WorkspaceDatabase>(
    db: &Db,
    with_regex: RegexToDocumentLink,
    params: DocumentLinkParams,
) -> anyhow::Result<Option<Vec<DocumentLink>>> {
    let re = &with_regex.regex;
    let to_document_link = &with_regex.to_document_link;
    let uri = params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let document = file.document(db).read();
    let root = file.get_ast(db).clone().into_inner();

    let mut results = vec![];
    let matches = root.find_all_with_regex(&document, re);
    matches.into_iter().for_each(|(m, line)| {
        to_document_link(m, line, &document, &root, &mut results);
    });

    Ok(Some(results))
}
