use lsp_types::{Location, ReferenceParams};

use crate::session::{Session, WORKSPACES};

impl Session {
    pub fn get_references(
        &mut self,
        params: ReferenceParams,
    ) -> anyhow::Result<Option<Vec<Location>>> {
        let uri = &params.text_document_position.text_document.uri;
        let workspace = WORKSPACES.lock();

        let workspace = workspace
            .get(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;
        let position = params.text_document_position.position;
        let doc = &workspace.document;

        let offset = doc.offset_at(position).unwrap();
        let item = workspace
            .ast
            .iter()
            .find_map(|symbol| symbol.read().find_at_offset(offset));

        match item {
            Some(item) => match item.read().get_referrers().as_ref() {
                Some(item) => Ok(Some(
                    item.into_iter()
                        .filter_map(|reference| {
                            let reference = reference.to_dyn()?;
                            let reference = reference.read();
                            Some(Location::new(
                                (*reference.get_url()).clone(),
                                reference.get_lsp_range(doc),
                            ))
                        })
                        .collect::<Vec<_>>(),
                )),
                None => return Ok(None),
            },
            None => Ok(None),
        }
    }
}
