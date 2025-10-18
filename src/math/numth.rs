use std::ops::{Mul, Rem};

use super::structs::zero::Zero;

pub fn sq<T>(x: T) -> T
where
	T: Copy + Mul<Output = T>,
{
	x * x
}
pub fn gcd<T>(x: T, y: T) -> T
where
	T: Zero + Rem<Output = T> + Clone + PartialEq,
{
	if x == T::zero() {
		y
	} else {
		gcd(y % x.clone(), x.clone())
	}
}
pub fn gcd_fast<T>(x: T, y: T) -> T {
	//TODO: binary_gcd
	assert!(false);
	let _ = x;
	y
}

pub fn prime_factors(mut n: usize) -> Vec<usize> {
	let mut res = Vec::new();
	let mut f = 2;
	while f * f <= n {
		while n % f == 0 {
			n /= f;
			res.push(f);
		}
		f += 1;
	}
	if n > 1 {
		res.push(n);
	}
	res
}
