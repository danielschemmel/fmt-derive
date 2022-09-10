#![doc(hidden)]
/// This module contains the runtime used by the implementations generated from the derive macros.
///
/// The whole of this module is semver version exempt, as it is not intended to be used directly (think of it as an
/// unexposed internal dependency).

pub trait Replacement {
	fn tuple_field<'a, 'b, 'c>(
		&self,
		replacement: &str,
		w: &'a mut core::fmt::DebugTuple<'b, 'c>,
	) -> &'a mut core::fmt::DebugTuple<'b, 'c> {
		w.field(&DebugDisplay(replacement))
	}

	fn struct_field<'a, 'b, 'c>(
		&self,
		name: &str,
		replacement: &str,
		w: &'a mut core::fmt::DebugStruct<'b, 'c>,
	) -> &'a mut core::fmt::DebugStruct<'b, 'c> {
		w.field(name, &DebugDisplay(replacement))
	}
}
impl<T> Replacement for T {}

pub struct DebugOrReplacement<'a, T>(pub &'a T);

impl<'a, T: core::fmt::Debug> DebugOrReplacement<'a, T> {
	pub fn tuple_field<'b, 'c, 'd>(
		&self,
		_replacement: &str,
		w: &'b mut core::fmt::DebugTuple<'c, 'd>,
	) -> &'b mut core::fmt::DebugTuple<'c, 'd> {
		w.field(&self.0)
	}

	pub fn struct_field<'b, 'c, 'd>(
		&self,
		name: &str,
		_replacement: &str,
		w: &'b mut core::fmt::DebugStruct<'c, 'd>,
	) -> &'b mut core::fmt::DebugStruct<'c, 'd> {
		w.field(name, &self.0)
	}
}

pub struct DisplayOrReplacement<'a, T>(pub &'a T);

impl<'a, T: core::fmt::Display> DisplayOrReplacement<'a, T> {
	pub fn tuple_field<'b, 'c, 'd>(
		&self,
		_replacement: &str,
		w: &'b mut core::fmt::DebugTuple<'c, 'd>,
	) -> &'b mut core::fmt::DebugTuple<'c, 'd> {
		w.field(&DebugDisplay(&self.0))
	}

	pub fn struct_field<'b, 'c, 'd>(
		&self,
		name: &str,
		_replacement: &str,
		w: &'b mut core::fmt::DebugStruct<'c, 'd>,
	) -> &'b mut core::fmt::DebugStruct<'c, 'd> {
		w.field(name, &DebugDisplay(&self.0))
	}
}

pub struct DebugDisplay<'a, T: core::fmt::Display + ?Sized>(pub &'a T);

impl<'a, T: core::fmt::Display + ?Sized> core::fmt::Debug for DebugDisplay<'a, T> {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "{}", self.0)
	}
}
