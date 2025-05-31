/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/

use std::collections::HashMap;

use auto_lsp_core::parsers::Parsers;
use lsp_types::{
    ServerCapabilities, ServerInfo,
};

/// Initialization options for the LSP server
pub struct InitOptions {
    pub parsers: &'static HashMap<&'static str, Parsers>,
    pub capabilities: ServerCapabilities,
    pub server_info: Option<ServerInfo>,
}
