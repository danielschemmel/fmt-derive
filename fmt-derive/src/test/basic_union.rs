#![cfg(feature = "std")]

use pretty_assertions::assert_eq;

mod our {
	use crate::{Debug, Display};

	#[derive(Debug)]
	pub union UnionDebug {
		pub unsigned: u32,
		pub float: f32,
	}

	#[derive(Display)]
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
