use pretty_assertions::assert_eq;

mod std {
	// We disable the dead_code analysis, as it intentionally ignores usages via `derive(Debug)`
	#![allow(dead_code)]

	#[derive(Debug)]
	pub struct Unit;

	#[derive(Debug)]
	pub struct Tuple(pub u32);

	#[derive(Debug)]
	pub struct Struct {
		pub x: u32,
	}
}

mod our {
	use crate::Debug;

	pub struct Unprintable;

	#[derive(Debug)]
	pub struct Unit;

	#[derive(Debug)]
	pub struct Tuple(pub u32);

	#[derive(Debug)]
	pub struct TupleUnprintable(pub Unprintable);

	#[derive(Debug)]
	pub struct Struct {
		pub x: u32,
	}

	#[derive(Debug)]
	pub struct StructUnprintable {
		pub x: Unprintable,
	}
}

#[test]
fn unit_struct_test() {
	assert_eq!(format!("{:?}", std::Unit), format!("{:?}", our::Unit));
}

#[test]
fn tuple_struct_test() {
	assert_eq!(format!("{:?}", std::Tuple(0)), format!("{:?}", our::Tuple(0)));
	assert_eq!(
		format!("{:?}", our::TupleUnprintable(our::Unprintable)),
		"TupleUnprintable(<Unprintable>)"
	);
}

#[test]
fn struct_test() {
	assert_eq!(
		format!("{:?}", std::Struct { x: 0 }),
		format!("{:?}", our::Struct { x: 0 })
	);
	assert_eq!(
		format!("{:?}", our::StructUnprintable { x: our::Unprintable }),
		"StructUnprintable { x: <Unprintable> }"
	);
}
