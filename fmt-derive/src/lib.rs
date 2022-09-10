#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

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
