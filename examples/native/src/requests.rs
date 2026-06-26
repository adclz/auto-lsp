use auto_lsp::lsp_types::request::Request;

pub struct GetWorkspaceFiles {}

impl GetWorkspaceFiles {
    pub fn request(id: u32) -> String {
        format!(
            r#"{{"jsonrpc":"2.0","id":{id},"method":"custom/getWorkspaceFiles"}}"#
        )
    }
}

impl Request for GetWorkspaceFiles {
    type Params = ();
    type Result = Vec<String>;
    const METHOD: &'static str = "custom/getWorkspaceFiles";
}
