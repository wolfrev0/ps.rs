use std::ops::Mul;

use super::{field::Field, one::One};

#[derive(Clone, PartialEq, Eq)]
pub struct Matrix<T: Field, const R: usize, const C: usize> {
	pub a: [[T; C]; R],
}

impl<T: Field, const R: usize, const C: usize> Matrix<T, R, C> {
	pub fn new() -> Self {
		Self {
			a: [[T::zero(); C]; R],
		}
	}
}

impl<T: Field, const R: usize, const C: usize, const C2: usize> Mul<Matrix<T, C, C2>>
	for Matrix<T, R, C>
{
	type Output = Matrix<T, R, C2>;

	fn mul(self, rhs: Matrix<T, C, C2>) -> Self::Output {
		let mut ret = Self::Output::new();
		for i in 0..R {
			for j in 0..C {
				for k in 0..C2 {
					ret.a[i][k] = ret.a[i][k] + self.a[i][j] * rhs.a[j][k];
				}
			}
		}
		ret
	}
}

impl<T: Field, const R: usize, const C: usize> One for Matrix<T, R, C> {
	fn one() -> Self {
		let mut ret = Self::new();
		assert!(R == C); //todo: static assert
		for i in 0..R {
			ret.a[i][i] = T::one();
		}
		ret
	}

	fn is_one(&self) -> bool {
		*self == Self::one()
	}
}
