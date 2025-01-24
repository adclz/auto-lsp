use crate::{self as auto_lsp};
use auto_lsp::{choice, seq};
use auto_lsp_core::ast::{AstSymbol, Symbol};
use auto_lsp_core::build::{CheckQueryable, Queryable};
use downcast_rs::Downcast;
use static_assertions::{assert_fields, assert_impl_all};

#[test]
fn simple_seq() {
    #[seq(query_name = "module", kind(symbol()))]
    struct Module {}

    assert_impl_all!(Module: Send, Sync, Clone, Downcast, AstSymbol);
    assert_fields!(Module: _data);
    assert_eq!(Module::QUERY_NAMES[0], "module");
    assert_eq!(Module::CHECK, ());
}

#[test]
fn seq_with_field() {
    #[seq(query_name = "module", kind(symbol()))]
    struct Module {
        function: Function,
    }

    #[seq(query_name = "function", kind(symbol()))]
    struct Function {}

    assert_impl_all!(Module: Send, Sync, Clone, Downcast, AstSymbol);
    assert_fields!(Module: _data, function);
    assert_eq!(Module::QUERY_NAMES.len(), 1);
    assert_eq!(Module::QUERY_NAMES[0], "module");
    assert_eq!(Module::CHECK, ());
}

#[test]
fn simple_choice() {
    #[choice]
    enum Choice {
        A(A),
    }

    #[seq(query_name = "module", kind(symbol()))]
    struct A {}

    assert_impl_all!(Choice: Send, Sync, Clone, Downcast, AstSymbol);
    assert_eq!(Choice::QUERY_NAMES.len(), 1);
    assert_eq!(Choice::QUERY_NAMES[0], "module");
    assert_eq!(Choice::CHECK, ());
}

#[test]
fn multiple_choices() {
    #[choice]
    enum Choice {
        A(A),
        B(B),
    }

    #[seq(query_name = "module1", kind(symbol()))]
    struct A {}

    #[seq(query_name = "module2", kind(symbol()))]
    struct B {}

    assert_impl_all!(Choice: Send, Sync, Clone, Downcast, AstSymbol);
    assert_eq!(Choice::QUERY_NAMES.len(), 2);
    assert_eq!(Choice::QUERY_NAMES[0], "module1");
    assert_eq!(Choice::QUERY_NAMES[1], "module2");
    assert_eq!(Choice::CHECK, ());
}

#[test]
fn seq_with_optional() {
    #[seq(query_name = "module", kind(symbol()))]
    struct Module {
        function: Option<Function>,
    }

    #[seq(query_name = "function", kind(symbol()))]
    struct Function {}

    assert_impl_all!(Module: Send, Sync, Clone, Downcast, AstSymbol);
    assert_fields!(Module: _data, function);
    assert_eq!(Module::QUERY_NAMES.len(), 1);
    assert_eq!(Module::QUERY_NAMES[0], "module");
    assert_eq!(Module::CHECK, ());
}

#[test]
fn seq_with_recursive() {
    #[seq(query_name = "module1", kind(symbol()))]
    struct A {
        elems: Vec<A>,
    }

    assert_impl_all!(A: Send, Sync, Clone, Downcast, AstSymbol);
    assert_fields!(A: _data, elems);
    assert_eq!(A::QUERY_NAMES.len(), 1);
    assert_eq!(A::QUERY_NAMES[0], "module1");
    assert_eq!(A::CHECK, ());
}
