use pretty_assertions::assert_eq;

mod our {
	use crate::{Debug, Display};

	#[derive(Debug)]
	#[allow(dead_code)]
	pub union UnionDebug {
		pub unsigned: u32,
		pub float: f32,
	}

	#[derive(Display)]
	#[allow(dead_code)]
	pub union UnionDisplay {
		pub unsigned: u32,
		pub float: f32,
	}
}

#[test]
fn union_test() {
	assert_eq!(format!("{:?}", our::UnionDebug { unsigned: 32 }), "<UnionDebug>");
	assert_eq!(format!("{}", our::UnionDisplay { unsigned: 32 }), "<UnionDisplay>");
}
