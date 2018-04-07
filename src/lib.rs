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
///
/// # Type Arguments
///
/// * `A` – Type of the partial function input.
/// * `B` – Type of the partial function output.
pub struct PartialFn<'a, A, B> {
    __call_fn: Box<Fn(A) -> Option<B> + 'a>,
    __is_defined_at_fn: Box<Fn(A) -> bool + 'a>,
}

impl<'a, A, B> PartialFn<'a, A, B> {
    /// Applies the argument to the partial function.
    /// 
    /// # Arguments
    /// 
    /// * `arg` – Intput to the partial function.
    /// 
    /// # Result
    /// 
    /// If `arg` is in the partial function's domain, returns the result of applying the partial
    /// function to `arg` wrapped in
    /// [Some](https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some).
    /// [None](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None) otherwise.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// # #[macro_use] extern crate partial_fn;
    /// # use partial_fn::PartialFn;
    /// # fn main() {
    /// let pf = partial_fn! {
    ///         1 => 2
    /// };
    /// assert_eq!(Some(2), pf.call(1));
    /// # }
    /// ```
    pub fn call(&self, arg: A) -> Option<B> {
        (*self.__call_fn)(arg)
    }

    /// Checks if the partial function is defined at a given input.
    ///
    /// # Arguments
    ///
    /// * `arg` – Intput to the partial function.
    ///
    /// # Result
    ///
    /// True if the argument is within the partial function's domain. False otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[macro_use] extern crate partial_fn;
    /// # use partial_fn::PartialFn;
    /// # fn main() {
    /// let pf = partial_fn! {
    ///         1 => 2
    /// };
    /// assert_eq!(true, pf.is_defined_at(1));
    /// # }
    /// ```
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

/// Construct a partial function `PartialFn<A,B>` from a series of one or more match
/// statements.
///
/// # Remarks
/// 
/// If the match arms provided represent a total function, then you will get an 
/// "unreachable pattern" warning. This macro automatically generates a catch-all case and
/// in the case of a total function, this catch-all case is superfluous. It can be safely
/// ignored.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate partial_fn;
/// # use partial_fn::PartialFn;
/// # fn main() {
/// let pf: PartialFn<i32, String> = partial_fn! {
///         1 => "foo".to_string(),
///         2 | 3 => "bar".to_string(),
///         4...5 => "baz".to_string(),
///         x if x >= 6 && x <= 7 => format!("qux{}", x)
/// };
/// assert_eq!(Some("foo".to_string()), pf.call(1));
/// assert_eq!(Some("bar".to_string()), pf.call(2));
/// assert_eq!(Some("baz".to_string()), pf.call(4));
/// assert_eq!(Some("qux6".to_string()), pf.call(6));
/// # }
/// ```
#[macro_export]
macro_rules! partial_fn (
    ($($($pat:pat)|+ $(if $cond:expr)* => $result:expr),*) => (
        PartialFn::new(
            Box::new(__call_macro!($($($pat)|+ $(if $cond)* => $result),*)),
            Box::new(__is_defined_at_macro!($($($pat)|+ $(if $cond)* => $result),*))
        )
    );
);
