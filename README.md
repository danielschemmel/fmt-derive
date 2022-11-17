More robust and versatile implementation of `derive(Debug)` and `derive(Display)`. Unlike the version of`derive(Debug)`
in the standard library, these macros will always successfully generate implementation - even if a member does not
implement `Debug`/`Display`. In that case, the generated implementation will print a replacement string of the form
`<TypeName>`.

# More Robust
These derive macros always work, even when `derive(std::fmt::Debug)` fails.
```rust
# use fmt_derive::_rt; // required for doc-tests in this crate only
struct Unprintable;

#[derive(fmt_derive::Debug, fmt_derive::Display)]
struct Printable(Unprintable);

fn main() {
	// error[E0277]: `Unprintable` doesn't implement `Debug`
	// println!("{:?}", Unprintable);

	assert_eq!(format!("{:?}", Printable(Unprintable)), "Printable(<Unprintable>)");

	// deriving display is often more useful with a custom format expression, but will silently fall back to the same
	// behavior as `Debug`
	assert_eq!(format!("{}", Printable(Unprintable)), "Printable(<Unprintable>)");
}
```

# Drop in Usage
Anything that derives [`std::fmt::Debug`] or [`core::fmt::Debug`] can derive [`fmt_derive::Debug`](`Debug`) instead
without any changes required.

However, there is a small problem when `use`ing both at the same time:
```rust,compile_fail
# use fmt_derive::_rt; // required for doc-tests in this crate only
# fn main() {}
// error[E0252]: the name `Debug` is defined multiple times
use fmt_derive::Debug;
use core::fmt::Debug;
```

The same problem exists for `Display`:
```rust,compile_fail
# use fmt_derive::_rt; // required for doc-tests in this crate only
# fn main() {}
// error[E0252]: the name `Display` is defined multiple times
use fmt_derive::Display;
use core::fmt::Display;
```

If you encounter this problem, there are two simple solutions
- Qualifying the derive macro. `#[derive(fmt_derive::Debug)]` works just fine
- Rewriting the imports. `use fmt_derive::Debug;` also pulls in the [`std::fmt::Debug`]/[`core::fmt::Debug`] trait (not just the macro).
```rust
# use fmt_derive::_rt; // required for doc-tests in this crate only
use fmt_derive::Debug; // replace `use std::fmt::Debug;` and `use core::fmt::Debug;`

struct Unprintable;

#[derive(Debug)]
struct Printable(Unprintable);

fn main() {
	// error[E0277]: `Unprintable` doesn't implement `Debug`
	// println!("{:?}", Unprintable);

	assert_eq!(format!("{:?}", &Printable(Unprintable) as &dyn Debug), "Printable(<Unprintable>)");
}
```

# More Versatile
The derived implementation can be easily customized using additional attributes.

## Custom Format Expressions
A custom representation can be quickly derived using a format expression for the whole structure, enumeration, or
untagged unions. This is the expected case when deriving `Display` or when 
```rust
# use fmt_derive::_rt; // required for doc-tests in this crate only
use fmt_derive::{Debug, Display};

#[derive(Display, Debug)]
#[debug("T<0x{:X}>", self.0)]
#[display("A thing that sits on the number {}", self.0)]
struct Thing(u32);

fn main() {
	// error[E0277]: `Unprintable` doesn't implement `Debug`
	// println!("{:?}", Unprintable);

	assert_eq!(format!("{:?}", Thing(0xF7A)), "T<0xF7A>");
	assert_eq!(format!("{}", Thing(42)), "A thing that sits on the number 42");
}
```

## Custom Format Expressions for Enumeration Variants
For enumerations, variants can also be customized:
```rust
# use fmt_derive::_rt; // required for doc-tests in this crate only
use fmt_derive::Debug;

#[derive(Debug)]
enum Thing{
	#[debug("Hello")]
	Variant(u32),
}

fn main() {
	// error[E0277]: `Unprintable` doesn't implement `Debug`
	// println!("{:?}", Unprintable);

	assert_eq!(format!("{:?}", Thing::Variant(0xF7A)), "Hello");
}
```

## Custom Format Expressions for Individual Fields
Or by customizing an individual field:
```rust
# use fmt_derive::_rt; // required for doc-tests in this crate only
use fmt_derive::Debug;

#[derive(Debug)]
struct Thing(#[debug("0x{:X}", self.0)] u32);

fn main() {
	// error[E0277]: `Unprintable` doesn't implement `Debug`
	// println!("{:?}", Unprintable);

	assert_eq!(format!("{:?}", Thing(0xF7A)), "Thing(0xF7A)");
}
```

## Ignoring a Field
Although it is possible to derive a debug message for any field, it is sometimes preferable to not print a field at
all:
```rust
# use fmt_derive::_rt; // required for doc-tests in this crate only
use fmt_derive::Debug;

#[derive(Debug)]
struct Function(#[debug(ignore)] fn());

fn main() {
	// error[E0277]: `Unprintable` doesn't implement `Debug`
	// println!("{:?}", Unprintable);

	assert_eq!(format!("{:?}", Function(main)), "Function");
}
```

# `no_std`
This crate is `no_std` and can be used from both `no_std` and `std` contexts without any action required.

Tests are built in a `std` context to allow usage of `format!` and friends.

# MSRV
The current MSRV is 1.56.1.

# TODO
- The format expressions on enumeration variants and fields show that this crate will probably need a better method of referring to fields (currently, `self` is accessible, which requires unwrapping the variant each time it is used)
