use crate::math::structs::monoid::MonoidLazy;

struct Node<T> {
	val: T,
	lz: T,
	l: usize,
	r: usize,
}

pub struct SegLazy<T, const XS: i64, const XE: i64> {
	a: Vec<Node<T>>,
}
impl<T, const XS: i64, const XE: i64> SegLazy<T, XS, XE>
where
	T: MonoidLazy + PartialEq + Clone + Copy,
{
	pub fn new() -> SegLazy<T, XS, XE> {
		let mut ret = SegLazy { a: Vec::new() };
		ret.alloc();
		ret
	}
	pub fn q(&mut self, qs: i64, qe: i64) -> T {
		self.qimpl(qs, qe, 0, XS, XE)
	}
	pub fn upd(&mut self, us: i64, ue: i64, x: T) {
		self.updimpl(us, ue, x, 0, XS, XE);
	}
	pub fn updass(&mut self, idx: i64, x: T) {
		self.updassimpl(idx, x, 0, XS, XE);
	}
	fn alloc(&mut self) -> usize {
		self.a.push(Node {
			val: T::idq(),
			lz: T::idu(),
			l: 0,
			r: 0,
		});
		self.a.len() - 1
	}
	fn pushdown(&mut self, id: usize, s: i64, e: i64) -> T {
		if self.a[id].lz == T::idu() {
			self.a[id].val
		} else {
			let idl = self.a[id].l;
			let idr = self.a[id].r;
			self.a[id].val = self.a[id].val.upd(self.a[id].lz, e - s);
			self.a[idl].lz = self.a[idl].lz.acc(self.a[id].lz);
			self.a[idr].lz = self.a[idr].lz.acc(self.a[id].lz);
			self.a[id].lz = T::idu();
			self.a[id].val
		}
	}
	fn qimpl(&mut self, qs: i64, qe: i64, id: usize, s: i64, e: i64) -> T {
		if self.a[id].l == 0 {
			self.a[id].l = self.alloc();
		}
		if self.a[id].r == 0 {
			self.a[id].r = self.alloc();
		}
		self.pushdown(id, s, e);
		if qe <= s || e <= qs {
			T::idq()
		} else if qs <= s && e <= qe {
			self.a[id].val
		} else {
			let m = (s + e) / 2;
			self.qimpl(qs, qe, self.a[id].l, s, m)
				.q(self.qimpl(qs, qe, self.a[id].r, m, e))
		}
	}
	fn updimpl(&mut self, us: i64, ue: i64, x: T, id: usize, s: i64, e: i64) -> T {
		if self.a[id].l == 0 {
			self.a[id].l = self.alloc();
		}
		if self.a[id].r == 0 {
			self.a[id].r = self.alloc();
		}
		self.pushdown(id, s, e);
		if ue <= s || e <= us {
			self.a[id].val
		} else if us <= s && e <= ue {
			self.a[id].lz = self.a[id].lz.acc(x);
			self.pushdown(id, s, e)
		} else {
			let m = (s + e) / 2;
			let resl = self.updimpl(us, ue, x, self.a[id].l, s, m);
			let resr = self.updimpl(us, ue, x, self.a[id].r, m, e);
			self.a[id].val = resl.q(resr);
			self.a[id].val
		}
	}
	fn updassimpl(&mut self, idx: i64, x: T, id: usize, s: i64, e: i64) -> T {
		if self.a[id].l == 0 {
			self.a[id].l = self.alloc();
		}
		if self.a[id].r == 0 {
			self.a[id].r = self.alloc();
		}
		self.pushdown(id, s, e);
		if idx + 1 <= s || e <= idx {
		} else if idx <= s && e <= idx + 1 {
			self.a[id].val = x;
		} else {
			let m = (s + e) / 2;
			let resl = self.updassimpl(idx, x, self.a[id].l, s, m);
			let resr = self.updassimpl(idx, x, self.a[id].r, m, e);
			self.a[id].val = resl.q(resr);
		}
		self.a[id].val
	}
}

#[cfg(test)]
mod tests {
	use crate::{math::structs::monoid::MonoidLazy, tree::segdynlazy::SegLazy};

	#[test]
	fn test0() {
		//Query max, Update +
		impl MonoidLazy for i16 {
			fn idq() -> Self {
				0 /*i64::MIN*/
			}
			fn idu() -> Self {
				0
			}
			fn q(&self, rhs: Self) -> Self {
				*self.max(&rhs)
			}
			fn upd(&self, rhs: Self, _cnt: i64) -> Self {
				*self + rhs
			}
			fn acc(&self, rhs: Self) -> Self {
				*self + rhs
			}
		}
		let mut st = SegLazy::<i16, 0, 10>::new();

		assert_eq!(st.q(0, 3), 0);
		st.upd(2, 3, 7);
		st.upd(5, 7, 5);
		assert_eq!(st.q(2, 3), 7);
		assert_eq!(st.q(5, 6), 5);
		assert_eq!(st.q(2, 7), 7);
		assert_eq!(st.q(3, 5), 0);
	}
}
