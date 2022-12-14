use pretty_assertions::assert_eq;

use crate::{Debug, Display};

pub struct Unprintable(u32);

#[derive(Debug, Display)]
pub enum Tuple {
	#[fmt("V({})", t0)]
	Variant(u32),
}

#[derive(Debug, Display)]
pub enum TupleUnprintable {
	#[fmt("V({})", t0.0)]
	Variant(Unprintable),
}

#[derive(Debug, Display)]
pub enum Struct {
	#[fmt("A({})", x)]
	VariantA { x: u32 },
	#[fmt("B({})", x)]
	VariantB { x: u64 },
}

#[derive(Debug, Display)]
pub enum StructUnprintable {
	#[fmt("V({})", x.0)]
	Variant { x: Unprintable },
}

#[test]
fn tuple_variant_test() {
	assert_eq!(format!("{:?}", Tuple::Variant(0)), "V(0)");
	assert_eq!(format!("{}", Tuple::Variant(0)), "V(0)");
	assert_eq!(format!("{:?}", TupleUnprintable::Variant(Unprintable(0))), "V(0)");
	assert_eq!(format!("{}", TupleUnprintable::Variant(Unprintable(0))), "V(0)");
}

#[test]
fn struct_variant_test() {
	assert_eq!(format!("{:?}", Struct::VariantA { x: 0 }), "A(0)");
	assert_eq!(format!("{}", Struct::VariantA { x: 0 }), "A(0)");
	assert_eq!(format!("{:?}", Struct::VariantB { x: 0 }), "B(0)");
	assert_eq!(format!("{}", Struct::VariantB { x: 0 }), "B(0)");
	assert_eq!(
		format!("{:?}", StructUnprintable::Variant { x: Unprintable(0) }),
		"V(0)"
	);
	assert_eq!(format!("{}", StructUnprintable::Variant { x: Unprintable(0) }), "V(0)");
}
