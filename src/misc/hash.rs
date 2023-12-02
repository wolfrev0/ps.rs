use std::fmt::Debug;

use crate::math::structs::{modulo::Mod, monoid::Monoid, zero::Zero};

#[deprecated(note = "Check upper bound before use it")]
const MAX_LEN: usize = 1_000_000;
const MOD: usize = 998244353;
const POW13: [usize; MAX_LEN] = pows::<13, MAX_LEN>();
const POW31: [usize; MAX_LEN] = pows::<31, MAX_LEN>();
const fn pows<const X: usize, const N: usize>() -> [usize; N] {
	let mut a = [1; N];
	let mut i = 1;
	while i < N {
		a[i] = a[i - 1] * X % MOD;
		i += 1;
	}
	a
}

//NOTE: extend is O(log) as of now.
//Can be improved to O(1) by memoizing p.pow(c) and p.pow(c).inv()
//TODO: Hash.insert(), Hash.erase() (Zobrist Hashing? refer ps.cpp::Hash)
#[derive(Clone, PartialEq, Debug)]
pub struct HashSeq {
	a: [Mod<MOD>; 2],
	c: usize,
}
impl HashSeq {
	pub fn len(&self) -> usize {
		self.c
	}
	pub fn new0() -> Self {
		Self {
			a: [Mod::<MOD>::zero(), Mod::<MOD>::zero()],
			c: 0,
		}
	}
	pub fn new1<T>(x: T) -> Self
	where
		T: Copy,
		Mod<MOD>: From<T>,
	{
		Self {
			a: [Mod::<MOD>::from(x), Mod::<MOD>::from(x)],
			c: 1,
		}
	}
	pub fn extend_back(&self, x: HashSeq) -> Self {
		let mut ret = HashSeq::new0();
		ret.a[0] = self.a[0] * Mod::<MOD>::from(POW13[x.c]) + x.a[0];
		ret.a[1] = self.a[1] * Mod::<MOD>::from(POW31[x.c]) + x.a[1];
		ret.c = self.c + x.c;
		ret
	}
	pub fn extend_front(&self, x: HashSeq) -> Self {
		let mut ret = HashSeq::new0();
		ret.a[0] = x.a[0] * Mod::<MOD>::from(POW13[self.c]) + self.a[0];
		ret.a[1] = x.a[1] * Mod::<MOD>::from(POW31[self.c]) + self.a[1];
		ret.c = self.c + x.c;
		ret
	}
	// pub fn drop_back(&self, x: HashSeq) -> Self {
	// 	let mut ret = HashSeq::new0();
	// 	ret.a[0] = (self.a[0] - x.a[0]) / Mod::<MOD>::from(P[0]).pow(x.c);
	// 	ret.a[1] = (self.a[1] - x.a[1]) / Mod::<MOD>::from(P[1]).pow(x.c);
	// 	ret.c = self.c - x.c;
	// 	ret
	// }
	pub fn drop_front(&self, x: HashSeq) -> Self {
		let mut ret = HashSeq::new0();
		ret.a[0] = self.a[0] - x.a[0] * Mod::<MOD>::from(POW13[self.c - x.c]);
		ret.a[1] = self.a[1] - x.a[1] * Mod::<MOD>::from(POW31[self.c - x.c]);
		ret.c = self.c - x.c;
		ret
	}
}

impl Monoid for HashSeq {
	fn id() -> Self {
		Self::new0()
	}
	fn f(&self, rhs: Self) -> Self {
		self.extend_back(rhs)
	}
}
