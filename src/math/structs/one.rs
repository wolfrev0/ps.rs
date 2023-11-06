pub trait One {
	fn one() -> Self;
	fn is_one(&self) -> bool;
}
impl One for i32 {
	fn one() -> Self {
		1
	}
	fn is_one(&self) -> bool {
		*self == 1
	}
}
impl One for i64 {
	fn one() -> Self {
		1
	}
	fn is_one(&self) -> bool {
		*self == 1
	}
}
impl One for f32 {
	fn one() -> Self {
		1.0
	}
	fn is_one(&self) -> bool {
		*self == 1.
	}
}
impl One for f64 {
	fn one() -> Self {
		1.0
	}
	fn is_one(&self) -> bool {
		*self == 1.
	}
}
impl One for usize {
	fn one() -> Self {
		1
	}
	fn is_one(&self) -> bool {
		*self == 1
	}
}
