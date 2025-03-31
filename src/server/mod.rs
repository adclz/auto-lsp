//! # Server module
//!
//! This module is available when the `lsp_server` feature is enabled.
//!
//! Configuring the server involves having:
//! - The tree_sitter language and node_types paths (which should be present in any rust bindings).
//! - The core [`tree_sitter::Query`] to build the AST.
//! - A root symbol created with either [`crate::seq`] or [`crate::choice`] macro.
//!
//! ## Minimal example
//! The first step is to configure parsers using the [crate::configure_parsers] macro.
//!
//! ```rust
//! # use auto_lsp::configure_parsers;
//! # use auto_lsp::core::ast::*;
//! # use auto_lsp::seq;
//! static CORE_QUERY: &'static str = "(module) @module";
//!
//! #[seq(query = "module")]
//! struct Module {}
//!
//! configure_parsers!(
//!     PARSER_LIST,
//!     "python" => {
//!         language: tree_sitter_python::LANGUAGE,
//!         node_types: tree_sitter_python::NODE_TYPES,
//!         ast_root: Module,
//!         core: CORE_QUERY,
//!         // optional
//!         comment: None,
//!         fold: None,
//!         highlights: None
//!     }
//! );
//! ```
//!
//!  Next, define the server's capabilities.
//!
//!  ```rust
//! # use auto_lsp::configure_parsers;
//! # use auto_lsp::core::ast::*;
//! # use auto_lsp::seq;
//! # static CORE_QUERY: &'static str = "(module) @module";
//! # #[seq(query = "module")]
//! # struct Module {}
//! # configure_parsers!(
//! #     PARSER_LIST,
//! #    "python" => {
//! #        language: tree_sitter_python::LANGUAGE,
//! #        node_types: tree_sitter_python::NODE_TYPES,
//! #        ast_root: Module,
//! #        core: CORE_QUERY,
//! #        // optional
//! #        comment: None,
//! #        fold: None,
//! #        highlights: None
//! #    }
//! # );
//!  use auto_lsp::server::{InitOptions, LspOptions};
//!  use std::error::Error;
//!
//!  let init_options = InitOptions {
//!     parsers: &PARSER_LIST,
//!     lsp_options: LspOptions {
//!         ..Default::default()
//!     }
//!  };
//!  ```
//!
//!  Finally, create your main function and initialize a new session.
//!
//! ```no_run
//! # use auto_lsp::python::PYTHON_PARSERS;
//! use std::error::Error;
//! use auto_lsp::server::{Session, InitOptions, LspOptions};
//! use auto_lsp::lsp_server::Connection;
//!
//! fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
//!   let (connection, io_threads) = Connection::stdio();
//!
//!   let init_options = InitOptions {
//!         parsers: &PYTHON_PARSERS,
//!         lsp_options: LspOptions {
//!             ..Default::default()
//!         }
//!    };
//!    let mut session = Session::create(init_options, connection)?;
//!
//!    session.main_loop()?;
//!    io_threads.join()?;
//!    Ok(())
//! }
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
