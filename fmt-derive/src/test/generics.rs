#![cfg(feature = "std")]

use pretty_assertions::{assert_eq, assert_ne};

mod std {
	// We disable the dead_code analysis, as it intentionally ignores usages via `derive(Debug)`
	#![allow(dead_code)]

	#[derive(Debug)]
	pub struct Unit(pub &'static str);
}

mod our {
	pub mod a {
		use crate::Debug;

		#[derive(Debug)]
		pub struct Unit<'a>(pub &'a str);
	}

	pub mod b {
		use crate::Debug;

		#[derive(Debug)]
		pub struct Unit<'a, T>(pub &'a T);
	}

	pub mod c {
		use crate::Debug;

		#[derive(Debug)]
		pub struct Unit<'a, T: Debug + ?Sized>(pub &'a T);
	}
}

#[test]
fn test_a() {
	assert_eq!(format!("{:?}", std::Unit("a")), format!("{:?}", our::a::Unit("a")));
}

#[test]
fn test_b() {
	assert_ne!(format!("{:?}", std::Unit("a")), format!("{:?}", our::b::Unit(&"a")));
	assert_eq!(format!("{:?}", our::b::Unit(&"a")), "Unit(<& 'a T>)");
}

#[test]
fn test_c() {
	assert_eq!(format!("{:?}", std::Unit("a")), format!("{:?}", our::c::Unit("a")));
}
