use auto_lsp::lsp_types::request::Request;

#[cfg(test)]
pub mod tests;

pub struct GetWorkspaceFiles {}

impl GetWorkspaceFiles {
    pub const REQUEST: &'static str =
        &r#"{"jsonrpc":"2.0","id":2,"method":"custom/getWorkspaceFiles"}"#;
}

impl Request for GetWorkspaceFiles {
    type Params = ();
    type Result = Vec<String>;
    const METHOD: &'static str = "custom/getWorkspaceFiles";
}
