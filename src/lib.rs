//! A partial function of type `PartialFn<A,B>` is a unary function whose domain is a subset of A.
//! In addition to being able to call a `PartialFn`, a method `is_defined_at` is provided in
//! order to test whether the given `PartialFn` is defined at a particular value.

#![deny(missing_docs)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]
#![cfg_attr(not(feature = "unstable"), deny(unstable_features))]
#![cfg_attr(feature = "unstable", feature(fn_traits))]
#![cfg_attr(feature = "unstable", feature(unboxed_closures))]

/// The `PartialFn` type.
pub struct PartialFn<'a, A, B> {
    __call_fn: Box<Fn(A) -> Option<B> + 'a>,
    __is_defined_at_fn: Box<Fn(A) -> bool + 'a>,
}

impl<'a, A, B> PartialFn<'a, A, B> {
    /// TODO: call documentation
    pub fn call(&self, arg: A) -> Option<B> {
        (*self.__call_fn)(arg)
    }

    /// TODO: is_defined_at documentation
    pub fn is_defined_at(&self, arg: A) -> bool {
        (*self.__is_defined_at_fn)(arg)
    }

    #[doc(hidden)]
    pub fn new(
        call: Box<Fn(A) -> Option<B> + 'a>,
        defined: Box<Fn(A) -> bool + 'a>,
    ) -> PartialFn<'a, A, B> {
        PartialFn {
            __call_fn: call,
            __is_defined_at_fn: defined,
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

#[macro_export]
#[doc(hidden)]
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

#[macro_export]
#[doc(hidden)]
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

#[macro_export]
macro_rules! partial_fn (
    ($($($pat:pat)|+ $(if $cond:expr)* => $result:expr),*) => (
        PartialFn::new(
            Box::new(__call_macro!($($($pat)|+ $(if $cond)* => $result),*)),
            Box::new(__is_defined_at_macro!($($($pat)|+ $(if $cond)* => $result),*))
        )
    );
);
