use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

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
pub struct Float<T: Field> {
	a: T,
	b: T,
}
impl<T: Field> Group for Float<T> {}
impl<T: Field> Ring for Float<T> {}
impl<T: Field> Field for Float<T> {}
impl<T: Field> Rem for Float<T> {
	type Output = Self;
	fn rem(self, rhs: Self) -> Self {
		Self::zero()
	}
}
impl<T: Field> Div for Float<T> {
	type Output = Self;
	fn div(self, rhs: Self) -> Self {
		Self {
			a: self.a * rhs.b,
			b: self.b * rhs.a,
		}
	}
}
impl<T: Field> Add for Float<T> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self {
		Self {
			a: self.a * rhs.b + rhs.a * self.b,
			b: self.b * rhs.b,
		}
	}
}
impl<T: Field> Sub for Float<T> {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self {
		self + -rhs
	}
}
impl<T: Field> Neg for Float<T> {
	type Output = Self;
	fn neg(self) -> Self {
		Self {
			a: -self.a,
			b: self.b,
		}
	}
}
impl<T: Field> Mul for Float<T> {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self {
		Self {
			a: self.a * rhs.a,
			b: self.b * rhs.b,
		}
	}
}
impl<T: Field> One for Float<T> {
	fn one() -> Self {}
}
impl<T: Field> Zero for Float<T> {}
impl<T: Field> Float<T> {
	pub fn normalize() {
		//div gcd
		//make b non negative
	}
}

// pub struct Mod<const MOD:u32>{
// 	pub n:u64
// }
// impl<const MOD:u32> Field for Mod<MOD>{}
// impl<const MOD:u32> Mod<MOD>{

// }
