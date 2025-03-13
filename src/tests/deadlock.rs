use crate::core::workspace::Workspace;
use parking_lot::Mutex;
use rstest::{fixture, rstest};
use std::time::Duration;

use super::python_workspace::ast::Module;
use crate::tests::python_utils::{create_python_workspace, into_python_file};

#[fixture]
fn foo_bar() -> Workspace {
    create_python_workspace(
        r#"# foo comment
def foo(param1, param2: int, param3: int = 5):
    pass

def bar():
    pass  
"#,
    )
}

static DEADLOCK_DETECTION_LOCK: Mutex<()> = parking_lot::const_mutex(());

fn has_deadlock() -> bool {
    use parking_lot::deadlock::check_deadlock;
    !check_deadlock().is_empty()
}

#[rstest]
fn read_write(foo_bar: Workspace) {
    let (root, _) = into_python_file(foo_bar);
    let ast = root.ast.unwrap();

    let _guard = DEADLOCK_DETECTION_LOCK.lock();

    // not allowed in the same thread
    let _t = std::thread::spawn(move || {
        let _read = ast.read();
        let _write = ast.write();
    });

    std::thread::sleep(Duration::from_millis(50));

    assert!(has_deadlock());
}

#[rstest]
fn multiple_readers(foo_bar: Workspace) {
    let _guard = DEADLOCK_DETECTION_LOCK.lock();

    let (root, _) = into_python_file(foo_bar);
    let ast = root.ast.unwrap();
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
fn multiple_writers(foo_bar: Workspace) {
    let _guard = DEADLOCK_DETECTION_LOCK.lock();

    let (root, _) = into_python_file(foo_bar);
    let ast = root.ast.unwrap();

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
fn nested_writer(foo_bar: Workspace) {
    let _guard = DEADLOCK_DETECTION_LOCK.lock();

    let (root, _) = into_python_file(foo_bar);
    let ast = root.ast.unwrap();

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
