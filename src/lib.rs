#![feature(macro_at_most_once_rep)]
#![feature(fn_traits)]
#![feature(overloaded_calls)]
#![feature(unboxed_closures)]

pub enum PartialFnError<A> {
	MatchError(A)
}

impl<A> std::fmt::Debug for PartialFnError<A> where A: std::fmt::Debug {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		match *self {
			PartialFnError::MatchError(ref value) => write!(f, "MatchError: {:?}", value)
		}
	}
}

impl<A> std::fmt::Display for PartialFnError<A> where A: std::fmt::Debug {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			PartialFnError::MatchError(ref value) => write!(f, "MatchError: {:?}", value),
		}
	}
}

impl<A> std::error::Error for PartialFnError<A> where A: std::fmt::Debug {
	fn description(&self) -> &str {
        "partial function not defined at value"
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

pub struct PartialFn<'a, A, B> {
	__call_fn: Box<Fn(A) -> Result<B, PartialFnError<A>> + 'a>,
	__is_defined_at_fn: Box<Fn(A) -> bool + 'a>
}


impl<'a, A, B> PartialFn<'a, A, B> {
	pub fn call(&self, arg: A) -> Result<B, PartialFnError<A>> {
		(*self.__call_fn)(arg)
	}

	pub fn is_defined_at(&self, arg: A) -> bool {
		(*self.__is_defined_at_fn)(arg)
	}

	pub fn new(call: Box<Fn(A) -> Result<B, PartialFnError<A>> + 'a>, defined: Box<Fn(A) -> bool + 'a>) -> PartialFn<'a, A, B> {
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
	type Output = Result<B, PartialFnError<A>>;

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
					$($pat)|+ $(if $cond)? => Ok($result)
				),*,
				_ => Err(PartialFnError::MatchError(arg))
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