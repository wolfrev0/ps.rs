use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use super::{
	field::{Field, Group, Ring},
	one::One,
	zero::Zero,
};

//NOTE: MOD should prime number to be Field
//NOTE: MOD should be smaller than u32 to not overflow
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd)]
pub struct Mod<const MOD: usize> {
	n: usize,
}
impl<const MOD: usize> From<usize> for Mod<MOD> {
	fn from(value: usize) -> Self {
		Self {
			n: if value < MOD {
				value
			} else if value < MOD + MOD {
				value - MOD
			} else {
				value % MOD
			},
		}
	}
}
impl<const MOD: usize> Group for Mod<MOD> {}
impl<const MOD: usize> Ring for Mod<MOD> {}
impl<const MOD: usize> Field for Mod<MOD> {}
impl<const MOD: usize> Rem for Mod<MOD> {
	type Output = Self;
	fn rem(self, _: Self) -> Self {
		todo!("TODO")
	}
}
impl<const MOD: usize> Div for Mod<MOD> {
	type Output = Self;
	fn div(self, _: Self) -> Self {
		todo!("TODO");
	}
}
impl<const MOD: usize> Add for Mod<MOD> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self {
		Self {
			n: if self.n + rhs.n < MOD {
				self.n + rhs.n
			} else {
				self.n + rhs.n - MOD
			},
		}
	}
}
impl<const MOD: usize> Sub for Mod<MOD> {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self {
		self + -rhs
	}
}
impl<const MOD: usize> Neg for Mod<MOD> {
	type Output = Self;
	fn neg(self) -> Self {
		Self { n: MOD - self.n }
	}
}
impl<const MOD: usize> Mul for Mod<MOD> {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self {
		Self {
			n: self.n * rhs.n % MOD,
		}
	}
}
impl<const MOD: usize> Zero for Mod<MOD> {
	fn zero() -> Self {
		Self { n: 0 }
	}

	fn is_zero(&self) -> bool {
		*self == Self::zero()
	}
}
impl<const MOD: usize> One for Mod<MOD> {
	fn one() -> Self {
		Self { n: 1 }
	}
	fn is_one(&self) -> bool {
		*self == Self::one()
	}
}
impl<const MOD: usize> Mod<MOD> {
	pub fn get(&self) -> usize {
		self.n
	}
}
