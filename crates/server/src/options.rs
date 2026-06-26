use lsp_types::{ServerCapabilities, ServerInfo};

/// Initialization options for the LSP server
pub struct InitOptions {
    pub capabilities: ServerCapabilities,
    pub server_info: Option<ServerInfo>,
}
