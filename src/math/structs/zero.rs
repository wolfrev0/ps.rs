pub trait Zero {
	fn zero() -> Self;
	fn is_zero(&self) -> bool;
}
impl Zero for i32 {
	fn zero() -> i32 {
		0
	}
	fn is_zero(&self) -> bool {
		*self == 0
	}
}
impl Zero for i64 {
	fn zero() -> i64 {
		0
	}
	fn is_zero(&self) -> bool {
		*self == 0
	}
}
impl Zero for f32 {
	fn zero() -> f32 {
		0.0
	}
	fn is_zero(&self) -> bool {
		*self == 0.
	}
}
impl Zero for f64 {
	fn zero() -> f64 {
		0.0
	}
	fn is_zero(&self) -> bool {
		*self == 0.
	}
}
impl Zero for usize {
	fn zero() -> Self {
		0
	}
	fn is_zero(&self) -> bool {
		*self == 0
	}
}
