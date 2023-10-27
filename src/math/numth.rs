use std::ops::{Mul, Rem};

use super::structs::Zero;

pub fn sq<T>(x:T)->T where T:Copy + Mul<Output = T>{ x*x }
pub fn gcd<T>(x:T, y:T)->T where T:Copy+Rem<Output=T>+Zero<T>+PartialEq{
	if x==T::zero(){y}
	else{gcd(y%x, x)}
}
pub fn gcd_fast<T>(x:T, y:T)->T{
	//TODO: binary_gcd
	assert!(false);
	let _ = x;
	y
}