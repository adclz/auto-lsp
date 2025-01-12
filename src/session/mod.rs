use std::collections::HashMap;

use crate::session::init::TextFn;
use auto_lsp_core::workspace::Workspace;
use init::InitOptions;
use lsp_server::{Connection, IoThreads};
use lsp_types::Url;

pub mod comment;
pub mod documents;
pub mod init;
pub mod lexer;
pub mod main_loop;
pub mod senders;
pub mod workspace;

pub struct Session {
    pub init_options: InitOptions,
    pub connection: Connection,
    pub io_threads: IoThreads,
    pub text_fn: TextFn,
    pub extensions: HashMap<String, String>,
    pub workspaces: HashMap<Url, Workspace>,
}
