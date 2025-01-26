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
//! The client is responsible for specifying how different file extensions are linked to specific parsers.
//!  
//! ```rust
//! # use auto_lsp::configure_parsers;
//! # use auto_lsp::core::ast::*;
//! # use auto_lsp::seq;
//! static CORE_QUERY: &'static str = "(module) @module";
//!
//! #[seq(query_name = "module", kind(symbol()))]
//! struct Module {}
//!
//! configure_parsers!(
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
//!  Next, define the server's capabilities using the [crate::server::capabilities] module.
//!  
//!  [`crate::server::InitOptions`]  has only one mandatory field, `parsers`, which is a map of file extensions to parsers previously created.
//!  ```rust
//! # use auto_lsp::configure_parsers;
//! # use auto_lsp::core::ast::*;
//! # use auto_lsp::seq;
//! # static CORE_QUERY: &'static str = "(module) @module";
//! # #[seq(query_name = "module", kind(symbol()))]
//! # struct Module {}
//! # configure_parsers!(
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
//!     parsers: &PARSERS,
//!     lsp_options: LspOptions {
//!         ..Default::default()
//!     }
//!  };
//!  ```
//!
//!  Finally, create your main function and initialize a new session..
//!
//! ```no_run
//! # use auto_lsp::python::PARSERS;
//! use std::error::Error;
//! use auto_lsp::server::{Session, InitOptions, LspOptions};
//!
//! fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
//!   let init_options = InitOptions {
//!         parsers: &PARSERS,
//!         lsp_options: LspOptions {
//!             ..Default::default()
//!         }
//!    };
//!    let mut session = Session::create(init_options)?;
//!
//!    session.main_loop()?;
//!    session.io_threads.join()?;
//!    Ok(())
//! }
//! ```
//!

/// LSP server capabilities (executed when receiving requests or notifications from client)
pub(crate) mod capabilities;
/// Session handling
mod session;

pub use session::init::*;
pub use session::Session;
