use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use crate::math::common::pow;

use super::{
	field::{Field, Group, Ring},
	one::One,
	zero::Zero,
};

//NOTE: MOD should prime number to be Field
//NOTE: MOD should be smaller than u32 to not overflow
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug)]
pub struct Mod<const MOD: usize>(usize);
impl<const MOD: usize> From<usize> for Mod<MOD> {
	fn from(value: usize) -> Self {
		Self(if value < MOD {
			value
		} else if value < MOD + MOD {
			value - MOD
		} else {
			value % MOD
		})
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
	fn div(self, rhs: Self) -> Self {
		self * rhs.reci()
	}
}
impl<const MOD: usize> Add for Mod<MOD> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self {
		Self(if self.0 + rhs.0 < MOD {
			self.0 + rhs.0
		} else {
			self.0 + rhs.0 - MOD
		})
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
		Self(MOD - self.0)
	}
}
impl<const MOD: usize> Mul for Mod<MOD> {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self {
		Self(self.0 * rhs.0 % MOD)
	}
}
impl<const MOD: usize> Zero for Mod<MOD> {
	fn zero() -> Self {
		Self(0)
	}

	fn is_zero(&self) -> bool {
		*self == Self::zero()
	}
}
impl<const MOD: usize> One for Mod<MOD> {
	fn one() -> Self {
		Self(1)
	}
	fn is_one(&self) -> bool {
		*self == Self::one()
	}
}
impl<const MOD: usize> Mod<MOD> {
	pub fn get(&self) -> usize {
		self.0
	}
	pub fn reci(self) -> Self {
		pow(self, MOD - 2)
		// Mod inv()const{auto [g,x,y]=xgcd(n, m);assert(g==1);return x<0?x+m:x;}
	}
}
