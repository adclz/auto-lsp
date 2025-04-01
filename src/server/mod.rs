//! # Server module
//!
//! This module is available when the `lsp_server` feature is enabled.
//! ```
//!

/// LSP server capabilities (executed when receiving requests or notifications from client)
pub mod capabilities;
/// Session handling
mod session;

pub use session::notification_registry::NotificationRegistry;
pub use session::options::*;
pub use session::request_registry::RequestRegistry;
pub use session::Session;
