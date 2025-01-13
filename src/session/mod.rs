use std::{collections::HashMap, sync::LazyLock};

use crate::session::init::TextFn;
use auto_lsp_core::workspace::Workspace;
use init::InitOptions;
use lsp_server::{Connection, IoThreads};
use lsp_types::Url;
use parking_lot::Mutex;

pub mod comment;
pub mod documents;
pub mod init;
pub mod lexer;
pub mod main_loop;
pub mod senders;
pub mod workspace;

pub static WORKSPACES: LazyLock<Mutex<HashMap<Url, Workspace>>> = LazyLock::new(Mutex::default);

pub struct Session {
    pub init_options: InitOptions,
    pub connection: Connection,
    pub io_threads: IoThreads,
    pub text_fn: TextFn,
    pub extensions: HashMap<String, String>,
}
