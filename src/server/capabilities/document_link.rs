use crate::server::RegexToDocumentLink;
use auto_lsp_core::{
    regex::find_all_with_regex,
    salsa::db::BaseDatabase,
};
use lsp_types::{DocumentLink, DocumentLinkParams};

/// Get document links for a document.
///
/// To find a document link, we need the comment [`tree_sitter::Query`] to find all comments,
/// then we use the regex from the [`crate::server::RegexToDocumentLink`] to find the links,
/// and finally we pass matches to the **to_document_link** function.
pub fn get_document_links<Db: BaseDatabase>(
    db: &Db,
    query: &tree_sitter::Query,
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

    let mut results = vec![];
    let matches = find_all_with_regex(query, &document, re);
    matches.into_iter().for_each(|(m, line)| {
        to_document_link(m, line, &document, &mut results);
    });

    Ok(Some(results))
}
