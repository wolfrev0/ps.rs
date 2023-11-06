pub trait Inf {
	fn inf() -> Self;
	fn is_inf(&self) -> bool;
}
impl Inf for i32 {
	fn inf() -> Self {
		i32::MAX / 2
	}
	fn is_inf(&self) -> bool {
		*self == i32::MAX / 2
	}
}
impl Inf for i64 {
	fn inf() -> Self {
		i64::MAX / 2
	}
	fn is_inf(&self) -> bool {
		*self == i64::MAX / 2
	}
}
impl Inf for f32 {
	fn inf() -> Self {
		f32::MAX
	}
	fn is_inf(&self) -> bool {
		*self == f32::MAX
	}
}
impl Inf for f64 {
	fn inf() -> Self {
		f64::MAX
	}
	fn is_inf(&self) -> bool {
		*self == f64::MAX
	}
}
impl Inf for usize {
	fn inf() -> Self {
		usize::MAX / 2
	}
	fn is_inf(&self) -> bool {
		*self == usize::MAX / 2
	}
}
