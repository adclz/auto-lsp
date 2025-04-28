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

use crate::{self as auto_lsp};
use auto_lsp::{choice, seq};
use auto_lsp_core::ast::AstSymbol;
use auto_lsp_core::build::{Buildable, InvokeParser, Queryable};
use auto_lsp_core::document::Document;
use auto_lsp_core::parsers::Parsers;
use downcast_rs::Downcast;
use impls::impls;
use static_assertions::{assert_fields, assert_impl_all};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;

#[test]
fn simple_seq() {
    #[seq(query = "module")]
    struct Module {}

    assert_impl_all!(Module: Send, Sync, Clone, Display, Downcast, AstSymbol);
    assert!(
        impls!(Module: TryFrom<(&'static ModuleBuilder, &'static Option<usize>, &'static Document, &'static Parsers, &'static HashMap<usize, usize>, &'static mut Vec<Arc<dyn AstSymbol>>)>)
    );
    assert!(impls!(Module: InvokeParser<ModuleBuilder, Module>));
    assert_fields!(Module: _data);

    assert_impl_all!(ModuleBuilder: Queryable, Buildable);
    assert_fields!(ModuleBuilder: query_index, range);
    assert_eq!(ModuleBuilder::QUERY_NAMES.len(), 1);
    assert_eq!(ModuleBuilder::QUERY_NAMES[0], "module");
}

#[test]
fn seq_with_field() {
    #[seq(query = "module")]
    struct Module {
        function: Function,
    }

    #[seq(query = "function")]
    struct Function {}

    assert_impl_all!(Module: Send, Sync, Clone, Display, Downcast, AstSymbol);
    assert!(
        impls!(Module: TryFrom<(&'static ModuleBuilder, &'static Option<usize>, &'static Document, &'static Parsers, &'static HashMap<usize, usize>, &'static mut Vec<Arc<dyn AstSymbol>>)>)
    );
    assert!(impls!(Module: InvokeParser<ModuleBuilder, Module>));
    assert_fields!(Module: _data, function);

    assert_impl_all!(ModuleBuilder: Queryable, Buildable);
    assert_fields!(ModuleBuilder: query_index, range);
    assert_eq!(ModuleBuilder::QUERY_NAMES.len(), 1);
    assert_eq!(ModuleBuilder::QUERY_NAMES[0], "module");
}

#[test]
fn simple_choice() {
    #[choice]
    enum Choice {
        A(A),
    }

    #[seq(query = "module")]
    struct A {}

    assert_impl_all!(Choice: Send, Sync, Clone, Display, Downcast, AstSymbol);
    assert!(
        impls!(Choice: TryFrom<(&'static ChoiceBuilder, &'static Option<usize>, &'static Document, &'static Parsers, &'static HashMap<usize, usize>, &'static mut Vec<Arc<dyn AstSymbol>>)>)
    );
    assert!(impls!(Choice: InvokeParser<ChoiceBuilder, Choice>));

    assert_impl_all!(ChoiceBuilder: Queryable, Buildable);
    assert_fields!(ChoiceBuilder: unique_field);
    assert_eq!(ChoiceBuilder::QUERY_NAMES.len(), 1);
    assert_eq!(ChoiceBuilder::QUERY_NAMES[0], "module");
}

#[test]
fn multiple_choices() {
    #[choice]
    enum Choice {
        A(A),
        B(B),
    }

    #[seq(query = "module1")]
    struct A {}

    #[seq(query = "module2")]
    struct B {}

    assert_impl_all!(Choice: Send, Sync, Clone, Display, Downcast, AstSymbol);
    assert!(
        impls!(Choice: TryFrom<(&'static ChoiceBuilder, &'static Option<usize>, &'static Document, &'static Parsers, &'static HashMap<usize, usize>, &'static mut Vec<Arc<dyn AstSymbol>>)>)
    );
    assert!(impls!(Choice: InvokeParser<ChoiceBuilder, Choice>));

    assert_impl_all!(ChoiceBuilder: Queryable, Buildable);
    assert_fields!(ChoiceBuilder: unique_field);
    assert_eq!(ChoiceBuilder::QUERY_NAMES.len(), 2);
    assert_eq!(ChoiceBuilder::QUERY_NAMES[0], "module1");
    assert_eq!(ChoiceBuilder::QUERY_NAMES[1], "module2");
}

#[test]
fn seq_with_optional() {
    #[seq(query = "module")]
    struct Module {
        function: Option<Function>,
    }

    #[seq(query = "function")]
    struct Function {}

    assert_impl_all!(Module: Send, Sync, Clone, Display, Downcast, AstSymbol);
    assert!(
        impls!(Module: TryFrom<(&'static ModuleBuilder, &'static Option<usize>, &'static Document, &'static Parsers, &'static HashMap<usize, usize>, &'static mut Vec<Arc<dyn AstSymbol>>)>)
    );
    assert!(impls!(Module: InvokeParser<ModuleBuilder, Module>));
    assert_fields!(Module: _data, function);

    assert_impl_all!(ModuleBuilder: Queryable, Buildable);
    assert_fields!(ModuleBuilder: query_index, range);
    assert_eq!(ModuleBuilder::QUERY_NAMES.len(), 1);
    assert_eq!(ModuleBuilder::QUERY_NAMES[0], "module");
}

#[test]
fn seq_with_recursive() {
    #[seq(query = "module1")]
    struct A {
        elems: Vec<A>,
    }

    assert_impl_all!(A: Send, Sync, Clone, Display, Downcast, AstSymbol);
    assert!(
        impls!(A: TryFrom<(&'static ABuilder, &'static Option<usize>, &'static Document, &'static Parsers, &'static HashMap<usize, usize>, &'static mut Vec<Arc<dyn AstSymbol>>)>)
    );
    assert!(impls!(A: InvokeParser<ABuilder, A>));
    assert_fields!(A: _data, elems);

    assert_impl_all!(ABuilder: Queryable, Buildable);
    assert_fields!(ABuilder: query_index, range);
    assert_eq!(ABuilder::QUERY_NAMES.len(), 1);
    assert_eq!(ABuilder::QUERY_NAMES[0], "module1");
}
