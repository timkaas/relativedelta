#![allow(unused_macro_rules)]

//! Macros for easy operator overloading.
//!
//! The primary macro to learn is `impl_op!(<op> <closure>);`
//! where `<op>` is an operator and `<closure>` is a closure with the same signature as the trait function associated with `<op>`.
//! The macro you'll actually want to use most of the time, however, is [`impl_op_ex!`](macro.impl_op_ex.html). It works the same way as `impl_op!` but with some extra magic behind the scenes.
//!
//! To use, import the macros with `use auto_ops::*;`. Remember that you can only overload operators between one or more types defined in the current crate (respecting Rust orphan rules).
//! # Examples
//! All of the following examples are run with the following struct defined:
//!
//! ```
//! #[derive(Clone, Debug, PartialEq)]
//! struct DonkeyKong {
//!     pub bananas: i32,
//! }
//! impl DonkeyKong {
//!     pub fn new(bananas: i32) -> DonkeyKong {
//!         DonkeyKong { bananas: bananas }
//!     }
//! }
//! ```
//! ## Binary operators
//!
//! ```ignore
//! // impl_op!(op |a: LHS, b: RHS| -> OUT {...});
//! // impl_op!(op |a: LHS, b: &RHS| -> OUT {...});
//! // impl_op!(op |a: &LHS, b: RHS| -> OUT {...});
//! // impl_op!(op |a: &LHS, b: &RHS| -> OUT {...});
//! // where
//! // OP  : +, -, *, /, %, &, |, ^, <<, >>
//! // a, b: variable names
//!
//! use auto_ops::impl_op;
//! # #[derive(Clone, Debug, PartialEq)]
//! # struct DonkeyKong {
//! #     pub bananas: i32,
//! # }
//! # impl DonkeyKong {
//! #     pub fn new(bananas: i32) -> DonkeyKong {
//! #         DonkeyKong { bananas: bananas }
//! #     }
//! # }
//!
//! impl_op!(- |a: DonkeyKong, b: i32| -> DonkeyKong { DonkeyKong::new(a.bananas - b) });
//! impl_op!(+ |a: &DonkeyKong, b: &DonkeyKong| -> i32 { a.bananas + b.bananas });
//!
//! fn main() {
//!     let dk = DonkeyKong::new(3) - 1;
//!     assert_eq!(DonkeyKong::new(2), dk);
//!     let total_bananas = &dk + &DonkeyKong::new(4);
//!     assert_eq!(6, total_bananas);
//! }
//! ```
//! ## Assignment operators
//! ```ignore
//! // impl_op!(op #[attr] |a: &mut LHS, b: RHS| {...});
//! // impl_op!(op #[attr] |a: &mut LHS, b: &RHS| {...});
//! // impl_op!(op, #[attr] |a: &mut LHS, b: RHS| {...});
//! // impl_op!(op, #[attr] |a: &mut LHS, b: &RHS| {...});
//! // where
//! // attr: any number of attributes
//! // op  : +=, -=, *=, /=, %=, &=, |=, ^=, <<=, >>=
//! // a, b: variable names
//!
//! // Note: the comma is optional and used solely for better formatting.
//!
//! use auto_ops::impl_op;
//! # #[derive(Clone, Debug, PartialEq)]
//! # struct DonkeyKong {
//! #     pub bananas: i32,
//! # }
//! # impl DonkeyKong {
//! #     pub fn new(bananas: i32) -> DonkeyKong {
//! #         DonkeyKong { bananas: bananas }
//! #     }
//! # }
//!
//! impl_op!(+= |a: &mut DonkeyKong, b: DonkeyKong| { a.bananas += b.bananas });
//! impl_op!(+= |a: &mut DonkeyKong, b: &DonkeyKong| { a.bananas += b.bananas });
//!
//! fn main() {
//!     let mut dk = DonkeyKong::new(3);
//!     dk += DonkeyKong::new(1);
//!     assert_eq!(DonkeyKong::new(4), dk);
//!     dk += &DonkeyKong::new(1);
//!     assert_eq!(DonkeyKong::new(5), dk);
//! }
//! ```
//! ## Unary operators
//! ```ignore
//! // impl_op!(op #[attr] |a: LHS| -> OUT {...});
//! // impl_op!(op #[attr] |a: &LHS| -> OUT {...});
//! // impl_op!(op, #[attr] |a: LHS| -> OUT {...});
//! // impl_op!(op, #[attr] |a: &LHS| -> OUT {...});
//! // where
//! // attr: any number of attributes
//! // op: !, -
//! // a: variable name
//!
//! // Note: the comma is optional and used solely for better formatting.
//!
//! use auto_ops::impl_op;
//! # #[derive(Clone, Debug, PartialEq)]
//! # struct DonkeyKong {
//! #     pub bananas: i32,
//! # }
//! # impl DonkeyKong {
//! #     pub fn new(bananas: i32) -> DonkeyKong {
//! #         DonkeyKong { bananas: bananas }
//! #     }
//! # }
//!
//! impl_op!(- |a: DonkeyKong| -> DonkeyKong { DonkeyKong::new(-a.bananas) });
//! impl_op!(- |a: &DonkeyKong| -> DonkeyKong { DonkeyKong::new(-a.bananas) });
//!
//! fn main() {
//!     let dk = -DonkeyKong::new(3);
//!     assert_eq!(DonkeyKong::new(-3), dk);
//!     assert_eq!(DonkeyKong::new(3), -&dk);
//! }
//! ```
//! ## Generics
//! Any of the above forms can additionally be generic by putting generic params just before the lambda.
//! ```ignore
//! // impl_op!(op $[attr] <generic_params> |a: LHS, b: RHS| -> OUT {...});
//! // where
//! // generic_params: anything that would go just after an `impl`, including lifetimes and
//! //                 joining multiple bounds with `+`
//!
//! use auto_ops::impl_op;
//!
//! #[derive(Debug, PartialEq)]
//! struct Barrel<T> {
//!     pub bananas: T,
//! }
//!
//! impl<T> Barrel<T> {
//!     pub fn new(bananas: T) -> Self {
//!         Barrel { bananas }
//!     }
//! }
//!
//! impl_op!(+ <T: ::core::ops::Add<T>> |a: Barrel<T>, b: Barrel<T>| -> Barrel<<T as ::core::ops::Add<T>>::Output> {
//!     Barrel::new(a.bananas + b.bananas)
//! });
//!
//! assert_eq!(Barrel::new(5u8), Barrel::new(2u8) + Barrel::new(3u8));
//! assert_eq!(Barrel::new(1.2f32), Barrel::new(0.5f32) + Barrel::new(0.7f32));
//! ```
//! # Limitations
//! * The output type of any operation must be an owned type (i.e. `impl_op!(+ |a: DonkeyKong b: i32| -> &DonkeyKong {...})` is invalid).
//! * Only some Rust patterns are supported in the closure (`_`, `mut x`, `x`). If you wish to use destructuring or other such patterns, wrap them in parens (`(DonkeyKong { bananas }): DonkeyKong`).
//! * Cannot put `where` clauses on the `impl`, all generic constraints must be in the generic parameters themselves.
//! * Bare generics cannot be used as the type for the first argument in the lambda.
//!
//! ```ignore
//! // impl_op!(+ <T>|a: T, b: SomeType<T>| -> SomeType<T> {...}) // INVALID
//! impl_op!(+ <T>|a: SomeType<T>, b: T| -> SomeType<T> {...})    // VALID
//! impl_op!(+ |a: i32, b: SomeType<i32>| -> SomeType<i32> {...}) // VALID
//! ```
#[macro_use]
mod assignment;
#[macro_use]
mod binary;
#[macro_use]
mod unary;

/// Overloads an operator using the given closure as its body.
///
/// See the [module level documentation](index.html) for more information.
macro_rules! impl_op {
	// For some reason $(,)? doesn't work here
	($op:tt , $($args:tt)*) => {
		_generic_params_shifter_internal!(_impl_op_internal, $op $($args)*);
	};
	($op:tt $($args:tt)*) => {
		_generic_params_shifter_internal!(_impl_op_internal, $op $($args)*);
	};
}

#[doc(hidden)]
macro_rules! _impl_op_internal {
	// Assignment Ops
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : &mut $lhs:ty, $rhs_i:tt : &$rhs:ty| $body:block $($generic_params:tt)*) => {
		_parse_assignment_op!($op, $lhs, &$rhs, lhs, rhs, $(#[$attrs])* {
			|$lhs_i: &mut $lhs, mut $rhs_i: &$rhs| -> () { $body }(lhs, rhs);
		} $($generic_params)*);
	};
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : &mut $lhs:ty, mut $rhs_i:ident : $rhs:ty| $body:block $($generic_params:tt)*) => {
		_parse_assignment_op!($op, $lhs, $rhs, lhs, rhs, $(#[$attrs])* {
			|$lhs_i: &mut $lhs, mut $rhs_i: $rhs| -> () { $body }(lhs, rhs);
		} $($generic_params)*);
	};
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : &mut $lhs:ty, $rhs_i:tt : $rhs:ty| $body:block $($generic_params:tt)*) => {
		_parse_assignment_op!($op, $lhs, $rhs, lhs, rhs, $(#[$attrs])* {
			|$lhs_i: &mut $lhs, $rhs_i: $rhs| -> () { $body }(lhs, rhs);
		} $($generic_params)*);
	};

	// Unary Ops
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : &$lhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => {
		_parse_unary_op!($op, &$lhs, $out, lhs, $(#[$attrs])* {
			|$lhs_i: &$lhs| -> $out { $body }(lhs)
		} $($generic_params)*);
	};
	($op:tt $(#[$attrs:meta])* |mut $lhs_i:ident : $lhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => {
		_parse_unary_op!($op, $lhs, $out, lhs, $(#[$attrs])* {
			|mut $lhs_i: $lhs| -> $out { $body }(lhs)
		} $($generic_params)*);
	};
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : $lhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => {
		_parse_unary_op!($op, $lhs, $out, lhs, $(#[$attrs])* {
			|$lhs_i: $lhs| -> $out { $body }(lhs)
		} $($generic_params)*);
	};

	// Binary Ops
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : &$lhs:ty, $rhs_i:tt : &$rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => {
		_parse_binary_op!($op, &$lhs, &$rhs, $out, lhs, rhs, $(#[$attrs])* {
			|$lhs_i: &$lhs, $rhs_i: &$rhs| -> $out { $body }(lhs, rhs)
		} $($generic_params)*);
	};
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : &$lhs:ty, mut $rhs_i:ident : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => {
		_parse_binary_op!($op, &$lhs, $rhs, $out, lhs, rhs, $(#[$attrs])* {
			|$lhs_i: &$lhs, mut $rhs_i: $rhs| -> $out { $body }(lhs, rhs)
		} $($generic_params)*);
	};
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : &$lhs:ty, $rhs_i:tt : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => {
		_parse_binary_op!($op, &$lhs, $rhs, $out, lhs, rhs, $(#[$attrs])* {
			|$lhs_i: &$lhs, $rhs_i: $rhs| -> $out { $body }(lhs, rhs)
		} $($generic_params)*);
	};
	($op:tt $(#[$attrs:meta])* |mut $lhs_i:ident : $lhs:ty, $rhs_i:tt : &$rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => {
		_parse_binary_op!($op, $lhs, &$rhs, $out, lhs, rhs, $(#[$attrs])* {
			|mut $lhs_i: $lhs, $rhs_i: &$rhs| -> $out { $body }(lhs, rhs)
		} $($generic_params)*);
	};
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : $lhs:ty, $rhs_i:tt : &$rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => {
		_parse_binary_op!($op, $lhs, &$rhs, $out, lhs, rhs, $(#[$attrs])* {
			|$lhs_i: $lhs, $rhs_i: &$rhs| -> $out { $body }(lhs, rhs)
		} $($generic_params)*);
	};

	($op:tt $(#[$attrs:meta])* |mut $lhs_i:ident : $lhs:ty, mut $rhs_i:ident : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => {
		_parse_binary_op!($op, $lhs, $rhs, $out, lhs, rhs, $(#[$attrs])* {
			|mut $lhs_i: $lhs, mut $rhs_i: $rhs| -> $out { $body }(lhs, rhs)
		} $($generic_params)*);
	};
	($op:tt $(#[$attrs:meta])* |mut $lhs_i:ident : $lhs:ty, $rhs_i:tt : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => {
		_parse_binary_op!($op, $lhs, $rhs, $out, lhs, rhs, $(#[$attrs])* {
			|mut $lhs_i: $lhs, $rhs_i: $rhs| -> $out { $body }(lhs, rhs)
		} $($generic_params)*);
	};
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : $lhs:ty, mut $rhs_i:ident : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => {
		_parse_binary_op!($op, $lhs, $rhs, $out, lhs, rhs, $(#[$attrs])* {
			|$lhs_i: $lhs, mut $rhs_i: $rhs| -> $out { $body }(lhs, rhs)
		} $($generic_params)*);
	};
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : $lhs:ty, $rhs_i:tt : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => {
		_parse_binary_op!($op, $lhs, $rhs, $out, lhs, rhs, $(#[$attrs])* {
			|$lhs_i: $lhs, $rhs_i: $rhs| -> $out { $body }(lhs, rhs)
		} $($generic_params)*);
	};
}

/// Overloads an operator using the given closure as its body. Generates overloads for both owned and borrowed variants where possible.
///
/// Used with the same syntax as `impl_op!` (see the [module level documentation](index.html) for more information).
///
/// Expands any borrowed inputs into both owned and borrowed variants.
///
/// `impl_op_ex!(op |a: &LHS, b: &RHS| -> OUT {...});`
/// gets expanded to
///
/// ```ignore
/// impl_op!(op |a: LHS, b: RHS| -> OUT {...});
/// impl_op!(op |a: LHS, b: &RHS| -> OUT {...});
/// impl_op!(op |a: &LHS, b: RHS| -> OUT {...});
/// impl_op!(op |a: &LHS, b: &RHS| -> OUT {...});
/// ```
///
/// and `impl_op_ex!(op |a: &LHS, b: RHS| -> OUT {...});`
/// gets expanded to
///
/// ```ignore
/// impl_op!(op |a: LHS, b: RHS| -> OUT {...});
/// impl_op!(op |a: &LHS, b: RHS| -> OUT {...});
/// ```
/// # Examples
/// ```ignore
/// use auto_ops::impl_op_ex;
/// # #[derive(Clone, Debug, PartialEq)]
/// # struct DonkeyKong {
/// #     pub bananas: i32,
/// # }
/// # impl DonkeyKong {
/// #     pub fn new(bananas: i32) -> DonkeyKong {
/// #         DonkeyKong { bananas: bananas }
/// #     }
/// #  }
///
/// impl_op_ex!(+ |a: &DonkeyKong, b: &DonkeyKong| -> i32 { a.bananas + b.bananas });
///
/// fn main() {
///     let total_bananas = &DonkeyKong::new(2) + &DonkeyKong::new(4);
///     assert_eq!(6, total_bananas);
///     let total_bananas = &DonkeyKong::new(2) + DonkeyKong::new(4);
///     assert_eq!(6, total_bananas);
///     let total_bananas = DonkeyKong::new(2) + &DonkeyKong::new(4);
///     assert_eq!(6, total_bananas);
///     let total_bananas = DonkeyKong::new(2) + DonkeyKong::new(4);
///     assert_eq!(6, total_bananas);
/// }
/// ```
macro_rules! impl_op_ex {
	// For some reason $(,)? doesn't work here
	($op:tt , $($args:tt)*) => {
		_generic_params_shifter_internal!(_impl_op_ex_internal, $op $($args)*);
	};
	($op:tt $($args:tt)*) => {
		_generic_params_shifter_internal!(_impl_op_ex_internal, $op $($args)*);
	};
}

#[doc(hidden)]
macro_rules! _impl_op_ex_internal {
	// Assignment Ops
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : &mut $lhs:ty, $rhs_i:tt : &$rhs:ty| $body:block $($generic_params:tt)*) => (
		_parse_assignment_op!($op, $lhs, &$rhs, lhs, rhs, $(#[$attrs])* {|$lhs_i : &mut $lhs, $rhs_i : &$rhs| -> () {$body} (lhs, rhs);} $($generic_params)*);
		_parse_assignment_op!($op, $lhs, $rhs, lhs, rhs, $(#[$attrs])* {|$lhs_i : &mut $lhs, $rhs_i : &$rhs| -> () {$body} (lhs, &rhs);} $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : &mut $lhs:ty, mut $rhs_i:ident : $rhs:ty| $body:block $($generic_params:tt)*) => (
		_parse_assignment_op!($op, $lhs, $rhs, lhs, rhs, $(#[$attrs])* {|$lhs_i : &mut $lhs, mut $rhs_i : $rhs| -> () {$body} (lhs, rhs);} $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : &mut $lhs:ty, $rhs_i:tt : $rhs:ty| $body:block $($generic_params:tt)*) => (
		_parse_assignment_op!($op, $lhs, $rhs, lhs, rhs, $(#[$attrs])* {|$lhs_i : &mut $lhs, $rhs_i : $rhs| -> () {$body} (lhs, rhs);} $($generic_params)*);
	);

	// Unary Ops
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : &$lhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		_parse_unary_op!($op, &$lhs, $out, lhs, $(#[$attrs])* {|$lhs_i : &$lhs| -> $out {$body} (lhs)} $($generic_params)*);
		_parse_unary_op!($op, $lhs, $out, lhs, $(#[$attrs])* {|$lhs_i : &$lhs| -> $out {$body} (&lhs)} $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |mut $lhs_i:ident : $lhs:ty|  -> $out:ty $body:block $($generic_params:tt)*) => (
		_parse_unary_op!($op, $lhs, $out, lhs, $(#[$attrs])* {|mut $lhs_i : $lhs| -> $out {$body} (lhs)} $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : $lhs:ty|  -> $out:ty $body:block $($generic_params:tt)*) => (
		_parse_unary_op!($op, $lhs, $out, lhs, $(#[$attrs])* {|$lhs_i : $lhs| -> $out {$body} (lhs)} $($generic_params)*);
	);

	// Binary Ops
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : &$lhs:ty, $rhs_i:tt : &$rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op!($op $(#[$attrs])* |$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out $body $($generic_params)*);
		_parse_binary_op!($op, &$lhs, $rhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out {$body} (lhs, &rhs)} $($generic_params)*);
		_parse_binary_op!($op, $lhs, &$rhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out {$body} (&lhs, rhs)} $($generic_params)*);
		_parse_binary_op!($op, $lhs, $rhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out {$body} (&lhs, &rhs)} $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : &$lhs:ty, mut $rhs_i:ident : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op!($op $(#[$attrs])* |$lhs_i : &$lhs, mut $rhs_i : $rhs| -> $out $body $($generic_params)*);
		_parse_binary_op!($op, $lhs, $rhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : &$lhs, mut $rhs_i : $rhs| -> $out {$body} (&lhs, rhs)} $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : &$lhs:ty, $rhs_i:tt : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op!($op $(#[$attrs])* |$lhs_i : &$lhs, $rhs_i : $rhs| -> $out $body $($generic_params)*);
		_parse_binary_op!($op, $lhs, $rhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : &$lhs, $rhs_i : $rhs| -> $out {$body} (&lhs, rhs)} $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |mut $lhs_i:ident : $lhs:ty, $rhs_i:tt : &$rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op!($op $(#[$attrs])* |mut $lhs_i : $lhs, $rhs_i : &$rhs| -> $out $body $($generic_params)*);
		_parse_binary_op!($op, $lhs, $rhs, $out, lhs, rhs, $(#[$attrs])* {|mut $lhs_i : $lhs, $rhs_i : &$rhs| -> $out {$body} (lhs, &rhs)} $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : $lhs:ty, $rhs_i:tt : &$rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op!($op $(#[$attrs])* |$lhs_i : $lhs, $rhs_i : &$rhs| -> $out $body $($generic_params)*);
		_parse_binary_op!($op, $lhs, $rhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : $lhs, $rhs_i : &$rhs| -> $out {$body} (lhs, &rhs)} $($generic_params)*);
	);

	($op:tt $(#[$attrs:meta])* |mut $lhs_i:ident : $lhs:ty, mut $rhs_i:ident : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op!($op $(#[$attrs])* |mut $lhs_i : $lhs, mut $rhs_i : $rhs| -> $out $body $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |mut $lhs_i:ident : $lhs:ty, $rhs_i:tt : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op!($op $(#[$attrs])* |mut $lhs_i : $lhs, $rhs_i : $rhs| -> $out $body $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : $lhs:ty, mut $rhs_i:ident : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op!($op $(#[$attrs])* |$lhs_i : $lhs, mut $rhs_i : $rhs| -> $out $body $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : $lhs:ty, $rhs_i:tt : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op!($op $(#[$attrs])* |$lhs_i : $lhs, $rhs_i : $rhs| -> $out $body $($generic_params)*);
	);
}

/// Overloads a binary operator commutatively using the given closure as its body.
///
/// Used with the same syntax as `impl_op!` (see the [module level documentation](index.html) for more information).
/// Can only be used with binary operators, and the operation must be between two different types.
///
/// An operator is commutative if A <op> B == B <op> A. Common commutative operators are `+` and `*`.
///
/// ```ignore
/// impl_op_commutative!(op |a: LHS, b: RHS| -> OUT {...});
/// // where LHS != RHS
/// ```
///
/// gets expanded to
///
/// ```ignore
/// impl_op!(op |a: LHS, b: RHS| -> OUT {...});
/// impl_op!(op |a: RHS, b: LHS| -> OUT {...});
/// ```
/// Make sure that LHS != RHS, and that the operator you are trying to overload is a commutative one.
/// See the examples for what happens when you try `impl_op_commutative!` on the `-` operator (which isn't usually commutative).
/// # Examples
/// ```ignore
/// use auto_ops::impl_op_commutative;
/// # #[derive(Clone, Debug, PartialEq)]
/// # struct DonkeyKong {
/// #     pub bananas: i32,
/// # }
/// # impl DonkeyKong {
/// #     pub fn new(bananas: i32) -> DonkeyKong {
/// #         DonkeyKong { bananas: bananas }
/// #     }
/// #  }
///
/// impl_op_commutative!(+ |a: DonkeyKong, b: i32| -> i32 { a.bananas + b });
/// // Don't do this unless you know what you are doing:
/// impl_op_commutative!(- |a: DonkeyKong, b: i32| -> i32 { a.bananas - b });
///
/// fn main() {
///     let total_bananas = DonkeyKong::new(5) + 1;
///     assert_eq!(6, total_bananas);
///     let total_bananas = 1 + DonkeyKong::new(5);
///     assert_eq!(6, total_bananas);
///     let total_bananas = DonkeyKong::new(5) - 1;
///     assert_eq!(4, total_bananas);
///     let total_bananas = 1 - DonkeyKong::new(5);
///     assert_eq!(4, total_bananas);
///     // notice that in this case (5 - 1 == 4) and (1 - 5 == 1): that is the definition of a
///     // commutative operator, but probably not what you want for the '-' operator
/// }
/// ```
macro_rules! impl_op_commutative {
	// For some reason $(,)? doesn't work here
	($op:tt , $($args:tt)*) => {
		_generic_params_shifter_internal!(_impl_op_commutative_internal, $op $($args)*);
	};
	($op:tt $($args:tt)*) => {
		_generic_params_shifter_internal!(_impl_op_commutative_internal, $op $($args)*);
	};
}

#[doc(hidden)]
macro_rules! _impl_op_commutative_internal {
	// Binary Ops Only
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : &$lhs:ty, $rhs_i:tt : &$rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op!($op $(#[$attrs])* |$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out $body $($generic_params)*);
		_parse_binary_op!($op, &$rhs, &$lhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out {$body} (rhs, lhs)} $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : &$lhs:ty, mut $rhs_i:ident : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op!($op $(#[$attrs])* |$lhs_i : &$lhs, mut $rhs_i : $rhs| -> $out $body $($generic_params)*);
		_parse_binary_op!($op, $rhs, &$lhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : &$lhs, mut $rhs_i : $rhs| -> $out {$body} (rhs, lhs)} $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : &$lhs:ty, $rhs_i:tt : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op!($op $(#[$attrs])* |$lhs_i : &$lhs, $rhs_i : $rhs| -> $out $body $($generic_params)*);
		_parse_binary_op!($op, $rhs, &$lhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : &$lhs, $rhs_i : $rhs| -> $out {$body} (rhs, lhs)} $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |mut $lhs_i:ident : $lhs:ty, $rhs_i:tt : &$rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op!($op $(#[$attrs])* |mut $lhs_i : $lhs, $rhs_i : &$rhs| -> $out $body $($generic_params)*);
		_parse_binary_op!($op, &$rhs, $lhs, $out, lhs, rhs, $(#[$attrs])* {|mut $lhs_i : $lhs, $rhs_i : &$rhs| -> $out {$body} (rhs, lhs)} $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : $lhs:ty, $rhs_i:tt : &$rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op!($op $(#[$attrs])* |$lhs_i : $lhs, $rhs_i : &$rhs| -> $out $body $($generic_params)*);
		_parse_binary_op!($op, &$rhs, $lhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : $lhs, $rhs_i : &$rhs| -> $out {$body} (rhs, lhs)} $($generic_params)*);
	);

	($op:tt $(#[$attrs:meta])* |mut $lhs_i:ident : $lhs:ty, mut $rhs_i:ident : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op!($op $(#[$attrs])* |mut $lhs_i : $lhs, mut $rhs_i : $rhs| -> $out $body $($generic_params)*);
		_parse_binary_op!($op, $rhs, $lhs, $out, lhs, rhs, $(#[$attrs])* {|mut $lhs_i : $lhs, mut $rhs_i : $rhs| -> $out {$body} (rhs, lhs)} $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |mut $lhs_i:ident : $lhs:ty, $rhs_i:tt : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op!($op $(#[$attrs])* |mut $lhs_i : $lhs, $rhs_i : $rhs| -> $out $body $($generic_params)*);
		_parse_binary_op!($op, $rhs, $lhs, $out, lhs, rhs, $(#[$attrs])* {|mut $lhs_i : $lhs, $rhs_i : $rhs| -> $out {$body} (rhs, lhs)} $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : $lhs:ty, mut $rhs_i:ident : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op!($op $(#[$attrs])* |$lhs_i : $lhs, mut $rhs_i : $rhs| -> $out $body $($generic_params)*);
		_parse_binary_op!($op, $rhs, $lhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : $lhs, mut $rhs_i : $rhs| -> $out {$body} (rhs, lhs)} $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : $lhs:ty, $rhs_i:tt : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op!($op $(#[$attrs])* |$lhs_i : $lhs, $rhs_i : $rhs| -> $out $body $($generic_params)*);
		_parse_binary_op!($op, $rhs, $lhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : $lhs, $rhs_i : $rhs| -> $out {$body} (rhs, lhs)} $($generic_params)*);
	);
}

/// Overloads a binary operator commutatively using the given closure as its body. Generates overloads for both owned and borrowed variants where possible.
///
/// See [`impl_op_commutative!`](macro.impl_op_commutative.html) for usage.
///
/// Expands borrowed inputs to both borrowed and owned variants.
///
/// ```ignore
/// impl_op_ex_commutative!(op |a: &LHS, b: &RHS| -> OUT {...});
/// // where LHS != RHS
/// ```
///
/// gets expanded to
///
/// ```ignore
/// impl_op!(op |a: &LHS, b: &RHS| -> OUT {...});
/// impl_op!(op |a: &LHS, b: RHS| -> OUT {...});
/// impl_op!(op |a: LHS, b: &RHS| -> OUT {...});
/// impl_op!(op |a: LHS, b: RHS| -> OUT {...});
///
/// impl_op!(op |a: &RHS, b: &LHS| -> OUT {...});
/// impl_op!(op |a: &RHS, b: LHS| -> OUT {...});
/// impl_op!(op |a: RHS, b: &LHS| -> OUT {...});
/// impl_op!(op |a: RHS, b: LHS| -> OUT {...});
/// ```
/// # Examples
/// ```ignore
/// use auto_ops::impl_op_ex_commutative;
/// # #[derive(Clone, Debug, PartialEq)]
/// # struct DonkeyKong {
/// #     pub bananas: i32,
/// # }
/// # impl DonkeyKong {
/// #     pub fn new(bananas: i32) -> DonkeyKong {
/// #         DonkeyKong { bananas: bananas }
/// #     }
/// #  }
/// # #[derive(Clone, Debug, PartialEq)]
/// # struct DiddyKong {
/// #     pub bananas: i32,
/// # }
/// # impl DiddyKong {
/// #     pub fn new(bananas: i32) -> DiddyKong {
/// #         DiddyKong { bananas: bananas }
/// #     }
/// #  }
///
/// impl_op_ex_commutative!(+ |a: &DonkeyKong, b: &DiddyKong| -> i32 { a.bananas + b.bananas });
/// impl_op_ex_commutative!(+ |a: &DonkeyKong, b: i32| -> i32 { a.bananas + b });
///
/// fn main() {
///     let total_bananas = &DonkeyKong::new(5) + &DiddyKong::new(1);
///     assert_eq!(6, total_bananas);
///     let total_bananas = &DonkeyKong::new(5) + DiddyKong::new(1);
///     assert_eq!(6, total_bananas);
///     let total_bananas = DonkeyKong::new(5) + &DiddyKong::new(1);
///     assert_eq!(6, total_bananas);
///     let total_bananas = DonkeyKong::new(5) + DiddyKong::new(1);
///     assert_eq!(6, total_bananas);
///
///     let total_bananas = &DiddyKong::new(1) + &DonkeyKong::new(5);
///     assert_eq!(6, total_bananas);
///     let total_bananas = &DiddyKong::new(1) + DonkeyKong::new(5);
///     assert_eq!(6, total_bananas);
///     let total_bananas = DiddyKong::new(1) + &DonkeyKong::new(5);
///     assert_eq!(6, total_bananas);
///     let total_bananas = DiddyKong::new(1) + DonkeyKong::new(5);
///     assert_eq!(6, total_bananas);
///
///     let total_bananas = &DonkeyKong::new(5) + 1;
///     assert_eq!(6, total_bananas);
///     let total_bananas = DonkeyKong::new(5) + 1;
///     assert_eq!(6, total_bananas);
///
///     let total_bananas = 1 + &DonkeyKong::new(5);
///     assert_eq!(6, total_bananas);
///     let total_bananas = 1 + DonkeyKong::new(5);
///     assert_eq!(6, total_bananas);
/// }
/// ```
macro_rules! impl_op_ex_commutative {
	// For some reason $(,)? doesn't work here
	($op:tt , $($args:tt)*) => {
		_generic_params_shifter_internal!(_impl_op_ex_commutative_internal, $op $($args)*);
	};
	($op:tt $($args:tt)*) => {
		_generic_params_shifter_internal!(_impl_op_ex_commutative_internal, $op $($args)*);
	};
}

#[doc(hidden)]
macro_rules! _impl_op_ex_commutative_internal {
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : &$lhs:ty, $rhs_i:tt : &$rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op_ex!($op $(#[$attrs])* |$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out $body $($generic_params)*);

		_parse_binary_op!($op, &$rhs, &$lhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out {$body} (rhs, lhs)} $($generic_params)*);
		_parse_binary_op!($op, &$rhs, $lhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out {$body} (&rhs, lhs)} $($generic_params)*);
		_parse_binary_op!($op, $rhs, &$lhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out {$body} (rhs, &lhs)} $($generic_params)*);
		_parse_binary_op!($op, $rhs, $lhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out {$body} (&rhs, &lhs)} $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : &$lhs:ty, mut $rhs_i:ident : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op_ex!($op $(#[$attrs])* |$lhs_i : &$lhs, mut $rhs_i : $rhs| -> $out $body $($generic_params)*);

		_parse_binary_op!($op, $rhs, &$lhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : &$lhs, mut $rhs_i : $rhs| -> $out {$body} (rhs, lhs)} $($generic_params)*);
		_parse_binary_op!($op, $rhs, $lhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : &$lhs, mut $rhs_i : $rhs| -> $out {$body} (&rhs, lhs)} $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : &$lhs:ty, $rhs_i:tt : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op_ex!($op $(#[$attrs])* |$lhs_i : &$lhs, $rhs_i : $rhs| -> $out $body $($generic_params)*);

		_parse_binary_op!($op, $rhs, &$lhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : &$lhs, $rhs_i : $rhs| -> $out {$body} (rhs, lhs)} $($generic_params)*);
		_parse_binary_op!($op, $rhs, $lhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : &$lhs, $rhs_i : $rhs| -> $out {$body} (&rhs, lhs)} $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |mut $lhs_i:ident : $lhs:ty, $rhs_i:tt : &$rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op_ex!($op $(#[$attrs])* |mut $lhs_i : $lhs, $rhs_i : &$rhs| -> $out $body $($generic_params)*);

		_parse_binary_op!($op, &$rhs, $lhs, $out, lhs, rhs, $(#[$attrs])* {|mut $lhs_i : $lhs, $rhs_i : &$rhs| -> $out {$body} (rhs, lhs)} $($generic_params)*);
		_parse_binary_op!($op, $rhs, $lhs, $out, lhs, rhs, $(#[$attrs])* {|mut $lhs_i : $lhs, $rhs_i : &$rhs| -> $out {$body} (rhs, &lhs)} $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : $lhs:ty, $rhs_i:tt : &$rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op_ex!($op $(#[$attrs])* |$lhs_i : $lhs, $rhs_i : &$rhs| -> $out $body $($generic_params)*);

		_parse_binary_op!($op, &$rhs, $lhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : $lhs, $rhs_i : &$rhs| -> $out {$body} (rhs, lhs)} $($generic_params)*);
		_parse_binary_op!($op, $rhs, $lhs, $out, lhs, rhs, $(#[$attrs])* {|$lhs_i : $lhs, $rhs_i : &$rhs| -> $out {$body} (rhs, &lhs)} $($generic_params)*);
	);

	($op:tt $(#[$attrs:meta])* |mut $lhs_i:ident : $lhs:ty, mut $rhs_i:ident : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op_commutative!($op $(#[$attrs])* |mut $lhs_i : $lhs, mut $rhs_i : $rhs| -> $out $body $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |mut $lhs_i:ident : $lhs:ty, $rhs_i:tt : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op_commutative!($op $(#[$attrs])* |mut $lhs_i : $lhs, $rhs_i : $rhs| -> $out $body $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : $lhs:ty, mut $rhs_i:ident : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op_commutative!($op $(#[$attrs])* |$lhs_i : $lhs, mut $rhs_i : $rhs| -> $out $body $($generic_params)*);
	);
	($op:tt $(#[$attrs:meta])* |$lhs_i:tt : $lhs:ty, $rhs_i:tt : $rhs:ty| -> $out:ty $body:block $($generic_params:tt)*) => (
		impl_op_commutative!($op $(#[$attrs])* |$lhs_i : $lhs, $rhs_i : $rhs| -> $out $body $($generic_params)*);
	);
}

/// This helper allows us to put the generic parameters in a place that's ergonomic (just before
/// the lambda), but still be able to match things after them. The problem with matching things
/// where they are is we *need* to use a `tt*` to capture the generic params in order to allow
/// arbitrary params and constraints, but that will eat the rest of the input (there's no way to
/// limit the kinds of tokens it will consume, and the macro matching engine prevents against
/// arbitrarily long back-tracking). Therefore the `tt*` *must* be at the end of any rule that tries
/// to actually match anything useful.
///
/// To accomplish this, we semantically match the tokens before the generic parameters, then use the
/// tt muncher pattern to shift the tokens representing the generic params to the end of the token
/// stream. We use the assumption that that `|` (a token that is just the pipe character) will never
/// show up in generic parameters, and stop shifting as soon as we encounter it. We then invokes the
/// `callback` macro with the shifted arguments.
#[doc(hidden)]
macro_rules! _generic_params_shifter_internal {
	// Once we've hit a `|`, use the final rule
	($callback:path, $op:tt $(#[$attrs:meta])* | $($tail:tt)*) => {
		$callback!($op $(#[$attrs])* | $($tail)*);
	};
	// Until then, shift tokens describing the generic parameters to the end
	($callback:path, $op:tt $(#[$attrs:meta])* $next:tt $($tail:tt)*) => {
		_generic_params_shifter_internal!($callback, $op $(#[$attrs])* $($tail)* $next);
	};
}
