use std::ops::Mul;

use super::structs::one::One;

pub fn pow<T: Clone + Mul<T, Output = T> + One>(x: T, y: usize) -> T {
	if y == 0 {
		T::one()
	} else if y == 1 {
		x
	} else {
		let half = pow(x.clone(), y / 2);
		half.clone() * half * if y % 2 == 0 { T::one() } else { x }
	}
}
