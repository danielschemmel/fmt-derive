#![cfg(feature = "std")]

use pretty_assertions::assert_eq;

mod our {
	use crate::{Debug, Display, Fmt};

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

	#[derive(Fmt)]
	pub union UnionFmt {
		pub unsigned: u32,
		pub float: f32,
	}
}

#[test]
fn union_test() {
	assert_eq!(format!("{:?}", our::UnionDebug { unsigned: 32 }), "<UnionDebug>");
	assert_eq!(format!("{}", our::UnionDisplay { unsigned: 32 }), "<UnionDisplay>");
	assert_eq!(format!("{:?}", our::UnionFmt { unsigned: 32 }), "<UnionFmt>");
	assert_eq!(format!("{}", our::UnionFmt { unsigned: 32 }), "<UnionFmt>");
}
