use crate::Debug; // replacement for `use std::fmt::Debug;`

struct Unprintable;

#[derive(Debug, crate::Display)]
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

#[test]
fn main() {
	let printable = Printable {
		hex_number: 0xDEADBEEF,
		ignored: 42,
		unprintable: Unprintable,
	};

	assert_eq!(
		format!("{:?}", printable),
		"Printable { unprintable: <Unprintable>, ignored: 42, hex_number: 0xDEADBEEF }"
	);
	assert_eq!(
		format!("{}", printable),
		"Printable { unprintable: <Unprintable>, hex_number: 0xDEADBEEF }"
	);
}
