#![cfg_attr(feature = "unstable", feature(macro_at_most_once_rep))]

#[macro_use]
extern crate partial_function;

use partial_function::PartialFn;

#[derive(Debug)]
enum Test {
    Foo(i32),
    Bar(f32),
    Baz(bool),
}

macro_rules! test_partial_function (
    ($var:expr, $lhs:expr, None) => (
        assert!(!$var.is_defined_at($lhs) && $var.call($lhs).is_none())
    );
    ($var:expr, $lhs:expr, $rhs:expr) => (
        assert!($var.is_defined_at($lhs) && $var.call($lhs).is_some() && $var.call($lhs) == Some($rhs))
    );
);

#[cfg(test)]
#[test]
fn handles_basic_functionality() {
    let pf =
        partial_function! {
        "foo" => 1,
        "bar" => 2
    };

    test_partial_function!(pf, "foo", 1);
    test_partial_function!(pf, "bar", 2);
    test_partial_function!(pf, "baz", None);
}

#[test]
fn handles_variable_capture() {
    let pf =
        partial_function! {
        Test::Foo(a) => a,
        Test::Bar(_b) => 1
    };

    test_partial_function!(pf, Test::Foo(2), 2);
    test_partial_function!(pf, Test::Bar(1.0), 1);
    test_partial_function!(pf, Test::Baz(true), None);
}

#[test]
fn handles_non_local_variable_in_expression() {
    let c = 1;

    let pf =
        partial_function! {
        Test::Foo(a) => a + c
    };

    test_partial_function!(pf, Test::Foo(1), 2);
    test_partial_function!(pf, Test::Bar(1.0), None);
}

#[test]
fn handles_pattern_guards() {
    let c1 = 1;
    let c2 = 2;

    let pf =
        partial_function! {
        Test::Foo(a) if a == 1 => a + c1,
        Test::Foo(a) if a == 2 => a + c2
    };

    test_partial_function!(pf, Test::Foo(1), 2);
    test_partial_function!(pf, Test::Foo(2), 4);
    test_partial_function!(pf, Test::Foo(3), None);
    test_partial_function!(pf, Test::Bar(1.0), None);
}

#[test]
fn handles_pattern_alternation() {
    let pf =
        partial_function! {
        1 | 2 => "foo",
        3 => "bar"
    };

    test_partial_function!(pf, 1, "foo");
    test_partial_function!(pf, 2, "foo");
    test_partial_function!(pf, 3, "bar");
    test_partial_function!(pf, 4, None);
}

#[test]
fn handles_inclusive_range() {
    let pf =
        partial_function! {
        1...3 => "foo"
    };

    test_partial_function!(pf, 1, "foo");
    test_partial_function!(pf, 2, "foo");
    test_partial_function!(pf, 3, "foo");
    test_partial_function!(pf, 4, None);
}

#[test]
fn handles_pattern_binding() {
    let pf =
        partial_function! {
        a @ "foo" => a
    };

    test_partial_function!(pf, "foo", "foo");
    test_partial_function!(pf, "bar", None);
}

#[test]
fn handles_all_of_it_mixed_together() {
    let pf =
        partial_function! {
        a @ 1...10 | a @ 21...30 if a % 2 == 0 => a/2,
        a @ 11...20 | a @ 31...40 if a % 2 == 1 => a
    };

    test_partial_function!(pf, 1, None);
    test_partial_function!(pf, 2, 1);
    test_partial_function!(pf, 11, 11);
    test_partial_function!(pf, 12, None);
    test_partial_function!(pf, 21, None);
    test_partial_function!(pf, 22, 11);
    test_partial_function!(pf, 31, 31);
    test_partial_function!(pf, 32, None);
}
