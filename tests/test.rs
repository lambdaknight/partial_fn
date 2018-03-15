#![feature(macro_at_most_once_rep)]

#[macro_use] extern crate partial_function;

use partial_function::{PartialFn, PartialFnError};

#[derive(Debug)]
enum Test {
	Foo(i32),
	Bar(f32),
	Baz(bool)
}

#[cfg(test)]

#[test]
fn handles_basic_functionality() {
	let pf = partial_function! {
		"foo" => 1,
		"bar" => 2
	};

	assert_eq!(pf.is_defined_at("foo"), true);
	assert_eq!(pf.is_defined_at("bar"), true);
	assert_eq!(pf.is_defined_at("baz"), false);
	assert!(pf("foo").is_ok() && pf("foo").ok() == Some(1));
	assert!(pf("bar").is_ok() && pf("bar").ok() == Some(2));
	assert!(pf("baz").is_err());
}

#[test]
fn handles_variable_capture() {
	let pf = partial_function! {
		Test::Foo(a) => a,
		Test::Bar(b) => 1
	};

	assert_eq!(pf.is_defined_at(Test::Foo(2)), true);
	assert_eq!(pf.is_defined_at(Test::Bar(1.0)), true);
	assert_eq!(pf.is_defined_at(Test::Baz(true)), false);
	assert!(pf(Test::Foo(2)).is_ok() && pf(Test::Foo(2)).ok() == Some(2));
	assert!(pf(Test::Bar(1.0)).is_ok() && pf(Test::Bar(1.0)).ok() == Some(1));
	assert!(pf(Test::Baz(true)).is_err());
}

#[test]
fn handles_non_local_variable_in_expression() {
	let c = 1;

	let pf = partial_function! {
		Test::Foo(a) => a + c
	};

	assert_eq!(pf.is_defined_at(Test::Foo(1)), true);
	assert_eq!(pf.is_defined_at(Test::Bar(1.0)), false);
	assert!(pf(Test::Foo(1)).is_ok() && pf(Test::Foo(1)).ok() == Some(2));
	assert!(pf(Test::Bar(1.0)).is_err());
}

#[test]
fn testTest() {
	let c1 = 1;
	let c2 = 2;

// 	let pf = partial_function! {
// 		Test::Foo(a) if a == 1 => a + c1,
// 		Test::Foo(a) if a == 2 => a + c2
// 	};

    let pf =
        PartialFn::new(Box::new(|arg|
                                    {
                                        match arg {
                                            Test::Foo(a) if a == 1 => Ok(a + c1),
                                            Test::Foo(a) if a == 2 => Ok(a + c2),
                                            _ =>
                                            Err(PartialFnError::MatchError(arg)),
                                        }
                                    }),
                       Box::new(|arg|
                                    {
                                        match arg {
                                            Test::Foo(a) if a == 1 => true,
                                            Test::Foo(a) if a == 2 => true,
                                            _ => false,
                                        }
                                    }));

	assert_eq!(pf.is_defined_at(Test::Foo(1)), true);
	assert_eq!(pf.is_defined_at(Test::Foo(2)), true);
	assert_eq!(pf.is_defined_at(Test::Foo(3)), false);
	assert_eq!(pf.is_defined_at(Test::Bar(1.0)), false);
	assert!(pf(Test::Foo(1)).is_ok() && pf(Test::Foo(1)).ok() == Some(2));
	assert!(pf(Test::Foo(2)).is_ok() && pf(Test::Foo(2)).ok() == Some(4));
	assert!(pf(Test::Bar(1.0)).is_err());
}

#[test]
fn handles_pattern_guards() {
	let c1 = 1;
	let c2 = 2;

	let pf = partial_function! {
		Test::Foo(a) if a == 1 => a + c1,
		Test::Foo(a) if a == 2 => a + c2
	};

	assert_eq!(pf.is_defined_at(Test::Foo(1)), true);
	assert_eq!(pf.is_defined_at(Test::Foo(2)), true);
	assert_eq!(pf.is_defined_at(Test::Foo(3)), false);
	assert_eq!(pf.is_defined_at(Test::Bar(1.0)), false);
	assert!(pf(Test::Foo(1)).is_ok() && pf(Test::Foo(1)).ok() == Some(2));
	assert!(pf(Test::Foo(2)).is_ok() && pf(Test::Foo(2)).ok() == Some(4));
	assert!(pf(Test::Bar(1.0)).is_err());
}

#[test]
fn handles_pattern_alternation() {
	let c1 = 1;
	let c2 = 2;

	let pf = partial_function! {
		1 | 2 => "foo",
		3 => "bar"
	};

	assert_eq!(pf.is_defined_at(1), true);
	assert_eq!(pf.is_defined_at(2), true);
	assert_eq!(pf.is_defined_at(3), true);
	assert_eq!(pf.is_defined_at(4), false);

	assert!(pf(1).is_ok() && pf(1).ok() == Some("foo"));
	assert!(pf(2).is_ok() && pf(2).ok() == Some("foo"));
	assert!(pf(3).is_ok() && pf(3).ok() == Some("bar"));
	assert!(pf(4).is_err());
}

#[test]
fn handles_inclusive_range() {
	let pf = partial_function! {
		1...3 => "foo"
	};

	assert_eq!(pf.is_defined_at(1), true);
	assert_eq!(pf.is_defined_at(2), true);
	assert_eq!(pf.is_defined_at(3), true);
	assert_eq!(pf.is_defined_at(4), false);
	assert!(pf(1).is_ok() && pf(1).ok() == Some("foo"));
	assert!(pf(4).is_err());
}

#[test]
fn handles_pattern_binding() {
	let pf = partial_function! {
		a @ "foo" => a
	};

	assert_eq!(pf.is_defined_at("foo"), true);
	assert_eq!(pf.is_defined_at("bar"), false);
	assert!(pf("foo").is_ok() && pf("foo").ok() == Some("foo"));
	assert!(pf("bar").is_err());
}