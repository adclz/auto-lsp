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

use super::ast::{Function, Module};
use crate::python::ast::Identifier;
use crate::{self as auto_lsp};
use auto_lsp::core::ast::BuildCompletionItems;
use auto_lsp_core::ast::BuildTriggeredCompletionItems;
use auto_lsp_core::document::Document;
use lsp_types::CompletionItem;
use std::sync::LazyLock;

/// Globally available completion items
static GLOBAL_COMPLETION_ITEMS: LazyLock<Vec<lsp_types::CompletionItem>> = LazyLock::new(|| {
    vec![lsp_types::CompletionItem {
        label: "def ...".to_string(),
        kind: Some(lsp_types::CompletionItemKind::SNIPPET),
        insert_text_format: Some(lsp_types::InsertTextFormat::SNIPPET),
        insert_text: Some("def ${1:func_name}(${2:arg1}):$0".to_string()),
        ..Default::default()
    }]
});

impl BuildCompletionItems for Module {
    fn build_completion_items(
        &self,
        _doc: &Document,
        acc: &mut Vec<lsp_types::CompletionItem>,
    ) -> anyhow::Result<()> {
        acc.extend(GLOBAL_COMPLETION_ITEMS.iter().cloned());
        Ok(())
    }
}

impl BuildCompletionItems for Function {
    fn build_completion_items(
        &self,
        _doc: &Document,
        acc: &mut Vec<lsp_types::CompletionItem>,
    ) -> anyhow::Result<()> {
        acc.extend(GLOBAL_COMPLETION_ITEMS.iter().cloned());
        Ok(())
    }
}

impl BuildTriggeredCompletionItems for Identifier {
    fn build_triggered_completion_items(
        &self,
        trigger: &str,
        _doc: &Document,
        acc: &mut Vec<CompletionItem>,
    ) -> anyhow::Result<()> {
        if trigger == "." {
            acc.push(CompletionItem {
                label: "triggered! ...".to_string(),
                kind: Some(lsp_types::CompletionItemKind::SNIPPET),
                insert_text_format: Some(lsp_types::InsertTextFormat::SNIPPET),
                insert_text: Some("def ${1:func_name}(${2:arg1}):$0".to_string()),
                ..Default::default()
            });
        };
        Ok(())
    }
}
