#![cfg_attr(feature = "unstable", feature(fn_traits))]
#![cfg_attr(feature = "unstable", feature(macro_at_most_once_rep))]
#![cfg_attr(feature = "unstable", feature(unboxed_closures))]

pub struct PartialFn<'a, A, B> {
	__call_fn: Box<Fn(A) -> Option<B> + 'a>,
	__is_defined_at_fn: Box<Fn(A) -> bool + 'a>
}

impl<'a, A, B> PartialFn<'a, A, B> {
	pub fn call(&self, arg: A) -> Option<B> {
		(*self.__call_fn)(arg)
	}

	pub fn is_defined_at(&self, arg: A) -> bool {
		(*self.__is_defined_at_fn)(arg)
	}

	pub fn new(call: Box<Fn(A) -> Option<B> + 'a>, defined: Box<Fn(A) -> bool + 'a>) -> PartialFn<'a, A, B> {
		PartialFn {
			__call_fn: call,
			__is_defined_at_fn: defined
		}		
	}
}

#[cfg(feature = "unstable")]
impl<'a, A, B> Fn<(A,)> for PartialFn<'a, A, B> {
    extern "rust-call" fn call(&self, (arg,): (A,)) -> Self::Output {
        (*self.__call_fn)(arg)
    }
}

#[cfg(feature = "unstable")]
impl<'a, A, B> FnMut<(A,)> for PartialFn<'a, A, B> {
    extern "rust-call" fn call_mut(&mut self, (arg,): (A,)) -> Self::Output {
        (*self.__call_fn)(arg)
    }
}

#[cfg(feature = "unstable")]
impl<'a, A, B> FnOnce<(A,)> for PartialFn<'a, A, B> {
	type Output = Option<B>;

    extern "rust-call" fn call_once(self, (arg,): (A,)) -> Self::Output {
    	(*self.__call_fn)(arg)
    }
}

#[cfg(not(feature = "unstable"))]
#[macro_export]
macro_rules! __call_macro (
	($($($pat:pat)|+ $(if $cond:expr)* => $result:expr),*) => (
		|arg| {
			match arg {
				$(
					$($pat)|+ $(if $cond)* => Some($result)
				),*,
				_ => None
			}
		}
	);
);

#[cfg(not(feature = "unstable"))]
#[macro_export]
macro_rules! __is_defined_at_macro (
	($($($pat:pat)|+ $(if $cond:expr)* => $result:expr),*) => (
		#[allow(unused_variables)]
		|arg| {
			match arg {
				$(
					$($pat)|+ $(if $cond)* => true
				),*,
				_ => false
			}
		}
	);
);

#[cfg(not(feature = "unstable"))]
#[macro_export]
macro_rules! partial_function (
	($($($pat:pat)|+ $(if $cond:expr)* => $result:expr),*) => (
		PartialFn::new(Box::new(__call_macro!($($($pat)|+ $(if $cond)* => $result),*)), Box::new(__is_defined_at_macro!($($($pat)|+ $(if $cond)* => $result),*)))
	);
);

#[cfg(feature = "unstable")]
#[macro_export]
macro_rules! __call_macro (
	($($($pat:pat)|+ $(if $cond:expr)? => $result:expr),*) => (
		|arg| {
			match arg {
				$(
					$($pat)|+ $(if $cond)? => Some($result)
				),*,
				_ => None
			}
		}
	);
);

#[cfg(feature = "unstable")]
#[macro_export]
macro_rules! __is_defined_at_macro (
	($($($pat:pat)|+ $(if $cond:expr)? => $result:expr),*) => (
		#[allow(unused_variables)]
		|arg| {
			match arg {
				$(
					$($pat)|+ $(if $cond)? => true
				),*,
				_ => false
			}
		}
	);
);

#[cfg(feature = "unstable")]
#[macro_export]
macro_rules! partial_function (
	($($($pat:pat)|+ $(if $cond:expr)? => $result:expr),*) => (
		PartialFn::new(Box::new(__call_macro!($($($pat)|+ $(if $cond)? => $result),*)), Box::new(__is_defined_at_macro!($($($pat)|+ $(if $cond)? => $result),*)))
	);
);

fn handles_pattern_guards() {
	let c1 = 1;
	let c2 = 2;

	let pf : PartialFn<i32, i32> = partial_function! {
		a if a == 1 => a + c1,
		a if a == 2 => a + c2
	};
}
