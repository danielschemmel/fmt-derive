use pretty_assertions::assert_eq;

use crate::Debug;

pub struct Unprintable(u32);

#[derive(Debug)]
pub enum Tuple {
	#[debug("V({})", t0)]
	Variant(u32),
}

#[derive(Debug)]
pub enum TupleUnprintable {
	#[debug("V({})", t0.0)]
	Variant(Unprintable),
}

#[derive(Debug)]
pub enum Struct {
	#[debug("A({})", x)]
	VariantA { x: u32 },
	#[debug("B({})", x)]
	VariantB { x: u64 },
}

#[derive(Debug)]
pub enum StructUnprintable {
	#[debug("V({})", x.0)]
	Variant { x: Unprintable },
}

#[test]
fn tuple_variant_test() {
	assert_eq!(format!("{:?}", Tuple::Variant(0)), "V(0)");
	assert_eq!(format!("{:?}", TupleUnprintable::Variant(Unprintable(0))), "V(0)");
}

#[test]
fn struct_variant_test() {
	assert_eq!(format!("{:?}", Struct::VariantA { x: 0 }), "A(0)");
	assert_eq!(format!("{:?}", Struct::VariantB { x: 0 }), "B(0)");
	assert_eq!(
		format!("{:?}", StructUnprintable::Variant { x: Unprintable(0) }),
		"V(0)"
	);
}
