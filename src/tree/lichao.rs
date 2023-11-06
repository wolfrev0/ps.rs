use crate::{
	geom::line2::Line2,
	math::structs::{field::Field, inf::Inf, one::One, zero::Zero},
};

pub trait LichaoFuncType {
	type F: Field + From<i64>;
	fn eval(&self, x: Self::F) -> Self::F;
	fn id() -> Self;
}
impl LichaoFuncType for Line2<i64> {
	type F = i64;
	fn eval(&self, x: Self::F) -> Self::F {
		self.caly(x)
	}
	fn id() -> Self {
		Self {
			a: Self::F::zero(),
			b: Self::F::one(),
			c: Self::F::inf(),
		}
	}
}

struct Node<T> {
	v: T,
	l: usize,
	r: usize,
}
pub struct Lichao<T, const XS: i64, const XE: i64> {
	a: Vec<Node<T>>,
}
//max lichao
impl<T: LichaoFuncType, const XS: i64, const XE: i64> Lichao<T, XS, XE> {
	pub fn new() -> Lichao<T, XS, XE> {
		let mut ret = Lichao { a: Vec::new() };
		ret.alloc();
		ret
	}
	fn alloc(&mut self) -> usize {
		self.a.push(Node {
			v: T::id(),
			l: 0,
			r: 0,
		});
		self.a.len() - 1
	}
	pub fn insert(&mut self, v: T) {}
	fn insert_impl(&mut self, v: T, id: usize, s: i64, e: i64) {
		if e - s == 1 {
			if self.a[id].v.eval(T::F::from(s)) < v.eval(T::F::from(s)) {
				self.a[id].v = v;
			}
		}
	}
}
