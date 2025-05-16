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
/// and finally, we pass matches to the **to_document_link** function.
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
