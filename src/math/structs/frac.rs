use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use crate::math::numth::gcd;

use super::{
	field::{Field, Group, Ring},
	one::One,
	zero::Zero,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd)]
pub struct Frac<T: Field> {
	a: T,
	b: T,
}
impl<T: Field> Group for Frac<T> {}
impl<T: Field> Ring for Frac<T> {}
impl<T: Field> Field for Frac<T> {}
impl<T: Field> Rem for Frac<T> {
	type Output = Self;
	fn rem(self, rhs: Self) -> Self {
		Self::zero()
	}
}
impl<T: Field> Div for Frac<T> {
	type Output = Self;
	fn div(self, rhs: Self) -> Self {
		Self {
			a: self.a * rhs.b,
			b: self.b * rhs.a,
		}
	}
}
impl<T: Field> Add for Frac<T> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self {
		Self {
			a: self.a * rhs.b + rhs.a * self.b,
			b: self.b * rhs.b,
		}
	}
}
impl<T: Field> Sub for Frac<T> {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self {
		self + -rhs
	}
}
impl<T: Field> Neg for Frac<T> {
	type Output = Self;
	fn neg(self) -> Self {
		Self {
			a: -self.a,
			b: self.b,
		}
	}
}
impl<T: Field> Mul for Frac<T> {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self {
		Self {
			a: self.a * rhs.a,
			b: self.b * rhs.b,
		}
	}
}
impl<T: Field> Zero for Frac<T> {
	fn zero() -> Self {
		Self {
			a: T::zero(),
			b: T::one(),
		}
	}

	fn is_zero(&self) -> bool {
		self.normalized() == Self::zero()
	}
}
impl<T: Field> One for Frac<T> {
	fn one() -> Self {
		Self {
			a: T::one(),
			b: T::one(),
		}
	}
	fn is_one(&self) -> bool {
		self.normalized() == Self::one()
	}
}
impl<T: Field> Frac<T> {
	pub fn normalized(self) -> Self {
		//NaN to 0/0
		//div gcd
		//make b non negative
		if self.isnan() {
			Self {
				a: T::zero(),
				b: T::zero(),
			}
		} else {
			let g = gcd(self.a, self.b);
			let mut ret = Self {
				a: self.a / g,
				b: self.b / g,
			};
			if ret.b < T::zero() {
				ret.b = -ret.b;
			}
			ret
		}
	}
	pub fn isnan(self) -> bool {
		self.b.is_zero()
	}
}
