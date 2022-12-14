#![cfg_attr(not(test), no_std)] // we build the tests with `std` so that we can use `std::format!` and friends

//! More robust and versatile implementation of `derive(Debug)` and `derive(Display)`. Unlike the version of
//! `derive(Debug)` in the standard library, these macros will always successfully generate an implementation - even if
//! a member does not implement `Debug`/`Display`. In that case, the generated implementation will print a replacement
//! string of the form `<TypeName>`.
//!
//! # More Robust
//! These derive macros always work, even when `derive(std::fmt::Debug)` fails.
//!
//! ```rust
//! # use fmt_derive::_rt; // required for doctests in the `fmt_derive` crate only
//! // a type that implements neither `Debug` nor `Display`
//! struct Unprintable;
//!
//! #[derive(fmt_derive::Debug, fmt_derive::Display)]
//! struct Printable(Unprintable);
//!
//! fn main() {
//!   // error[E0277]: `Unprintable` doesn't implement `Debug`
//!   // println!("{:?}", Unprintable);
//!
//!   assert_eq!(format!("{:?}", Printable(Unprintable)), "Printable(<Unprintable>)");
//!
//!   // deriving display is often more useful with a custom format expression, but will silently fall back to the same
//!   // behavior as `Debug`
//!   assert_eq!(format!("{}", Printable(Unprintable)), "Printable(<Unprintable>)");
//! }
//! ```
//!
//! # Drop in Usage
//! Anything that derives [`std::fmt::Debug`] or [`core::fmt::Debug`] can derive [`fmt_derive::Debug`](`Debug`) instead
//! without any changes required.
//!
//! However, both cannot be `use`d at the same time, as their names clash:
//!
//! ```rust,compile_fail
//! # use fmt_derive::_rt; // required for doctests in the `fmt_derive` crate only
//! # fn main() {}
//! // error[E0252]: the name `Debug` is defined multiple times
//! use fmt_derive::Debug;
//! use core::fmt::Debug;
//! ```
//!
//! The same problem exists for `Display`:
//! ```rust,compile_fail
//! # use fmt_derive::_rt; // required for doctests in the `fmt_derive` crate only
//! # fn main() {}
//! // error[E0252]: the name `Display` is defined multiple times
//! use fmt_derive::Display;
//! use core::fmt::Display;
//! ```
//!
//! If you encounter this problem, there is a simple solution: `use fmt_derive::Debug;` also pulls in the
//! [`std::fmt::Debug`]/[`core::fmt::Debug`] trait, there is no need to `use` the standard library `Debug`.
//!
//! ```rust
//! # use fmt_derive::_rt; // required for doctests in the `fmt_derive` crate only
//! use fmt_derive::Debug; // replace `use std::fmt::Debug;` and `use core::fmt::Debug;`
//!
//! struct Unprintable;
//!
//! #[derive(Debug)]
//! struct Printable(Unprintable);
//!
//! fn main() {
//!   // error[E0277]: `Unprintable` doesn't implement `Debug`
//!   // println!("{:?}", Unprintable);
//!
//!   assert_eq!(format!("{:?}", &Printable(Unprintable) as &dyn Debug), "Printable(<Unprintable>)");
//! }
//! ```
//!
//! # More Versatile
//! The derived implementation can be easily customized using additional attributes.
//!
//! ## Custom Format Expressions
//! A custom representation can be quickly derived using a format expression for the whole structure, enumeration, or
//! untagged unions. This is the expected case when deriving `Display` or when a member needs to be formatted in a
//! special manner:
//!
//! ```rust
//! # use fmt_derive::_rt; // required for doctests in the `fmt_derive` crate only
//! use fmt_derive::{Debug, Display};
//!
//! #[derive(Display, Debug)]
//! #[debug("T<0x{:X}>", self.0)]
//! #[display("A thing that sits on the number {}", self.0)]
//! struct Thing(u32);
//!
//! fn main() {
//!   assert_eq!(format!("{:?}", Thing(0xF7A)), "T<0xF7A>");
//!   assert_eq!(format!("{}", Thing(42)), "A thing that sits on the number 42");
//! }
//! ```
//!
//! ## Custom Format Expressions for Enumeration Variants
//! For enumerations, variants can also be customized:
//!
//! ```rust
//! # use fmt_derive::_rt; // required for doctests in the `fmt_derive` crate only
//! use fmt_derive::Debug;
//!
//! #[derive(Debug)]
//! enum Thing{
//!   // tuple members are exposed as `t0`, `t1`, and so forth
//!   #[debug("Thing::VariantA(0x{:X}, {})", t0, t1)]
//!   VariantA(u32, u32),
//!   // struct members are exposed under their name
//!   #[debug("Thing::VariantB({x})")]
//!   VariantB{
//!     x: u32,
//!     unused: u32,
//!   }
//! }
//!
//! fn main() {
//!   assert_eq!(format!("{:?}", Thing::VariantA(0xF7A, 42)), "Thing::VariantA(0xF7A, 42)");
//!   assert_eq!(format!("{:?}", Thing::VariantB{x: 42, unused: 0}), "Thing::VariantB(42)");
//! }
//! ```
//!
//! ## Custom Format Expressions for Individual Fields
//! Or by customizing an individual field:
//!
//! ```rust
//! # use fmt_derive::_rt; // required for doctests in the `fmt_derive` crate only
//! use fmt_derive::Debug;
//!
//! #[derive(Debug)]
//! struct Thing(#[debug("0x{:X}", self.0)] u32);
//!
//! fn main() {
//!   assert_eq!(format!("{:?}", Thing(0xF7A)), "Thing(0xF7A)");
//! }
//! ```
//!
//! ## Ignoring a Field
//! Although it is possible to derive a debug message for any field, it is sometimes preferable to not print a field at
//! all:
//!
//! ```rust
//! # use fmt_derive::_rt; // required for doctests in the `fmt_derive` crate only
//! use fmt_derive::Debug;
//!
//! #[derive(Debug)]
//! struct Function(#[debug(ignore)] fn());
//!
//! fn main() {
//!   assert_eq!(format!("{:?}", Function(main)), "Function");
//! }
//! ```

/// Derive implementations of `Debug` for arbitrary `struct`s and `enum`s (`union`s are supported only with a
/// top-level format directive). `use`ing [`fmt_derive::Debug`](crate::Debug), will also pull in the
/// [`core::fmt::Debug`]/[`std::fmt::Debug`] trait (but the macro of the same name will be replaced with this one).
pub use fmt_derive_proc::Debug;
/// Derive implementations of `Display` for arbitrary `struct`s and `enum`s (`union`s are supported only with a
/// top-level format directive).. `use`ing [`fmt_derive::Display`](crate::Display), will also pull in the
/// [`core::fmt::Display`]/[`std::fmt::Display`] trait (but the macro of the same name will be replaced with this
/// one).
pub use fmt_derive_proc::Display;

pub mod _rt;

mod test;

/// Glob-exporting this module reexports original [`core::fmt::Debug`] *trait*, while shadowing the macro of the same
/// name (due to the specific reexport of [`fmt_derive_proc::Debug`]). This enables the following use case:
///
/// ```
/// # use fmt_derive::_rt; // required for doctests in the `fmt_derive` crate only
/// use fmt_derive::Debug;
///
/// #[derive(Debug)]
/// struct Derived;
///
/// struct Custom;
///
/// // [`fmt_derive::Debug`] is also reexport of the [`core::fmt::Debug`] trait
/// impl Debug for Custom {
///   fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
///     write!(f, "Custom!")
///   }
/// }
///
/// // All ways of naming this trait are equivalent
/// fn as_debug(_: &dyn Debug) { }
/// fn as_core_debug(_: &dyn core::fmt::Debug) { }
/// fn as_fmt_derive_debug(_: &dyn fmt_derive::Debug) { }
///
/// fn main() {
///   assert_eq!(format!("{:?}", Derived), "Derived");
///   assert_eq!(format!("{:?}", Custom), "Custom!");
///
///   as_debug(&Derived);
///   as_debug(&Custom);
///   as_debug(&"");
///
///   as_core_debug(&Derived);
///   as_core_debug(&Custom);
///   as_core_debug(&"");
///
///   as_fmt_derive_debug(&Derived);
///   as_fmt_derive_debug(&Custom);
///   as_fmt_derive_debug(&"");
/// }
/// ```
mod shadowed_reexport_trick {
	/// The trait and macro from the standard library. Only the trait is actually exposed by this crate.
	pub use core::fmt::Debug;
	/// The trait and macro from the standard library. Only the trait is actually exposed by this crate.
	pub use core::fmt::Display;
}
pub use shadowed_reexport_trick::*;
