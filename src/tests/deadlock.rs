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

use auto_lsp_core::{salsa::db::BaseDatabase, salsa::tracked::get_ast};
use lsp_types::Url;
use parking_lot::Mutex;
use rstest::{fixture, rstest};
use std::time::Duration;

use super::python_utils::create_python_db;
use super::python_workspace::ast::Module;

#[fixture]
fn foo_bar() -> impl BaseDatabase {
    create_python_db(&[r#"# foo comment
def foo(param1, param2: int, param3: int = 5):
    pass

def bar():
    pass  
"#])
}

static DEADLOCK_DETECTION_LOCK: Mutex<()> = parking_lot::const_mutex(());

fn has_deadlock() -> bool {
    use parking_lot::deadlock::check_deadlock;
    !check_deadlock().is_empty()
}

#[rstest]
fn read_write(foo_bar: impl BaseDatabase) {
    let _guard = DEADLOCK_DETECTION_LOCK.lock();

    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    let root = get_ast(&foo_bar, file).to_symbol();

    let ast = root.unwrap().clone();

    // not allowed in the same thread
    let _t = std::thread::spawn(move || {
        let _read = ast.read();
        let _write = ast.write();
    });

    std::thread::sleep(Duration::from_millis(50));

    assert!(has_deadlock());
}

#[rstest]
fn multiple_readers(foo_bar: impl BaseDatabase) {
    let _guard = DEADLOCK_DETECTION_LOCK.lock();

    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    let root = get_ast(&foo_bar, file).to_symbol();

    let ast = root.unwrap().clone();
    let ast_clone = ast.clone();

    let _t1 = std::thread::spawn(move || {
        let _read = ast.read();
        let _read2 = ast.read();
        std::thread::sleep(std::time::Duration::from_secs(10));
    });

    let _t2 = std::thread::spawn(move || {
        let _read = ast_clone.read();
        std::thread::sleep(std::time::Duration::from_secs(10));
    });

    std::thread::sleep(Duration::from_millis(50));
    assert!(!has_deadlock());
}

#[rstest]
fn multiple_writers(foo_bar: impl BaseDatabase) {
    let _guard = DEADLOCK_DETECTION_LOCK.lock();

    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    let root = get_ast(&foo_bar, file).to_symbol();

    let ast = root.unwrap().clone();
    let ast_clone = ast.clone();

    let _t1 = std::thread::spawn(move || {
        let _write = ast.write();
        std::thread::sleep(std::time::Duration::from_secs(10));
    });

    let _t2 = std::thread::spawn(move || {
        let _write = ast_clone.write();
        std::thread::sleep(std::time::Duration::from_secs(10));
    });

    std::thread::sleep(Duration::from_millis(50));
    assert!(!has_deadlock());
}

#[rstest]
fn nested_writer(foo_bar: impl BaseDatabase) {
    let _guard = DEADLOCK_DETECTION_LOCK.lock();

    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();

    let root = get_ast(&foo_bar, file).to_symbol();
    let ast = root.unwrap().clone();

    let ast_clone = ast.clone();

    let _t1 = std::thread::spawn(move || {
        let _read = ast.read();
        std::thread::sleep(std::time::Duration::from_secs(10));
    });

    std::thread::sleep(Duration::from_millis(50));
    assert!(!has_deadlock());

    let _t2 = std::thread::spawn(move || {
        let _read = ast_clone.read();
        let _module = _read.downcast_ref::<Module>().unwrap();
        let _write = _module.statements[0].write();
        std::thread::sleep(std::time::Duration::from_secs(10));
    });

    std::thread::sleep(Duration::from_millis(50));
    assert!(!has_deadlock());
}
