use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use super::numth::gcd;

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

//TODO: Monoid refactoring
//pub trait Monoid: Zero+Add<Output=Self>로 하면 Group으로 확장 가능할듯?
pub trait Monoid {
	fn id() -> Self;
	fn f(&self, rhs: Self) -> Self;
}
#[derive(Clone)]
pub struct Mmaxi64(pub i64);
impl Monoid for Mmaxi64 {
	fn id() -> Self {
		Mmaxi64(i64::MIN)
	}
	fn f(&self, rhs: Self) -> Self {
		Mmaxi64(self.0.max(rhs.0))
	}
}
#[derive(Clone)]
pub struct Mmini64(pub i64);
impl Monoid for Mmini64 {
	fn id() -> Self {
		Mmini64(i64::MIN)
	}
	fn f(&self, rhs: Self) -> Self {
		Mmini64(self.0.max(rhs.0))
	}
}
#[derive(Clone)]
pub struct Mmaxusize(pub usize);
impl Monoid for Mmaxusize {
	fn id() -> Self {
		Mmaxusize(usize::MIN)
	}
	fn f(&self, rhs: Self) -> Self {
		Mmaxusize(self.0.max(rhs.0))
	}
}
#[derive(Clone)]
pub struct Mminusize(pub usize);
impl Monoid for Mminusize {
	fn id() -> Self {
		Mminusize(usize::MIN)
	}
	fn f(&self, rhs: Self) -> Self {
		Mminusize(self.0.max(rhs.0))
	}
}

// //monoid{i64,+}
// impl Monoid for i64{
// 	fn id()->Self {0}
// 	fn f(&self, rhs:Self)->Self {*self + rhs}
// }

// //monoid{i64,min}
// impl Monoid for i64{
// 	fn id()->Self {i64::MAX}
// 	fn f(&self, rhs:Self)->Self {min(*self,rhs)}
// }

// //monoid{i64,max}
// impl Monoid for i64{
// 	fn id()->Self {i64::MIN}
// 	fn f(&self, rhs:Self)->Self {max(*self,rhs)}
// }

// //monoid{(val,idx),min}
// impl Monoid for (usize,usize){
// 	fn id()->Self{(usize::MAX,usize::MAX)}
// 	fn f(&self,rhs:Self)->Self{min(*self,rhs)}
// }

// //monoid{(val,idx),max}
// impl Monoid for (usize,usize){
// 	fn id()->Self{(usize::MIN,usize::MIN)}
// 	fn f(&self,rhs:Self)->Self{max(*self,rhs)}
// }

pub trait MonoidLazy {
	fn idq() -> Self;
	fn idu() -> Self;
	fn q(&self, rhs: Self) -> Self;
	fn upd(&self, rhs: Self, cnt: i64) -> Self;
	fn acc(&self, rhs: Self) -> Self;
}
// //Query +, Update +
// impl MonoidLazy for i64{
// 	fn idq()->Self {0}
// 	fn idu()->Self {0}
// 	fn q(&self, rhs:Self)->Self { *self+rhs }
// 	fn upd(&self, rhs:Self, cnt:i64)->Self { *self+rhs*cnt }
// 	fn acc(&self, rhs:Self)->Self { *self+rhs }
// }
// //Query max, Update +
// impl MonoidLazy for i64{
// 	fn idq()->Self{0/*i64::MIN*/}
// 	fn idu()->Self{0}
// 	fn q(&self, rhs:Self)->Self{ *self.max(&rhs) }
// 	fn upd(&self, rhs:Self, cnt:i64)->Self{ *self+rhs }
// 	fn acc(&self, rhs:Self)->Self{ *self+rhs }
// }

pub trait Group:
	Copy + PartialEq + PartialOrd + Zero + Add<Output = Self> + Neg<Output = Self>
{
}
impl Group for i32 {}
impl Group for i64 {}
// impl Group for f32{}
// impl Group for f64{}

pub trait Ring: Group + One + Mul<Output = Self> + Sub<Output = Self> {}
impl Ring for i32 {}
impl Ring for i64 {}
// impl Ring for f32{}
// impl Ring for f64{}

pub trait Field: Ring + Div<Output = Self> + Rem<Output = Self> {}
impl Field for i32 {}
impl Field for i64 {}
// impl Field for f32{}
// impl Field for f64{}

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
	fn rem(self, rhs: Self) -> Self {
		panic!("TODO")
	}
}
impl<const MOD: usize> Div for Mod<MOD> {
	type Output = Self;
	fn div(self, rhs: Self) -> Self {
		panic!("TODO")
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
impl<const MOD: usize> Mod<MOD> {}
