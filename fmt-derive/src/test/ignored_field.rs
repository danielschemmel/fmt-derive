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

	pub mod not_unit_tuple {
		use super::Debug;

		#[derive(Debug)]
		pub struct Unit(#[debug(ignore)] pub u32);
	}

	pub mod not_unit_struct {
		use super::Debug;

		#[derive(Debug)]
		pub struct Unit {
			#[debug(ignore)]
			#[allow(dead_code)]
			pub x: u32,
		}
	}

	#[derive(Debug)]
	pub struct Tuple(pub u32, #[debug(ignore)] pub u32);

	#[derive(Debug)]
	pub struct Struct {
		pub x: u32,
		#[debug(ignore)]
		#[allow(dead_code)]
		pub y: u32,
	}
}

#[test]
fn not_unit_test() {
	assert_eq!(
		format!("{:?}", std::Unit),
		format!("{:?}", our::not_unit_tuple::Unit(0))
	);
	assert_eq!(
		format!("{:?}", std::Unit),
		format!("{:?}", our::not_unit_struct::Unit { x: 0 })
	);
}

#[test]
fn tuple_struct_test() {
	assert_eq!(format!("{:?}", std::Tuple(0)), format!("{:?}", our::Tuple(0, 1)));
}

#[test]
fn struct_test() {
	assert_eq!(
		format!("{:?}", std::Struct { x: 0 }),
		format!("{:?}", our::Struct { x: 0, y: 1 })
	);
}
