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

use crate::db::create_json_db;
use auto_lsp::default::db::BaseDatabase;
use auto_lsp::default::db::tracked::get_ast;
use auto_lsp::lsp_types::Url;
use rstest::{fixture, rstest};

#[fixture]
fn valid_json() -> impl BaseDatabase {
    create_json_db(&[r#"{"key": "value", "number": 42}"#])
}

#[fixture]
fn missing_value() -> impl BaseDatabase {
    // Missing value after colon, tree-sitter inserts a MISSING value node
    create_json_db(&[r#"{"key": }"#])
}

#[fixture]
fn missing_pair_value() -> impl BaseDatabase {
    // Object with two pairs, second missing its value
    create_json_db(&[r#"{"a": 1, "b": }"#])
}

#[rstest]
fn valid_json_has_no_missing_nodes(valid_json: impl BaseDatabase) {
    let file = valid_json
        .get_file(&Url::parse("file:///test0.json").unwrap())
        .unwrap();
    let ast = get_ast(&valid_json, file);

    for node in ast.iter() {
        assert!(
            !node.is_missing(),
            "Node id={} should not be missing in valid JSON",
            node.get_id()
        );
    }
}

#[rstest]
fn missing_value_detected(missing_value: impl BaseDatabase) {
    let file = missing_value
        .get_file(&Url::parse("file:///test0.json").unwrap())
        .unwrap();
    let ast = get_ast(&missing_value, file);

    assert!(
        // at least one node is missing
        ast.iter().any(|n| n.is_missing()),
        "Expected MISSING nodes in `{{\"key\": }}`"
    );

    // MISSING node has a start_byte equals to end_byte
    for node in ast.iter().filter(|n| n.is_missing()) {
        let range = node.get_range();
        assert_eq!(
            range.start_byte,
            range.end_byte,
            "MISSING node id={} should be zero-width",
            node.get_id()
        );
    }
}

#[rstest]
fn missing_pair_value_detected(missing_pair_value: impl BaseDatabase) {
    let file = missing_pair_value
        .get_file(&Url::parse("file:///test0.json").unwrap())
        .unwrap();
    let ast = get_ast(&missing_pair_value, file);

    assert!(
        // at least one node is missing
        ast.iter().any(|n| n.is_missing()),
        "Expected MISSING nodes in `{{\"a\": 1, \"b\": }}`"
    );
}
