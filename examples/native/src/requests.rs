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
