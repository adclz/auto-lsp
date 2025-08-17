extern crate ast_python;
extern crate auto_lsp;

use ast_python::db::create_python_db;
use auto_lsp::{
    default::db::{tracked::get_ast, BaseDatabase},
    lsp_types::Url,
};

static DJANGO: &'static str = include_str!("./django.py");
static SOURCES: &'static [&'static str] = &[DJANGO];

fn main() {
    divan::main();
}

#[divan::bench]
fn build_ast() {
    let db = create_python_db(SOURCES);
    let file = db
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    get_ast(&db, file);
}
