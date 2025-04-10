use crate::server::session::Session;
use auto_lsp_core::salsa::db::BaseDatabase;
use lsp_types::DidOpenTextDocumentParams;

pub fn open_text_document<Db: BaseDatabase>(
    session: &mut Session<Db>,
    params: DidOpenTextDocumentParams,
) -> anyhow::Result<()> {
    let url = &params.text_document.uri;

    if session.db.get_file(url).is_some() {
        // The file is already in db
        // We can ignore this change
        return Ok(());
    };

    let extension = &params.text_document.language_id;

    let extension = match session.extensions.get(extension) {
        Some(extension) => extension,
        None => {
            if session
                .extensions
                .values()
                .any(|x| x == extension)
            {
                extension
            } else {
                return Err(anyhow::format_err!(
                    "Extension {} is not registered, available extensions are: {:?}",
                    extension,
                    session.extensions
                ));
            }
        }
    };

    let text = (session.text_fn)(params.text_document.text.clone());

    let parsers = session
        .init_options
        .parsers
        .get(extension.as_str())
        .ok_or(anyhow::format_err!("No parser available for {}", extension))?;

    log::info!("Did Open Text Document: Created - {}", url.to_string());
    session.db.add_file_from_texter(parsers, url, text)
}
