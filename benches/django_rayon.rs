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

use auto_lsp_core::salsa::{
    db::{BaseDatabase, BaseDb},
    tracked::get_ast,
};
use criterion::{criterion_group, criterion_main, Criterion};
use lsp_types::Url;
use texter::core::text;

pub static DJANGO: &str = include_str!("django.py");

pub fn parse_rayon(c: &mut Criterion) {
    let mut db = BaseDb::default();

    let uri = Url::parse("file:///test.py").unwrap();
    let text = text::Text::new(DJANGO.to_string());
    db.add_file_from_texter(
        auto_lsp::python::PYTHON_PARSERS.get("python").unwrap(),
        &uri,
        text,
    )
    .unwrap();

    c.bench_function("parse_django_file_with_rayon", move |b| {
        b.iter(|| {
            let file = db.get_file(&uri).unwrap();
            let ast = get_ast(&db, file).get_root();
            assert!(ast.is_some())
        });
    });
}
criterion_group!(benches, parse_rayon);
criterion_main!(benches);
