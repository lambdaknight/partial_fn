#![feature(macro_at_most_once_rep)]
#![feature(fn_traits)]
#![feature(overloaded_calls)]
#![feature(unboxed_closures)]

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

impl<'a, A, B> Fn<(A,)> for PartialFn<'a, A, B> {
    extern "rust-call" fn call(&self, (arg,): (A,)) -> Self::Output {
        (*self.__call_fn)(arg)
    }
}

impl<'a, A, B> FnMut<(A,)> for PartialFn<'a, A, B> {
    extern "rust-call" fn call_mut(&mut self, (arg,): (A,)) -> Self::Output {
        (*self.__call_fn)(arg)
    }
}

impl<'a, A, B> FnOnce<(A,)> for PartialFn<'a, A, B> {
	type Output = Option<B>;

    extern "rust-call" fn call_once(self, (arg,): (A,)) -> Self::Output {
    	(*self.__call_fn)(arg)
    }
}

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

#[macro_export]
macro_rules! __is_defined_at_macro (
	($($($pat:pat)|+ $(if $cond:expr)? => $result:expr),*) => (
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

#[macro_export]
macro_rules! partial_function (
	($($($pat:pat)|+ $(if $cond:expr)? => $result:expr),*) => (
		PartialFn::new(Box::new(__call_macro!($($($pat)|+ $(if $cond)? => $result),*)), Box::new(__is_defined_at_macro!($($($pat)|+ $(if $cond)? => $result),*)))
	);
);