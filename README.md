[![crates.io](https://img.shields.io/crates/v/fmt-derive?style=flat-square)](https://crates.io/crates/fmt_derive)
[![docs.rs](https://img.shields.io/docsrs/fmt-derive?style=flat-square)](https://docs.rs/fmt-derive/latest/fmt_derive/)

More robust and versatile implementation of `derive(Debug)` and `derive(Display)`. Unlike the version of`derive(Debug)`
in the standard library, these macros will always successfully generate an implementation - even if a member does not
implement `Debug`/`Display`. In that case, the generated implementation will print a replacement string of the form
`<TypeName>`.

```rust
use fmt_derive::Debug; // replacement for `use std::fmt::Debug;`

// a type that implements neither `Debug` nor `Display`
struct Unprintable;

#[derive(Debug, fmt_derive::Display)]
struct Printable {
	// unprintable members will be printed as `<Type>`
	unprintable: Unprintable,

	// use `#[display(ignore)]` (or `#[debug(ignore]` respectively) to skip a member when printing
	#[display(ignore)]
	ignored: u32,

	// use the `fmt` attribute to refer to both `Debug` and `Display` at once
	#[fmt("{:#08X}", self.hex_number)]
	hex_number: u32,
}

fn main() {
	let printable = Printable{
		hex_number: 0xDEADBEEF,
		ignored: 42,
		unprintable: Unprintable,
	};

	assert_eq!(format!("{:?}", printable), "Printable { unprintable: <Unprintable>, ignored: 42, hex_number: 0xDEADBEEF }");
	assert_eq!(format!("{}", printable), "Printable { unprintable: <Unprintable>, hex_number: 0xDEADBEEF }");
}
```

# `no_std`
This crate is `no_std` and can be used from both `no_std` and `std` contexts without any action required.

# MSRV
The current MSRV is 1.56.0.
