use std::ops::Mul;

use super::structs::field::Field;

pub fn sq<T>(x: T) -> T
where
	T: Copy + Mul<Output = T>,
{
	x * x
}
pub fn gcd<T>(x: T, y: T) -> T
where
	T: Field,
{
	if x == T::zero() {
		y
	} else {
		gcd(y % x, x)
	}
}
pub fn gcd_fast<T>(x: T, y: T) -> T {
	//TODO: binary_gcd
	assert!(false);
	let _ = x;
	y
}
