use lsp_types::{Location, ReferenceParams};

use crate::server::session::{Session, WORKSPACE};

impl Session {
    /// Request to get references of a symbol
    ///
    /// To get the references, the server will look for the symbol at the given position,
    /// then read `get_referrers` from the symbol and return the references.
    pub fn get_references(
        &mut self,
        params: ReferenceParams,
    ) -> anyhow::Result<Option<Vec<Location>>> {
        let uri = &params.text_document_position.text_document.uri;

        let workspace = WORKSPACE.lock();

        let (root, document) = workspace
            .roots
            .get(uri)
            .ok_or(anyhow::anyhow!("Root not found"))?;

        let position = params.text_document_position.position;

        let offset = document.offset_at(position).unwrap();
        let item = root.descendant_at(offset);

        match item {
            Some(item) => match item.read().get_referrers().as_ref() {
                Some(item) => Ok(Some(
                    item.into_iter()
                        .filter_map(|reference| {
                            let reference = reference.to_dyn()?;
                            let reference = reference.read();
                            Some(Location::new(
                                (*reference.get_url()).clone(),
                                reference.get_lsp_range(document),
                            ))
                        })
                        .collect::<Vec<_>>(),
                )),
                None => Ok(None),
            },
            None => Ok(None),
        }
    }
}
