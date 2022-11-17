use pretty_assertions::assert_eq;

mod std {
	// We disable the dead_code analysis, as it intentionally ignores usages via `derive(Debug)`
	#![allow(dead_code)]

	#[derive(Debug)]
	pub enum Unit {
		Variant,
	}

	#[derive(Debug)]
	pub enum Tuple {
		Variant(u32),
	}

	#[derive(Debug)]
	pub enum Struct {
		Variant { x: u32 },
	}
}

mod our {
	use crate::Debug;

	pub struct Unprintable;

	#[derive(Debug)]
	pub enum Unit {
		Variant,
	}

	#[derive(Debug)]
	pub enum Tuple {
		Variant(u32),
	}

	#[derive(Debug)]
	pub enum TupleUnprintable {
		Variant(Unprintable),
	}

	#[derive(Debug)]
	pub enum Struct {
		Variant { x: u32 },
	}

	#[derive(Debug)]
	pub enum StructUnprintable {
		Variant { x: Unprintable },
	}
}

#[test]
fn unit_struct_test() {
	assert_eq!(format!("{:?}", std::Unit::Variant), format!("{:?}", our::Unit::Variant));
}

#[test]
fn tuple_struct_test() {
	assert_eq!(
		format!("{:?}", std::Tuple::Variant(0)),
		format!("{:?}", our::Tuple::Variant(0))
	);
	assert_eq!(
		format!("{:?}", our::TupleUnprintable::Variant(our::Unprintable)),
		"Variant(<Unprintable>)"
	);
}

#[test]
fn struct_test() {
	assert_eq!(
		format!("{:?}", std::Struct::Variant { x: 0 }),
		format!("{:?}", our::Struct::Variant { x: 0 })
	);
	assert_eq!(
		format!("{:?}", our::StructUnprintable::Variant { x: our::Unprintable }),
		"Variant { x: <Unprintable> }"
	);
}
