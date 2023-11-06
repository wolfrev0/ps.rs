use std::ops::{Add, BitXor, Neg};

use super::{inf::Inf, zero::Zero};

pub trait Monoid {
	fn id() -> Self;
	fn f(&self, rhs: Self) -> Self;
}
#[derive(Clone)]
pub struct MonAdd<T>(pub T);
impl<T> Monoid for MonAdd<T>
where
	T: Ord + Copy + Zero + Add<Output = T>,
{
	fn id() -> Self {
		Self(T::zero())
	}
	fn f(&self, rhs: Self) -> Self {
		Self(self.0 + rhs.0)
	}
}
#[derive(Clone)]
pub struct MonMax<T>(pub T);
impl<T> Monoid for MonMax<T>
where
	T: Ord + Copy + Inf + Neg<Output = T>,
{
	fn id() -> Self {
		Self(-T::inf())
	}
	fn f(&self, rhs: Self) -> Self {
		Self(self.0.max(rhs.0))
	}
}
#[derive(Clone)]
pub struct MonMin<T>(pub T);
impl<T> Monoid for MonMin<T>
where
	T: Ord + Copy + Inf,
{
	fn id() -> Self {
		Self(T::inf())
	}
	fn f(&self, rhs: Self) -> Self {
		Self(self.0.min(rhs.0))
	}
}
#[derive(Clone)]
pub struct MonXor<T>(pub T);
impl<T> Monoid for MonXor<T>
where
	T: Ord + Copy + Zero + BitXor<Output = T>,
{
	fn id() -> Self {
		Self(T::zero())
	}
	fn f(&self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

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
