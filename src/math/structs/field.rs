use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use super::{one::One, zero::Zero};

pub trait Group:
	Copy + PartialEq + PartialOrd + Zero + Add<Output = Self> + Neg<Output = Self>
{
}
impl Group for i32 {}
impl Group for i64 {}
impl Group for f32 {}
impl Group for f64 {}

pub trait Ring: Group + One + Mul<Output = Self> + Sub<Output = Self> {}
impl Ring for i32 {}
impl Ring for i64 {}
impl Ring for f32 {}
impl Ring for f64 {}

pub trait Field: Ring + Div<Output = Self> + Rem<Output = Self> {}
impl Field for i32 {}
impl Field for i64 {}
impl Field for f32 {}
impl Field for f64 {}
