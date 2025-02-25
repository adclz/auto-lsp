use crate::core::document::Document;
use crate::core::workspace::Workspace;
use lsp_types::Url;
use parking_lot::Mutex;
use rstest::{fixture, rstest};
use std::time::Duration;

use super::python_workspace::ast::Module;
use super::python_workspace::PYTHON_PARSERS;

#[fixture]
fn foo_bar() -> (Workspace, Document) {
    Workspace::from_utf8(
        PYTHON_PARSERS.get("python").unwrap(),
        Url::parse("file:///test.py").unwrap(),
        r#"# foo comment
def foo(param1, param2: int, param3: int = 5):
    pass

def bar():
    pass  
"#
        .into(),
    )
    .unwrap()
}

static DEADLOCK_DETECTION_LOCK: Mutex<()> = parking_lot::const_mutex(());

fn has_deadlock() -> bool {
    use parking_lot::deadlock::check_deadlock;
    !check_deadlock().is_empty()
}

#[rstest]
fn read_write(foo_bar: (Workspace, Document)) {
    let _guard = DEADLOCK_DETECTION_LOCK.lock();
    let ast = foo_bar.0.ast.unwrap();

    // not allowed in the same thread
    let _t = std::thread::spawn(move || {
        let _read = ast.read();
        let _write = ast.write();
    });

    std::thread::sleep(Duration::from_millis(50));

    assert!(has_deadlock());
}

#[rstest]
fn multiple_readers(foo_bar: (Workspace, Document)) {
    let _guard = DEADLOCK_DETECTION_LOCK.lock();
    let ast = foo_bar.0.ast.unwrap();
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
fn multiple_writers(foo_bar: (Workspace, Document)) {
    let _guard = DEADLOCK_DETECTION_LOCK.lock();
    let ast = foo_bar.0.ast.unwrap();
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
fn nested_writer(foo_bar: (Workspace, Document)) {
    let _guard = DEADLOCK_DETECTION_LOCK.lock();

    let ast = foo_bar.0.ast.unwrap();
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
